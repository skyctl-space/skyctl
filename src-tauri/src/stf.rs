use tauri::{command, AppHandle, Emitter};
use crate::rawimage::RawImage;




#[command]
pub async fn load_and_emit_fits_png(app: AppHandle, telescope_index: u32, display_width: usize, display_height: usize) -> Result<(), String> {
    log::info!("Loading FITS file...");
    let path = "/Users/dompegam/ws/skyctl/public/Light_NGC3372_300.0s_Bin1_gain100_20230205-040711_0.0C_0030.fit";
    let fits = fitsio::FitsFile::open(path).map_err(|e| e.to_string())?;

    let mut raw_image = RawImage::from_fits(fits).map_err(|e| e.to_string())?;

    raw_image.debayer().map_err(|e| e.to_string())?;
    raw_image.downsample(display_width, display_height).map_err(|e| e.to_string())?;
   
    let base64_png = raw_image.stretch(None, None).map_err(|e| e.to_string())?;

    // Send to frontend
    let payload = serde_json::json!({
        "index": telescope_index,
        "data": format!("data:image/png;base64,{}", base64_png)
    });

    app.emit("fits_image_updated", payload)
        .map_err(|e| e.to_string())?;

    Ok(())
}