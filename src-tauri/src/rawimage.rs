use std::vec;

use crate::{
    debayer::{debayer_image, BayerPattern},
    downsample::{downsample, downsample_rgb},
};
use fitsio::FitsFile;
use ndarray::{s, Array, Array2, Ix2, Ix3, IxDyn};
use rayon::join;
use std::simd::{f32x8};
use std::simd::num::SimdFloat;

#[derive(serde::Serialize)]
pub struct Stat{
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
    pub pixels: Vec<u16>,
    pub stats: Vec<Stat>,
}

pub struct RawImage {
    pub bayer_pattern: BayerPattern,
    pub raw_image: Array<u32, Ix2>,
    pub debayered_image: Option<Array<u32, Ix3>>,
    pub downsampled: bool,
    pub downsampled_width: usize,
    pub downsampled_height: usize,
}

fn median(data: &Array2<u32>) -> f32 {
    let len = data.len();
    if len == 0 {
        return 0.0;
    }
    let mid = len / 2;
    let mut v: Vec<u32> = data.iter().copied().collect();
    let (_, median, _) = v.select_nth_unstable(mid);
    *median as f32
}


fn calc_channel_stats(data: &Array2<u32>) -> Stat {
    let median_val = median(data);
    let len = data.len();
    let n = len as f32;
    let data_flat = data.as_slice().unwrap();
    let mut sum = 0.0f32;
    let mut min_val = f32::MAX;
    let mut max_val = f32::MIN;
    let mut sum_val = 0.0f32;
    let mut i = 0;
    let simd_width = 8; // Hardcoded for f32x8
    let median_simd = f32x8::splat(median_val);
    while i + simd_width <= len {
        let chunk_f = f32x8::from_array([
            data_flat[i] as f32,
            data_flat[i+1] as f32,
            data_flat[i+2] as f32,
            data_flat[i+3] as f32,
            data_flat[i+4] as f32,
            data_flat[i+5] as f32,
            data_flat[i+6] as f32,
            data_flat[i+7] as f32,
        ]);
        let dev = (chunk_f - median_simd).abs();
        sum += dev.reduce_sum();
        min_val = min_val.min(chunk_f.reduce_min());
        max_val = max_val.max(chunk_f.reduce_max());
        sum_val += chunk_f.reduce_sum();
        i += simd_width;
    }
    while i < len {
        let v_f = data_flat[i] as f32;
        sum += (v_f - median_val).abs();
        min_val = min_val.min(v_f);
        max_val = max_val.max(v_f);
        sum_val += v_f;
        i += 1;
    }
    let avg_dev = sum / n;
    let avg = sum_val / n;
    Stat {
        median: median_val,
        avg_dev,
        min: min_val.round(),
        max: max_val.round(),
        avg
    }
}

impl RawImage {
    pub fn from_fits(mut fits: FitsFile) -> Result<Self, String> {
        let hdu = fits.primary_hdu().map_err(|e| e.to_string())?;
        let raw_image_u32: Array<u32, Ix2> = hdu
            .read_image::<Array<u32, IxDyn>>(&mut fits)
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
            raw_image: raw_image_u32,
            bayer_pattern: bayer_pattern,
            debayered_image: None,
            downsampled: false,
            downsampled_width: 0,
            downsampled_height: 0,
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
            let mut rgb: Vec<u16> = Vec::with_capacity(width * height * 3);
            let stats = self.calculate_stats();

            for &v in self.raw_image.iter() {
                let v16 = v.min(u16::MAX as u32) as u16;
                rgb.extend_from_slice(&[v16, v16, v16]);
            }
    
            return RawRGBImage {
                width: width.try_into().unwrap(),
                height: height.try_into().unwrap(),
                pixels: rgb,
                stats: stats
            };
        } else {
            if let Some(debayered_image) = self.debayered_image.as_ref() {
                let (height, width, _) = debayered_image.dim();
                let mut rgb: Vec<u16>  = Vec::with_capacity(width * height * 3);
                let stats = self.calculate_stats();

                for pixel in debayered_image.outer_iter() {
                    for rgb_triplet in pixel.outer_iter() {
                        let r = rgb_triplet[0].min(u16::MAX as u32) as u16;
                        let g = rgb_triplet[1].min(u16::MAX as u32) as u16;
                        let b = rgb_triplet[2].min(u16::MAX as u32) as u16;
                        rgb.extend_from_slice(&[r, g, b]);
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
                let mut rgb: Vec<u16>  = Vec::with_capacity(width * height * 3);
                let stats = self.calculate_stats();
    
                for &v in self.raw_image.iter() {
                    let v16 = v.min(u16::MAX as u32) as u16;
                    rgb.extend_from_slice(&[v16, v16, v16]);
                }
        
                return RawRGBImage {
                    width: width.try_into().unwrap(),
                    height: height.try_into().unwrap(),
                    pixels: rgb,
                    stats: stats
                };
            }
        }
     }

     fn calculate_stats(&self) -> Vec<Stat> {
        let mut results = vec![];
        let start_time = std::time::Instant::now();
        if self.bayer_pattern == BayerPattern::NONE {
            results.push(calc_channel_stats(&self.raw_image));
        } else {
            if let Some(debayered_image) = &self.debayered_image {
                let (r, (g, b)) = join(
                    || calc_channel_stats(&debayered_image.slice(s![.., .., 0]).to_owned()),
                    || {
                        join(
                            || calc_channel_stats(&debayered_image.slice(s![.., .., 1]).to_owned()),
                            || calc_channel_stats(&debayered_image.slice(s![.., .., 2]).to_owned()),
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
