#[cfg(target_os = "macos")]
mod corelocation;

mod asiair;
pub mod rawimage;

use asiair_crate::ASIAir;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
// use tracing::Level;
// use tracing_subscriber::fmt::time::OffsetTime;
// use time::macros::{format_description, offset};

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

// pub fn setup_logging() {
//     let fmt = if cfg!(debug_assertions) {
//         format_description!("[hour]:[minute]:[second].[subsecond digits:3]")
//     } else {
//         format_description!("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]")
//     };

//     let timer = OffsetTime::new(offset!(+8), fmt);

//     // #[cfg(all(desktop, not(debug_assertions)))]
//     let writer = {
//         use std::{fs::File, sync::Mutex};
//         let log_file =
//             File::create("/tmp/skyctl.log").expect("Failed to create the log file");
//         Mutex::new(log_file)
//     };

//     // #[cfg(any(debug_assertions, mobile))]
//     // let writer = std::io::stderr;

//     let builder = tracing_subscriber::fmt()
//         .with_max_level(Level::TRACE)
//         .with_file(true)
//         .with_line_number(true)
//         // .with_env_filter("skyctl_lib")
//         .with_target(false)
//         .with_timer(timer)
//         .with_writer(writer);

//     if cfg!(debug_assertions) {
//         builder.init();
//     } else {
//         builder.json().init();
//     }
// }

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // setup_logging();

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
            asiair::get_current_img,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
