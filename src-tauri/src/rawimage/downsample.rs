use ndarray::{Array2, Array3};
use rayon::prelude::*;

pub fn downsample(data: &Array2<i32>, max_width: usize, max_height: usize) -> Array2<i32> {
    let (h, w) = data.dim();
    let aspect = w as f32 / h as f32;
    let (target_width, target_height) = if (max_width as f32 / max_height as f32) > aspect {
        let th = max_height.min(h);
        let tw = ((th as f32) * aspect).round() as usize;
        (tw, th)
    } else {
        let tw = max_width.min(w);
        let th = ((tw as f32) / aspect).round() as usize;
        (tw, th)
    };
    let scale_x = w as f32 / target_width as f32;
    let scale_y = h as f32 / target_height as f32;
    let pixels: Vec<i32> = (0..target_height)
        .into_par_iter()
        .flat_map_iter(|y| {
            (0..target_width).map(move |x| {
                let start_x = (x as f32 * scale_x).floor() as usize;
                let start_y = (y as f32 * scale_y).floor() as usize;
                let end_x = ((x as f32 + 1.0) * scale_x).ceil() as usize;
                let end_y = ((y as f32 + 1.0) * scale_y).ceil() as usize;
                let mut sum = 0i64;
                let mut count = 0i64;
                for yy in start_y..end_y.min(h) {
                    for xx in start_x..end_x.min(w) {
                        sum += data[[yy, xx]] as i64;
                        count += 1;
                    }
                }
                if count > 0 {
                    (sum / count) as i32
                } else {
                    0
                }
            })
        })
        .collect();
    Array2::from_shape_vec((target_height, target_width), pixels)
        .expect("Failed to reshape downsampled array")
}

pub fn downsample_rgb(data: &Array3<i32>, max_width: usize, max_height: usize) -> Array3<i32> {
    let (h, w, _) = data.dim();
    let aspect = w as f32 / h as f32;
    let (target_width, target_height) = if (max_width as f32 / max_height as f32) > aspect {
        let th = max_height.min(h);
        let tw = ((th as f32) * aspect).round() as usize;
        (tw, th)
    } else {
        let tw = max_width.min(w);
        let th = ((tw as f32) / aspect).round() as usize;
        (tw, th)
    };
    let scale_x = w as f32 / target_width as f32;
    let scale_y = h as f32 / target_height as f32;
    let pixels: Vec<i32> = (0..target_height)
        .into_par_iter()
        .flat_map_iter(|y| {
            (0..target_width).flat_map(move |x| {
                (0..3).map(move |c| {
                    let start_x = (x as f32 * scale_x).floor() as usize;
                    let start_y = (y as f32 * scale_y).floor() as usize;
                    let end_x = ((x as f32 + 1.0) * scale_x).ceil() as usize;
                    let end_y = ((y as f32 + 1.0) * scale_y).ceil() as usize;
                    let mut sum = 0i64;
                    let mut count = 0i64;
                    for yy in start_y..end_y.min(h) {
                        for xx in start_x..end_x.min(w) {
                            sum += data[[yy, xx, c]] as i64;
                            count += 1;
                        }
                    }
                    if count > 0 {
                        (sum / count) as i32
                    } else {
                        0
                    }
                })
            })
        })
        .collect();
    Array3::from_shape_vec((target_height, target_width, 3), pixels)
        .expect("Failed to reshape RGB downsampled array")
}
