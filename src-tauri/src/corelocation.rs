use objc2::rc::Retained;
use objc2::runtime::{AnyObject, NSObject, ProtocolObject};
use objc2::{define_class, msg_send, AllocAnyThread};
use objc2_core_location::{
    CLAuthorizationStatus, CLLocation, CLLocationManager, CLLocationManagerDelegate,
};
use objc2_foundation::{MainThreadMarker, NSArray, NSObjectProtocol};
use once_cell::sync::Lazy;
use tauri::AppHandle;
use tauri::Emitter;
use std::thread;
use std::time::Duration;
use std::sync::RwLock;

#[derive(Debug, Copy, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LastLocation {
    pub latitude: f64,
    pub longitude: f64,
}

pub static LAST_LOCATION: Lazy<RwLock<Option<LastLocation>>> =
    Lazy::new(|| RwLock::new(None));

// Define the MyLocationDelegate class
define_class!(
    #[name = "MyLocationDelegate"]
    #[unsafe(super = NSObject)]
    #[thread_kind = AllocAnyThread]
    struct MyLocationDelegate;

    unsafe impl CLLocationManagerDelegate for MyLocationDelegate {
        #[unsafe(method(locationManager:didChangeAuthorizationStatus:))]
        fn did_change_authorization(
            &self,
            _manager: &CLLocationManager,
            status: CLAuthorizationStatus,
        ) {
            log::info!("Authorization status changed: {:?}", status);
        }

        #[unsafe(method(locationManager:didUpdateLocations:))]
        fn did_update_locations(
            &self,
            _manager: &CLLocationManager,
            locations: &NSArray<CLLocation>,
        ) {
            if let Some(loc) = locations.firstObject() {
                let coord = unsafe { loc.coordinate() };
                log::info!("Updated location: {}, {}", coord.latitude, coord.longitude);
                let mut lock = LAST_LOCATION.write().unwrap();
  
                *lock = Some(LastLocation {
                    latitude: coord.latitude,
                    longitude: coord.longitude,
                });
            }
        }

        #[unsafe(method(locationManager:didFailWithError:))]
        fn did_fail_with_error(&self, _manager: &CLLocationManager, _error: &AnyObject) {
            log::error!("Location manager failed with error");
        }
    }

    unsafe impl NSObjectProtocol for MyLocationDelegate {}
);

impl MyLocationDelegate {
    fn new() -> Retained<Self> {
        let this = Self::alloc();
        unsafe { msg_send![this, init] }
    }
}

// Global delegate holder
static LOCATION_DELEGATE: Lazy<Retained<MyLocationDelegate>> =
    Lazy::new(|| MyLocationDelegate::new());

// Main-thread-only CLLocationManager holder
static mut LOCATION_MANAGER: Option<Retained<CLLocationManager>> = None;

// Startup function to start location manager
pub fn start_location_manager(app: AppHandle, _mtm: MainThreadMarker) {
    let manager = unsafe { CLLocationManager::new() };

    log::info!("Location manager started");

    unsafe {
        let delegate: &ProtocolObject<dyn CLLocationManagerDelegate> =
            ProtocolObject::from_ref(&**LOCATION_DELEGATE);
        manager.setDelegate(Some(delegate));
        manager.requestWhenInUseAuthorization();
        manager.startUpdatingLocation();

        LOCATION_MANAGER = Some(manager.clone());

        log::info!("Location manager updating location");

        // Start background thread to emit location every 5 seconds
        thread::spawn({
            let app = app.clone();
            move || loop {
                if let Some(loc) = *LAST_LOCATION.read().unwrap() {
                    let _ = app.emit("location_update", loc);
                }
                thread::sleep(Duration::from_secs(1));
            }
        });
    }
}
