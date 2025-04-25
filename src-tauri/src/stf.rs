use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use tauri::{command, AppHandle, Emitter};
use crate::rawimage::{RawImage, RawRGBImage};
use once_cell::sync::Lazy;

// Global hash table for RawImage objects
type RawImageMap = Arc<RwLock<HashMap<u32, Box<RawImage>>>>;
static RAW_IMAGE_TABLE: Lazy<RawImageMap> = Lazy::new(|| Arc::new(RwLock::new(HashMap::new())));

#[command]
pub async fn load_fits_image(
    app: AppHandle,
    telescope_index: u32,
    display_width: usize,
    display_height: usize,
) -> Result<(), String> {
    log::info!("Loading FITS file for telescope index {}...", telescope_index);

    // Load the FITS file and create a new RawImage
    let path = "/Users/dompegam/ws/skyctl/public/Light_NGC3372_300.0s_Bin1_gain100_20230205-040711_0.0C_0030.fit";
    let fits = fitsio::FitsFile::open(path).map_err(|e| e.to_string())?;
    let mut raw_image = RawImage::from_fits(fits).map_err(|e| e.to_string())?;

    raw_image.debayer().map_err(|e| e.to_string())?;
    raw_image.downsample(display_width, display_height).map_err(|e| e.to_string())?;

    // Override the existing RawImage in the hash table
    {
        let mut raw_image_map = RAW_IMAGE_TABLE.write().map_err(|e| e.to_string())?;
        raw_image_map.insert(telescope_index, Box::new(raw_image));
    }

    // Retrieve the raw arrays and determine if the image is mono or OSC
    let raw_image_map = RAW_IMAGE_TABLE.read().map_err(|e| e.to_string())?;
    let raw_image = raw_image_map
        .get(&telescope_index)
        .ok_or_else(|| format!("No RawImage found in cache for telescope index {}", telescope_index))?;

    let image_data : RawRGBImage = raw_image.get_raw_image();

    //assert!(image_data.width * image_data.height * 3 == image_data.pixels.len() as u32);

    // Send the raw arrays to the frontend
    let payload = serde_json::json!({
        "index": telescope_index,
        "image_data": image_data,
    });

    app.emit("fits_image_updated", payload)
        .map_err(|e| e.to_string())?;

    Ok(())
}