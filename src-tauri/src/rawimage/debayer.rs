use ndarray::{Array2, Array3};
use rayon::prelude::*;
use super::BayerPattern;

fn debayer_rggb(data: &Array2<i32>) -> Array3<i32> {
    let (h, w) = data.dim();
    let h2 = h / 2;
    let w2 = w / 2;
    let mut rgb = Array3::<i32>::zeros((h2, w2, 3));
    let data_ptr = data.as_slice_memory_order().unwrap();
    rgb.axis_iter_mut(ndarray::Axis(0))
        .into_par_iter()
        .enumerate()
        .for_each(|(y, mut row)| {
            let y2 = y * 2;
            let row_slice = row.as_slice_mut().expect("Failed to get mutable slice");
            row_slice
                .par_chunks_mut(3)
                .enumerate()
                .for_each(|(x, pixel)| {
                    let x2 = x * 2;
                    let r = data_ptr[y2 * w + x2];
                    let g1 = data_ptr[y2 * w + x2 + 1];
                    let g2 = data_ptr[(y2 + 1) * w + x2];
                    let b = data_ptr[(y2 + 1) * w + x2 + 1];
                    let g = ((g1 as i64 + g2 as i64) / 2) as i32;
                    pixel[0] = r;
                    pixel[1] = g;
                    pixel[2] = b;
                });
        });
    rgb
}

fn debayer_bggr(data: &Array2<i32>) -> Array3<i32> {
    let (h, w) = data.dim();
    let h2 = h / 2;
    let w2 = w / 2;
    let mut rgb = Array3::<i32>::zeros((h2, w2, 3));
    let data_ptr = data.as_slice_memory_order().unwrap();
    rgb.axis_iter_mut(ndarray::Axis(0))
        .into_par_iter()
        .enumerate()
        .for_each(|(y, mut row)| {
            let y2 = y * 2;
            let row_slice = row.as_slice_mut().expect("Failed to get mutable slice");
            row_slice
                .par_chunks_mut(3)
                .enumerate()
                .for_each(|(x, pixel)| {
                    let x2 = x * 2;
                    let b = data_ptr[y2 * w + x2];
                    let g1 = data_ptr[y2 * w + x2 + 1];
                    let g2 = data_ptr[(y2 + 1) * w + x2];
                    let r = data_ptr[(y2 + 1) * w + x2 + 1];
                    let g = ((g1 as i64 + g2 as i64) / 2) as i32;
                    pixel[0] = r;
                    pixel[1] = g;
                    pixel[2] = b;
                });
        });
    rgb
}

fn debayer_grbg(data: &Array2<i32>) -> Array3<i32> {
    let (h, w) = data.dim();
    let h2 = h / 2;
    let w2 = w / 2;
    let mut rgb = Array3::<i32>::zeros((h2, w2, 3));
    let data_ptr = data.as_slice_memory_order().unwrap();
    rgb.axis_iter_mut(ndarray::Axis(0))
        .into_par_iter()
        .enumerate()
        .for_each(|(y, mut row)| {
            let y2 = y * 2;
            let row_slice = row.as_slice_mut().expect("Failed to get mutable slice");
            row_slice
                .par_chunks_mut(3)
                .enumerate()
                .for_each(|(x, pixel)| {
                    let x2 = x * 2;
                    let g1 = data_ptr[y2 * w + x2];
                    let r = data_ptr[y2 * w + x2 + 1];
                    let b = data_ptr[(y2 + 1) * w + x2];
                    let g2 = data_ptr[(y2 + 1) * w + x2 + 1];
                    let g = ((g1 as i64 + g2 as i64) / 2) as i32;
                    pixel[0] = r;
                    pixel[1] = g;
                    pixel[2] = b;
                });
        });
    rgb
}

fn debayer_gbrg(data: &Array2<i32>) -> Array3<i32> {
    let (h, w) = data.dim();
    let h2 = h / 2;
    let w2 = w / 2;
    let mut rgb = Array3::<i32>::zeros((h2, w2, 3));
    let data_ptr = data.as_slice_memory_order().unwrap();
    rgb.axis_iter_mut(ndarray::Axis(0))
        .into_par_iter()
        .enumerate()
        .for_each(|(y, mut row)| {
            let y2 = y * 2;
            let row_slice = row.as_slice_mut().expect("Failed to get mutable slice");
            row_slice
                .par_chunks_mut(3)
                .enumerate()
                .for_each(|(x, pixel)| {
                    let x2 = x * 2;
                    let g1 = data_ptr[y2 * w + x2];
                    let b = data_ptr[y2 * w + x2 + 1];
                    let r = data_ptr[(y2 + 1) * w + x2];
                    let g2 = data_ptr[(y2 + 1) * w + x2 + 1];
                    let g = ((g1 as i64 + g2 as i64) / 2) as i32;
                    pixel[0] = r;
                    pixel[1] = g;
                    pixel[2] = b;
                });
        });
    rgb
}

pub fn debayer_image(data: &Array2<i32>, pattern: BayerPattern) -> Array3<i32> {
    match pattern {
        BayerPattern::RGGB => debayer_rggb(data),
        BayerPattern::BGGR => debayer_bggr(data),
        BayerPattern::GRBG => debayer_grbg(data),
        BayerPattern::GBRG => debayer_gbrg(data),
        _ => panic!("Unsupported Bayer pattern"),
    }
}
