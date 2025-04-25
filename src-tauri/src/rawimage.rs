use std::vec;

use crate::{
    debayer::{debayer_image, BayerPattern},
    downsample::{downsample, downsample_rgb},
};
use fitsio::FitsFile;
use ndarray::{s, Array, Array2, Ix2, Ix3, IxDyn};
use rayon::join;
use crate::stretch::calc_channel_stats;

#[derive(serde::Serialize)]
pub struct stat{
    min: f32,
    max: f32,
    avg: f32,
    median: f32,
    avg_dev: f32,
}

#[derive(serde::Serialize)]
pub struct RawRGBImage {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<f32>,
    pub stats: Vec<stat>,
}

pub struct RawImage {
    pub bayer_pattern: BayerPattern,
    pub raw_image: Array<f32, Ix2>,
    pub debayered_image: Option<Array<f32, Ix3>>,
    pub downsampled: bool,
    pub downsampled_width: usize,
    pub downsampled_height: usize,
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
            raw_image,
            bayer_pattern: bayer_pattern,
            debayered_image: None,
            downsampled: false,
            downsampled_width: 0,
            downsampled_height: 0,
            // stretch: Stretch::new(0.25, -1.5),
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

    pub fn get_raw_image(&self) -> RawRGBImage {
        if self.bayer_pattern == BayerPattern::NONE {
            let (height, width) = self.raw_image.dim();
            let mut rgb: Vec<f32> = Vec::with_capacity(width * height * 3);
            let stats = self.calculate_stats();

            self.raw_image.iter().for_each(|&v| {
                rgb.extend_from_slice(&[v / stats[0].max, v / stats[1].max, v / stats[2].max]); // R, G, B
            });
    
            return RawRGBImage {
                width: width.try_into().unwrap(),
                height: height.try_into().unwrap(),
                pixels: rgb,
                stats: stats
            };
        } else {
            if let Some(debayered_image) = self.debayered_image.as_ref() {
                let (height, width, _) = debayered_image.dim();
                let mut rgb: Vec<f32>  = Vec::with_capacity(width * height * 3);
                let stats = self.calculate_stats();

                for pixel in debayered_image.outer_iter() {
                    for rgb_triplet in pixel.outer_iter() {
                        rgb.extend_from_slice(&[
                            rgb_triplet[0] / stats[0].max,
                            rgb_triplet[1] / stats[1].max,
                            rgb_triplet[2] / stats[2].max
                        ]);
                    }
                }

                return RawRGBImage {
                    width: width.try_into().unwrap(),
                    height: height.try_into().unwrap(),
                    pixels: rgb,
                    stats: stats
                };
            } else {
                let (height, width) = self.raw_image.dim();
                let mut rgb: Vec<f32>  = Vec::with_capacity(width * height * 3);
                let stats = self.calculate_stats();
    
                self.raw_image.iter().for_each(|&v| {
                    rgb.extend_from_slice(&[v / stats[0].max, v / stats[1].max, v / stats[2].max]); // R, G, B
                });
        
                return RawRGBImage {
                    width: width.try_into().unwrap(),
                    height: height.try_into().unwrap(),
                    pixels: rgb,
                    stats: stats
                };
            }
        }
     }

     fn calculate_stats(&self) -> Vec<stat> {
        let mut results = vec![];

        let calculate_chanel = |data: &Array2<f32>| -> stat {
            let (median, avg_dev, min, max, avg) = calc_channel_stats(data);

            stat { min, max, avg, median, avg_dev}
        };

        let start_time = std::time::Instant::now();
        if self.bayer_pattern == BayerPattern::NONE {
            results.push(calculate_chanel(&self.raw_image));
        } else {
            if let Some(debayered_image) = &self.debayered_image {
                let (r, (g, b)) = join(
                    || calculate_chanel(&debayered_image.slice(s![.., .., 0]).to_owned()),
                    || {
                        join(
                            || calculate_chanel(&debayered_image.slice(s![.., .., 1]).to_owned()),
                            || calculate_chanel(&debayered_image.slice(s![.., .., 2]).to_owned()),
                        )
                    },
                );
                results.push(r);
                results.push(g);
                results.push(b);
            }
        }
        let elapsed_time = start_time.elapsed();
        log::info!("Stats took: {:?}", elapsed_time);

        results
     }
 
}
