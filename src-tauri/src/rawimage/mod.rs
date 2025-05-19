use std::vec;
mod debayer;
mod downsample;

use fitsrs::{card::Value, Fits, Pixels, HDU}; // Updated imports for fitsrs
use ndarray::{s, Array, Array2, Ix2, Ix3};
use rayon::join;
use std::io::BufReader;
use {
    debayer::debayer_image,
    downsample::{downsample, downsample_rgb},
};

#[allow(dead_code)]
#[derive(PartialEq, Copy, Clone, Debug)]
pub enum BayerPattern {
    NONE,
    RGGB,
    BGGR,
    GRBG,
    GBRG,
}

#[derive(serde::Serialize, Clone, Debug)]
pub struct Stat {
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

#[derive(Debug)]
pub struct RawImage {
    pub bayer_pattern: BayerPattern,
    pub raw_image: Array<i32, Ix2>,
    pub debayered_image: Option<Array<i32, Ix3>>,
    pub downsampled: bool,
    pub downsampled_width: usize,
    pub downsampled_height: usize,
}

fn median(data: &Array2<i32>) -> f32 {
    let len = data.len();
    if len == 0 {
        return 0.0;
    }
    let mid = len / 2;
    let mut v: Vec<i32> = data.iter().copied().collect();
    let (_, median, _) = v.select_nth_unstable(mid);
    *median as f32
}

fn calc_channel_stats(data: &Array2<i32>) -> Stat {
    let median_val = median(data);
    let len = data.len();
    let n = len as f32;
    let data_flat = data.as_slice().unwrap();
    let mut sum = 0.0f32;
    let mut min_val = f32::MAX;
    let mut max_val = f32::MIN;
    let mut sum_val = 0.0f32;

    for &v in data_flat {
        let v_f = v as f32;
        sum += (v_f - median_val).abs();
        min_val = min_val.min(v_f);
        max_val = max_val.max(v_f);
        sum_val += v_f;
    }

    let avg_dev = sum / n;
    let avg = sum_val / n;
    Stat {
        median: median_val,
        avg_dev,
        min: min_val.round(),
        max: max_val.round(),
        avg,
    }
}

impl RawImage {
    pub fn from_bytes_i16(
        raw_image: Vec<u8>,
        width: usize,
        height: usize,
        bayer_pattern: BayerPattern,
        bzero: i64,
        bscale: i64,
    ) -> Self {
        // Interpret the raw_image Vec<u8> as a Vec<i16>
        let raw_image_i16: Vec<i16> = raw_image
            .chunks_exact(2)
            .map(|b| i16::from_be_bytes([b[0], b[1]]))
            .collect();

        let raw_image_i32 = Array::from_shape_vec(
            (height, width),
            raw_image_i16
            .into_iter()
            .map(|x| ((x as i64 + bzero) * bscale) as i32)
            .collect(),
        )
        .expect("Failed to convert raw_image to 2D array");
        Self {
            raw_image: raw_image_i32,
            bayer_pattern,
            debayered_image: None,
            downsampled: false,
            downsampled_width: 0,
            downsampled_height: 0,
        }
    }


    pub fn from_reader(reader: BufReader<std::fs::File>) -> Result<Self, String> {
        let mut hdu_list = Fits::from_reader(reader);

        if let Some(Ok(HDU::Primary(hdu))) = hdu_list.next() {
            let xtension = hdu.get_header().get_xtension();
            let naxis1 = *xtension.get_naxisn(1).unwrap() as usize;
            let naxis2 = *xtension.get_naxisn(2).unwrap() as usize;

            let image = hdu_list.get_data(&hdu);

            if let Pixels::I16(data) = image.pixels() {
                let raw_image_i16 =
                    Array::from_shape_vec((naxis2, naxis1), data.collect::<Vec<_>>())
                        .map_err(|_| "Failed to convert to 2D array".to_string())?;

                let raw_image_i32;

                if let Some(Value::Integer { value, .. }) = hdu.get_header().get("BZERO") {
                    let bzero = value;
                    let mut bscale = 1i64;

                    if let Some(Value::Integer { value, .. }) = hdu.get_header().get("BSCALE") {
                        bscale = *value;
                    }

                    raw_image_i32 = raw_image_i16.mapv(|x| ((x as i64 + bzero) * bscale) as i32);
                } else {
                    raw_image_i32 = raw_image_i16.mapv(|x| x as i32);
                }

                let bayer_pattern = match hdu.get_header().get("BAYERPAT") {
                    Some(Value::String { value, .. }) => match value.as_str() {
                        "RGGB" => BayerPattern::RGGB,
                        "BGGR" => BayerPattern::BGGR,
                        "GRBG" => BayerPattern::GRBG,
                        "GBRG" => BayerPattern::GBRG,
                        _ => BayerPattern::NONE,
                    },
                    _ => BayerPattern::NONE,
                };

                return Ok(Self {
                    raw_image: raw_image_i32,
                    bayer_pattern,
                    debayered_image: None,
                    downsampled: false,
                    downsampled_width: 0,
                    downsampled_height: 0,
                });
            } else {
                return Err("Expected I16 pixel data".to_string());
            }
        }

        Err("No primary HDU found".to_string())
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
                let v16 = v.min(u16::MAX as i32) as u16;
                rgb.extend_from_slice(&[v16, v16, v16]);
            }

            return RawRGBImage {
                width: width.try_into().unwrap(),
                height: height.try_into().unwrap(),
                pixels: rgb,
                stats: stats,
            };
        } else {
            if let Some(debayered_image) = self.debayered_image.as_ref() {
                let (height, width, _) = debayered_image.dim();
                let mut rgb: Vec<u16> = Vec::with_capacity(width * height * 3);
                let stats = self.calculate_stats();

                for pixel in debayered_image.outer_iter() {
                    for rgb_triplet in pixel.outer_iter() {
                        let r = rgb_triplet[0].min(u16::MAX as i32) as u16;
                        let g = rgb_triplet[1].min(u16::MAX as i32) as u16;
                        let b = rgb_triplet[2].min(u16::MAX as i32) as u16;
                        rgb.extend_from_slice(&[r, g, b]);
                    }
                }

                return RawRGBImage {
                    width: width.try_into().unwrap(),
                    height: height.try_into().unwrap(),
                    pixels: rgb,
                    stats: stats,
                };
            } else {
                let (height, width) = self.raw_image.dim();
                let mut rgb: Vec<u16> = Vec::with_capacity(width * height * 3);
                let stats = self.calculate_stats();

                for &v in self.raw_image.iter() {
                    let v16 = v.min(u16::MAX as i32) as u16;
                    rgb.extend_from_slice(&[v16, v16, v16]);
                }

                return RawRGBImage {
                    width: width.try_into().unwrap(),
                    height: height.try_into().unwrap(),
                    pixels: rgb,
                    stats: stats,
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
