use tauri::{ipc::Channel};

use super::ASIAirState;
use asiair_crate::camera::{ConnectedCamera, CameraState, CameraInfo};
use serde::Serialize;

use tauri::State;
use crate::rawimage::{RawImage, RawRGBImage, Stat, BayerPattern};

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase", tag = "event", content = "data")]
pub enum ImageProgress {
    Fetching,
    #[allow(dead_code)]
    Downsampling,
    Debayering,
    Rendering {
        width: u32, 
        height: u32, 
        stats: Vec<Stat>,
    },
    Error(String),
}

/// Macro to generate tauri command functions that call an ASIAir method with no parameters and return Result<T, String>
macro_rules! asiair_simple_getter_cmd {
    (
        $(#[$outer:meta])* // Allow doc comments
        $vis:vis fn $fn_name:ident(
            $state:ident : State<'_, ASIAirState>,
            $guid:ident : String
        ) -> $ret:ty {
            $method:ident
        }
    ) => {
        $(#[$outer])*
        #[tauri::command]
        $vis async fn $fn_name(
            $state: State<'_, ASIAirState>,
            $guid: String,
        ) -> Result<$ret, String> {
            #[allow(unused_mut)]
            let mut asiair = {
                let asiairs = $state.asiairs.lock().unwrap();
                if let Some(asiair) = asiairs.get(&$guid) {
                    asiair.clone()
                } else {
                    return Err("That ASIAir is not currently connected".to_string());
                }
            };
            asiair
                .$method()
                .await
                .map_err(|e| format!("Failed to call {}: {:?}", stringify!($method), e))
        }
    };
}

/// Macro to generate tauri command functions that call an ASIAir method with parameters and return Result<(), String>
macro_rules! asiair_simple_setter_cmd {
    (
        $(#[$outer:meta])* // Allow doc comments
        $vis:vis fn $fn_name:ident(
            $state:ident : State<'_, ASIAirState>,
            $guid:ident : String,
            $( $param:ident : $ptype:ty ),* $(,)?
        ) {
            $method:ident
        }
    ) => {
        $(#[$outer])*
        #[tauri::command]
        $vis async fn $fn_name(
            $state: State<'_, ASIAirState>,
            $guid: String,
            $( $param : $ptype ),*
        ) -> Result<(), String> {
            let mut asiair = {
                let asiairs = $state.asiairs.lock().unwrap();
                if let Some(asiair) = asiairs.get(&$guid) {
                    asiair.clone()
                } else {
                    return Err("That ASIAir is not currently connected".to_string());
                }
            };
            asiair
                .$method($( $param ),*)
                .await
                .map_err(|e| format!("Failed to call {}: {:?}", stringify!($method), e))?;
            Ok(())
        }
    };
}

asiair_simple_getter_cmd! {
    /// Get the main camera name
    pub fn main_camera_get_name(
        state: State<'_, ASIAirState>,
        guid: String
    ) -> String {
        main_camera_get_name
    }
}

asiair_simple_getter_cmd! {
    /// Get all connected cameras
    pub fn main_camera_get_info(
        state: State<'_, ASIAirState>,
        guid: String
    ) -> CameraInfo {
        main_camera_get_info
    }
}

asiair_simple_getter_cmd! {
    /// Get the guide camera name
    pub fn guide_camera_get_name(
        state: State<'_, ASIAirState>,
        guid: String
    ) -> String {
        guide_camera_get_name
    }
}

asiair_simple_getter_cmd! {
    /// Get all connected cameras
    pub fn get_connected_cameras(
        state: State<'_, ASIAirState>,
        guid: String
    ) -> Vec<ConnectedCamera> {
        get_connected_cameras
    }
}

asiair_simple_getter_cmd! {
    /// Get the main camera state
    pub fn main_camera_get_state(
        state: State<'_, ASIAirState>,
        guid: String
    ) -> CameraState {
        main_camera_get_state
    }
}

asiair_simple_getter_cmd! {
    /// Open the main camera
    pub fn main_camera_open(
        state: State<'_, ASIAirState>,
        guid: String
    ) -> () {
        main_camera_open
    }
}

asiair_simple_getter_cmd! {
    /// Close the main camera
    pub fn main_camera_close(
        state: State<'_, ASIAirState>,
        guid: String
    ) -> () {
        main_camera_close
    }
}

asiair_simple_setter_cmd! {
    /// Set the main camera name
    pub fn main_camera_set_name(
        state: State<'_, ASIAirState>,
        guid: String,
        name: String
    ) {
        main_camera_set_name
    }
}

asiair_simple_setter_cmd! {
    /// Set the guide camera name
    pub fn guide_camera_set_name(
        state: State<'_, ASIAirState>,
        guid: String,
        name: String
    ) {
        guide_camera_set_name
    }
}


asiair_simple_setter_cmd! {
    pub fn main_camera_set_exposure(
        state: State<'_, ASIAirState>,
        guid: String,
        exposure: u64
    ) {
        main_camera_set_exposure
    }
}

asiair_simple_getter_cmd! {
    pub fn main_camera_get_exposure(
        state: State<'_, ASIAirState>,
        guid: String
    ) -> u64 {
        main_camera_get_exposure
    }
}

asiair_simple_setter_cmd! {
    pub fn main_camera_start_exposure(
        state: State<'_, ASIAirState>,
        guid: String,
    ) {
        main_camera_start_exposure
    }
}

#[tauri::command]
pub async fn main_camera_get_current_img(
    state: State<'_, ASIAirState>,
    guid: String,
    bayer_pattern: BayerPattern,
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

    let mut raw_image = RawImage::from_bytes_i16(img, width as usize, height as usize, bayer_pattern, 32768, 1);

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