use tauri::{ipc::Channel};

use super::ASIAirState;
use asiair_crate::camera::{ConnectedCamera, CameraState};
use serde::Serialize;

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
pub async fn get_connected_cameras(
    state: State<'_, ASIAirState>,
    guid: String,
) -> Result<Vec<ConnectedCamera>, String> {
    let mut asiair = {
        let asiairs = state.asiairs.lock().unwrap();
        if let Some(asiair) = asiairs.get(&guid) {
            asiair.clone()
        } else {
            return Err("That ASIAir is not currently connected".to_string());
        }
    };

    let cameras = asiair
        .get_connected_cameras()
        .await
        .map_err(|e| format!("Failed to get connected cameras: {:?}", e))?;

    Ok(cameras)
}

#[tauri::command]
pub async fn main_camera_get_state(
    state: State<'_, ASIAirState>,
    guid: String,
) -> Result<CameraState, String> {
    let mut asiair = {
        let asiairs = state.asiairs.lock().unwrap();
        if let Some(asiair) = asiairs.get(&guid) {
            asiair.clone()
        } else {
            return Err("That ASIAir is not currently connected".to_string());
        }
    };

    let camera_state = asiair
        .main_camera_get_state()
        .await
        .map_err(|e| format!("Failed to get camera state: {:?}", e))?;

    Ok(camera_state)
}

#[tauri::command]
pub async fn main_camera_open(
    state: State<'_, ASIAirState>,
    guid: String,
) -> Result<(), String> {
    let mut asiair = {
        let asiairs = state.asiairs.lock().unwrap();
        if let Some(asiair) = asiairs.get(&guid) {
            asiair.clone()
        } else {
            return Err("That ASIAir is not currently connected".to_string());
        }
    };

    asiair
        .main_camera_open()
        .await
        .map_err(|e| format!("Failed to open camera: {:?}", e))?;

    Ok(())
}

#[tauri::command]
pub async fn main_camera_close(
    state: State<'_, ASIAirState>,
    guid: String,
) -> Result<(), String> {
    let mut asiair = {
        let asiairs = state.asiairs.lock().unwrap();
        if let Some(asiair) = asiairs.get(&guid) {
            asiair.clone()
        } else {
            return Err("That ASIAir is not currently connected".to_string());
        }
    };

    asiair
        .main_camera_close()
        .await
        .map_err(|e| format!("Failed to close camera: {:?}", e))?;

    Ok(())
}

#[tauri::command]
pub async fn main_camera_set_name(
    state: State<'_, ASIAirState>,
    guid: String,
    camera_name: String,
) -> Result<(), String> {
    let mut asiair = {
        let asiairs = state.asiairs.lock().unwrap();
        if let Some(asiair) = asiairs.get(&guid) {
            asiair.clone()
        } else {
            return Err("That ASIAir is not currently connected".to_string());
        }
    };

    asiair
        .main_camera_set_name(camera_name)
        .await
        .map_err(|e| format!("Failed to set main camera: {:?}", e))?;

    Ok(())
}

#[tauri::command]
pub async fn main_camera_get_name(
    state: State<'_, ASIAirState>,
    guid: String,
) -> Result<String, String> {
    let mut asiair = {
        let asiairs = state.asiairs.lock().unwrap();
        if let Some(asiair) = asiairs.get(&guid) {
            asiair.clone()
        } else {
            return Err("That ASIAir is not currently connected".to_string());
        }
    };

    let camera_name = asiair
        .main_camera_get_name()
        .await
        .map_err(|e| format!("Failed to get main camera name: {:?}", e))?;

    Ok(camera_name)
}

#[tauri::command]
pub async fn main_camera_get_current_img(
    state: State<'_, ASIAirState>,
    guid: String,
    sender: Channel<ImageProgress>,
    binary_sender: Channel<&[u8]>
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
        .main_camera_get_current_img()
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

    binary_sender.send(&image_bytes).unwrap_or_else(|e| {
        log::error!("Failed to send image bytes: {:?}", e);
    });

    Ok(())
}