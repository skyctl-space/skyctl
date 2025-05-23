pub mod discovery;
pub mod camera_cmds;
use tauri::{AppHandle, Emitter};

use super::ASIAirState;
use asiair_crate::ASIAir;
use serde::Serialize;

use std::net::Ipv4Addr;
use tauri::State;

#[derive(Clone, Serialize, Debug)]
struct ConnectionChange {
    guid: String,
    connected: bool,
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

    let mut camera_state_rx = asiair.subscribe_camera_state_change();
    let app_clone = app.clone();
    let should_be_connected = asiair.should_be_connected.clone();
    let guid_clone = guid.clone();

    tokio::spawn(async move {
        while camera_state_rx.changed().await.is_ok()
            && should_be_connected.load(std::sync::atomic::Ordering::SeqCst){
            app_clone.emit("asiair_camera_state_change", guid_clone.clone()).expect("Failed to emit device list");
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
