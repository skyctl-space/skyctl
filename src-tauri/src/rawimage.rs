use crate::{
    debayer::{debayer_image, BayerPattern},
    downsample::{downsample, downsample_rgb},
    stretch::Stretch,
};
use fitsio::FitsFile;
use image::{ImageBuffer, ImageFormat, Luma};
use ndarray::{s, Array, Array3, Ix2, Ix3, IxDyn};
use rayon::join;
use std::io::Cursor;
use base64::Engine;

pub struct RawImage {
    #[allow(dead_code)]
    pub fits: FitsFile,
    pub bayer_pattern: BayerPattern,
    raw_image: Array<f32, Ix2>,
    debayered_image: Option<Array<f32, Ix3>>,
    pub downsampled: bool,
    pub downsampled_width: usize,
    pub downsampled_height: usize,
    stretch: Stretch,
}

impl RawImage {
    pub fn from_fits(mut fits: FitsFile) -> Result<Self, String> {
        let hdu = fits.primary_hdu().map_err(|e| e.to_string())?;
        let raw_image: Array<f32, Ix2> = hdu
            .read_image::<Array<f32, IxDyn>>(&mut fits)
            .map_err(|e| e.to_string())?
            .into_dimensionality::<Ix2>()
            .map_err(|_| "Failed to convert to 2D array".to_string())?;


        let bayer_pattern = 
            match hdu.read_key::<String>(&mut fits, "BAYERPAT").unwrap_or_else(|_| "".to_string()).to_uppercase().as_str() {
            "RGGB" => BayerPattern::RGGB,
            "BGGR" => BayerPattern::BGGR,
            "GRBG" => BayerPattern::GRBG,
            "GBRG" => BayerPattern::GBRG,
            _ => BayerPattern::NONE,
        };

        Ok(Self {
            fits,
            raw_image,
            bayer_pattern: bayer_pattern,
            debayered_image: None,
            downsampled: false,
            downsampled_width: 0,
            downsampled_height: 0,
            stretch: Stretch::new(0.25, -1.25),
        })
    }

    pub fn debayer(&mut self) -> Result<(), String> {
        if self.bayer_pattern == BayerPattern::NONE {
            return Ok(());
        }

        if self.debayered_image.is_some() {
            return Ok(());
        }

        let start_time = std::time::Instant::now();
        let debayered = debayer_image(&self.raw_image, self.bayer_pattern);
        let elapsed_time = start_time.elapsed();
        log::info!("Debayering took: {:?}", elapsed_time);

        self.debayered_image = Some(debayered);
        Ok(())
    }

    pub fn downsample(&mut self, target_width: usize, target_height: usize) -> Result<(), String> {
        if self.downsampled
            && self.downsampled_width == target_width
            && self.downsampled_height == target_height
        {
            return Ok(());
        }

        let start_time = std::time::Instant::now();
        if let Some(debayered_image) = &self.debayered_image {
            let downsampled = downsample_rgb(debayered_image, target_width, target_height);
            self.debayered_image = Some(downsampled);
            self.downsampled = true;
            self.downsampled_width = target_width;
            self.downsampled_height = target_height;
        } else {
            let downsampled = downsample(&self.raw_image, target_width, target_height);
            self.raw_image = downsampled;
            self.downsampled = true;
            self.downsampled_width = target_width;
            self.downsampled_height = target_height;
        }
        let elapsed_time = start_time.elapsed();
        log::info!("Downsampling took: {:?}", elapsed_time);

        Ok(())
    }

    pub fn stretch(&mut self, min: Option<f32>, max: Option<f32>) -> Result<String, String> {
        let mut stretch = self.stretch;

        if let Some(min) = min {
            stretch.target_bkg = min;
        }
        if let Some(max) = max {
            stretch.shadows_clip = max;
        }

        let mut png_bytes = Vec::new();
        let mut cursor = Cursor::new(&mut png_bytes);

        let mut start_time = std::time::Instant::now();
        if self.bayer_pattern == BayerPattern::NONE {
            let stretched_gray = stretch.stretch(&self.raw_image);
            let elapsed_time = start_time.elapsed();
            start_time = std::time::Instant::now();
            log::info!("Stretching took: {:?}", elapsed_time);

            let gray_img = ImageBuffer::<Luma<u8>, _>::from_fn(
                stretched_gray.ncols() as u32,
                stretched_gray.nrows() as u32,
                |x, y| {
                    let v =
                        (stretched_gray[[y as usize, x as usize]] * 255.0).clamp(0.0, 255.0) as u8;
                    Luma([v])
                },
            );

            gray_img
                .write_to(&mut cursor, ImageFormat::Png)
                .expect("Failed to encode image to PNG");
        } else {
            if let Some(debayered_image) = &self.debayered_image {
                let (r, (g, b)) = join(
                    || stretch.stretch(&debayered_image.slice(s![.., .., 0]).to_owned()),
                    || {
                        join(
                            || stretch.stretch(&debayered_image.slice(s![.., .., 1]).to_owned()),
                            || stretch.stretch(&debayered_image.slice(s![.., .., 2]).to_owned()),
                        )
                    },
                );

                let (h, w) = (r.nrows(), r.ncols());
                let stretched_rgb = Array3::from_shape_fn((h, w, 3), |(y, x, c)| match c {
                    0 => r[[y, x]],
                    1 => g[[y, x]],
                    2 => b[[y, x]],
                    _ => 0.0,
                });
                let elapsed_time = start_time.elapsed();
                start_time = std::time::Instant::now();
                log::info!("Stretching took: {:?}", elapsed_time);

                // Convert to Vec<u8> for image encoding
                let rgb_bytes: Vec<u8> = stretched_rgb.iter().map(|v| (v * 255.0).clamp(0.0, 255.0) as u8).collect();
                let rgb_img = ImageBuffer::<image::Rgb<u8>, _>::from_raw(w as u32, h as u32, rgb_bytes)
                .expect("Failed to create RgbImage");

            rgb_img
                .write_to(&mut cursor, ImageFormat::Png)
                .expect("Failed to encode image to PNG");
            } else {
                return Err("Debayered image is not available".to_string());
            }
        }
        let elapsed_time = start_time.elapsed();
        start_time = std::time::Instant::now();
        log::info!("Encoding png took: {:?}", elapsed_time);

        // Convert PNG bytes to base64
        let base64_png = base64::engine::general_purpose::STANDARD.encode(&png_bytes);

        let elapsed_time = start_time.elapsed();
        log::info!("Encoding base64 took: {:?}", elapsed_time);

        Ok(base64_png)
    }
}
