pub mod discovery;
use tauri::{AppHandle, Emitter, ipc::Channel};

use super::ASIAirState;
use asiair_crate::ASIAir;
use serde::Serialize;

use std::net::Ipv4Addr;
use tauri::State;
use crate::rawimage::{RawImage, RawRGBImage, Stat, BayerPattern};

#[derive(Clone, Serialize, Debug)]
struct ConnectionChange {
    guid: String,
    connected: bool,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase", tag = "event", content = "data")]
pub enum ImageProgress {
    Fetching,
    Downsampling,
    Debayering,
    Rendering {
        width: u32, 
        height: u32, 
        stats: Vec<Stat>,
    },
    Error(String),
}

#[tauri::command]
pub async fn asiair_attach(
    app: AppHandle,
    state: State<'_, ASIAirState>,
    guid: String,
    connection: String,
) -> Result<(), String> {
    let addr: Ipv4Addr;

    log::debug!("Connecting to ASIAir with guid: {}", guid);
    log::debug!("Connection type: {}", connection);

    if connection == "auto" {
        // Use the discovered IP address
        let discovered = state.discovered.lock().unwrap();
        if let Some(device) = discovered.get(&guid) {
            addr = device
                .value
                .parse()
                .map_err(|_| format!("Invalid IP address: {:?}", device.value))?;
        } else {
            return Err("That ASIAir is not currently detectable on the network".to_string());
        }
    } else {
        // Use the provided IP address
        addr = connection
            .parse()
            .map_err(|_| format!("Invalid IP address: {:?}", connection))?;
    }

    let mut asiair;

    // Check if the ASIAir instance already exists
    {
        let mut asiairs = state.asiairs.lock().unwrap();
        if asiairs.contains_key(&guid) {
            asiair = asiairs.get_mut(&guid).unwrap().clone();
            // Update the address if it has changed
            asiair.addr = addr;
        } else {
            asiair = ASIAir::new(addr);
        }
    }

    log::debug!("Connecting to ASIAir at {}", addr);
    asiair
        .connect()
        .await
        .map_err(|e| format!("Failed to connect to ASIAir: {:?}", e))?;
    log::debug!("Connected to ASIAir at {}", addr);

    let mut conn_rx = asiair.subscribe_connection_state();
    let app_clone = app.clone();
    let guid_clone = guid.clone();
    let should_be_connected = asiair.should_be_connected.clone();

    tokio::spawn(async move {
        // The the thread die if the asiair is disconnected since, otherwise we will leak workers
        while conn_rx.changed().await.is_ok()
            && should_be_connected.load(std::sync::atomic::Ordering::SeqCst)
        {
            let connected = *conn_rx.borrow();

            app_clone
                .emit(
                    "asiair_connection_state",
                    ConnectionChange {
                        guid: guid_clone.clone(),
                        connected,
                    },
                )
                .expect("Failed to emit device list");
        }
    });

    let mut asiairs = state.asiairs.lock().unwrap();
    asiairs.insert(guid.clone(), asiair);

    Ok(())
}

#[tauri::command]
pub async fn asiair_deattach(
    _app: AppHandle,
    state: State<'_, ASIAirState>,
    guid: String,
) -> Result<(), String> {
    let mut removed_asiair;

    {
        // Lock the mutex to get mutable access:
        let mut asiairs = state.asiairs.lock().unwrap();

        // Remove the ASIAir instance from the map
        if let Some(asiair) = asiairs.remove(&guid) {
            log::debug!("Found ASIAir with guid: {}", guid);
            removed_asiair = asiair.clone();
        } else {
            return Err("That ASIAir is not currently connected".to_string());
        }
    }

    log::debug!("Disconnecting ASIAir with guid: {}", guid);
    // Disconnect the ASIAir instance
    removed_asiair.disconnect().await;

    Ok(())
}

#[tauri::command]
pub async fn get_current_img(
    state: State<'_, ASIAirState>,
    guid: String,
    sender: Channel<ImageProgress>,
    binarySender: Channel<&[u8]>
) -> Result<(), String> {
    let mut asiair = {
        let asiairs = state.asiairs.lock().unwrap();
        if let Some(asiair) = asiairs.get(&guid) {
            asiair.clone()
        } else {
            sender.send(ImageProgress::Error("That ASIAir is not currently connected".to_string())).unwrap();
            return Err("That ASIAir is not currently connected".to_string());
        }
    };

    sender.send(ImageProgress::Fetching).unwrap_or_else(|e| {
        log::error!("Failed to send 'Fetching': {:?}", e);
    });
    let (img, width, height) = asiair
        .get_current_img()
        .await
        .map_err(|e| {
            sender.send(ImageProgress::Error(format!("Failed to get current image: {:?}", e))).unwrap();
            format!("Failed to get current image: {:?}", e)
        })?;

    let mut raw_image = RawImage::from_bytes_i16(img, width as usize, height as usize, BayerPattern::RGGB, 32768, 1);

    sender.send(ImageProgress::Debayering).unwrap();
    raw_image.debayer().map_err(|e| {
        sender.send(ImageProgress::Error(e.to_string())).unwrap();
        e.to_string()
    })?;

    //sender.send(ImageProgress::Downsampling).unwrap();
    // raw_image.downsample(display_width, display_height).map_err(|e| e.to_string())?;

    let image_data : RawRGBImage = raw_image.get_raw_image();

    sender.send(ImageProgress::Rendering {
        width: image_data.width,
        height: image_data.height,
        stats: image_data.stats.clone(),
    }).unwrap();

    assert!(image_data.width * image_data.height * 3 == image_data.pixels.len() as u32);

    use bytemuck::cast_slice;
    let image_bytes: Vec<u8> = cast_slice(image_data.pixels.as_slice()).to_vec();

//    let image_bytes = [0 as u8; 2048];
    binarySender.send(&image_bytes).unwrap_or_else(|e| {
        log::error!("Failed to send image bytes: {:?}", e);
    });

    Ok(())
}