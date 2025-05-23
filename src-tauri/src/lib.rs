#[cfg(target_os = "macos")]
mod corelocation;

mod asiair;
pub mod rawimage;

use asiair_crate::ASIAir;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, serde::Serialize)]
struct DiscoveredASIAir {
    title: String,
    value: String,
    guid: String,
}

struct ASIAirState {
    discovered: Arc<Mutex<HashMap<String, DiscoveredASIAir>>>,
    // Mapped by GUID
    asiairs: Arc<Mutex<HashMap<String, ASIAir>>>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(ASIAirState {
            discovered: Arc::new(Mutex::new(HashMap::new())),
            asiairs: Arc::new(Mutex::new(HashMap::new())),
        })
        .setup(|app| {
            #[cfg(target_os = "macos")]
            {
                log::info!("Starting CoreLocation...");
                // Must run on main thread!
                let main = objc2_foundation::MainThreadMarker::new().unwrap();
                corelocation::start_location_manager(app.handle().clone(), main);
            }

            // Setup ASIAir discovery thread
            asiair::discovery::start_asiair_discovery(app.handle().clone());

            Ok(())
        })
        .plugin(
            tauri_plugin_log::Builder::new()
                .target(tauri_plugin_log::Target::new(
                    tauri_plugin_log::TargetKind::Webview,
                ))
                .build(),
        )
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .invoke_handler(tauri::generate_handler![
            asiair::asiair_attach,
            asiair::asiair_deattach,
            asiair::camera_cmds::get_connected_cameras,
            asiair::camera_cmds::main_camera_open,
            asiair::camera_cmds::main_camera_close,
            asiair::camera_cmds::main_camera_get_name,
            asiair::camera_cmds::main_camera_set_name,
            asiair::camera_cmds::main_camera_get_state,
            asiair::camera_cmds::main_camera_get_current_img,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
