use ndarray::{Array2, Array3};
use rayon::prelude::*;

pub fn downsample(data: &Array2<f32>, target_width: usize, target_height: usize) -> Array2<f32> {
    let (h, w) = data.dim();
    let scale_x = w as f32 / target_width as f32;
    let scale_y = h as f32 / target_height as f32;

    let pixels: Vec<f32> = (0..target_height)
        .into_par_iter()
        .flat_map_iter(|y| {
            (0..target_width).map(move |x| {
                let start_x = (x as f32 * scale_x).floor() as usize;
                let start_y = (y as f32 * scale_y).floor() as usize;
                let end_x = ((x as f32 + 1.0) * scale_x).ceil() as usize;
                let end_y = ((y as f32 + 1.0) * scale_y).ceil() as usize;

                let mut sum = 0.0;
                let mut count = 0;
                for yy in start_y..end_y.min(h) {
                    for xx in start_x..end_x.min(w) {
                        sum += data[[yy, xx]];
                        count += 1;
                    }
                }

                if count > 0 { sum / count as f32 } else { 0.0 }
            })
        })
        .collect();

    Array2::from_shape_vec((target_height, target_width), pixels)
        .expect("Failed to reshape downsampled array")
}

pub fn downsample_rgb(data: &Array3<f32>, target_width: usize, target_height: usize) -> Array3<f32> {
    let (h, w, _) = data.dim();
    let scale_x = w as f32 / target_width as f32;
    let scale_y = h as f32 / target_height as f32;

    let pixels: Vec<f32> = (0..target_height)
        .into_par_iter()
        .flat_map_iter(|y| {
            (0..target_width).flat_map(move |x| {
                (0..3).map(move |c| {
                    let start_x = (x as f32 * scale_x).floor() as usize;
                    let start_y = (y as f32 * scale_y).floor() as usize;
                    let end_x = ((x as f32 + 1.0) * scale_x).ceil() as usize;
                    let end_y = ((y as f32 + 1.0) * scale_y).ceil() as usize;

                    let mut sum = 0.0;
                    let mut count = 0;
                    for yy in start_y..end_y.min(h) {
                        for xx in start_x..end_x.min(w) {
                            sum += data[[yy, xx, c]];
                            count += 1;
                        }
                    }

                    if count > 0 { sum / count as f32 } else { 0.0 }
                })
            })
        })
        .collect();

    Array3::from_shape_vec((target_height, target_width, 3), pixels)
        .expect("Failed to reshape RGB downsampled array")
}