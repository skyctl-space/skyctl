#[cfg(target_os = "macos")]
mod corelocation;

mod asiairdiscovery;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(target_os = "macos")]
            {
                log::info!("Starting CoreLocation...");
                // Must run on main thread!
                let main = objc2_foundation::MainThreadMarker::new().unwrap();
                corelocation::start_location_manager(app.handle().clone(), main);
            }
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
            asiairdiscovery::start_asiair_discovery,
            asiairdiscovery::stop_asiair_discovery,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
