use std::{
    collections::HashMap,
    net::UdpSocket,
    sync::{atomic::{AtomicBool, Ordering}, Arc, Mutex},
    thread,
    time::Duration,
};
use tauri::{AppHandle, Emitter};
use serde_json::{json, Value};

static DISCOVERY_RUNNING: AtomicBool = AtomicBool::new(false);

#[derive(Debug, Clone, serde::Serialize)]
struct Device{
    title: String,
    value: String,
}

#[tauri::command]
pub fn start_asiair_discovery(app: AppHandle) {
    if DISCOVERY_RUNNING.swap(true, Ordering::SeqCst) {
        return; // Already running
    }

    let socket = UdpSocket::bind("0.0.0.0:0").expect("Failed to bind UDP socket");
    socket
        .set_broadcast(true)
        .expect("Failed to enable broadcast");

    let mut discovery_id = 1; // Start with an initial ID

    let app_handle = app.clone();
    let devices_cache = Arc::new(Mutex::new(HashMap::<String, Device>::new())); // Cache for devices

    log::info!("Starting Asiair discovery...");
    let _ = thread::Builder::new().name("asiair_discovery".to_string()).spawn(move || {
        loop {
            if DISCOVERY_RUNNING.load(Ordering::SeqCst) {
                let discovery_message = json!({
                    "id": discovery_id.to_string(),
                    "method": "scan_air",
                    "name": "iphone"
                })
                .to_string() + "\r\n";

                discovery_id += 1; // Increment the ID for the next message

                let _ = socket.send_to(discovery_message.as_bytes(), "255.255.255.255:4720");

                let mut buf = [0u8; 4096];
                socket
                    .set_read_timeout(Some(Duration::from_millis(2000)))
                    .expect("Failed to set timeout");

                if let Ok((size, _src)) = socket.recv_from(&mut buf) {
                    let response = String::from_utf8_lossy(&buf[..size]);
                    if let Ok(json_response) = serde_json::from_str::<Value>(&response) {
                        if let Some(result) = json_response.get("result") {
                            if let Some(guid) = result.get("guid").and_then(|v| v.as_str()) {
                                if let Some(ip) = result.get("ip").and_then(|v| v.as_str()) {
                                    if let Some(ssid) = result.get("ssid").and_then(|v| v.as_str()) {
                                        let device = Device {
                                            title: ssid.to_string(),
                                            value: ip.to_string(),
                                        };

                                        // Assuming each device has a unique 'guid'
                                        // Store in cache to avoid duplicates
                                        let mut cache = devices_cache.lock().unwrap();
                                        cache.insert(guid.to_string(), device);
                                    }
                                }
                            }
                        }
                    }
                }

                let cache = devices_cache.lock().unwrap();
                let consolidated_list: Vec<Device> = cache.values().cloned().collect();
                app_handle
                    .emit("discovered_device", consolidated_list)
                    .expect("Failed to emit device list");
            } else {
                break; // Exit the loop when discovery is stopped
            }
        }
    });
}

#[tauri::command]
pub fn stop_asiair_discovery() {

    log::info!("Stoping Asiair discovery...");
    DISCOVERY_RUNNING.store(false, Ordering::SeqCst);
}