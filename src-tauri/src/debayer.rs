use ndarray::{Array2, Array3};
use rayon::prelude::*;

#[allow(dead_code)]
#[derive(PartialEq, Copy, Clone, Debug)]
pub enum BayerPattern {
    NONE,
    RGGB,
    BGGR,
    GRBG,
    GBRG,
}


fn debayer_rggb(data: &Array2<f32>) -> Array3<f32> {
    let (h, w) = data.dim();
    let h2 = h / 2;
    let w2 = w / 2;

    let mut rgb = Array3::<f32>::zeros((h2, w2, 3));

    // Get the raw pointer to the data
    let data_ptr = data.as_slice_memory_order().unwrap();

    // Parallelize over rows
    rgb.axis_iter_mut(ndarray::Axis(0))
        .into_par_iter()
        .enumerate()
        .for_each(|(y, mut row)| {
            let y2 = y * 2;

            // Get a mutable slice of the row
            let row_slice = row.as_slice_mut().expect("Failed to get mutable slice");

            // Iterate over each pixel in the row using par_chunks_mut
            row_slice
                .par_chunks_mut(3)  // Each chunk represents a pixel with 3 values (RGB)
                .enumerate()
                .for_each(|(x, pixel)| {
                    let x2 = x * 2;

                    // Access the RGGB pattern using raw pointers
                    let r = data_ptr[y2 * w + x2];
                    let g1 = data_ptr[y2 * w + x2 + 1];
                    let g2 = data_ptr[(y2 + 1) * w + x2];
                    let b = data_ptr[(y2 + 1) * w + x2 + 1];

                    // Apply bilinear interpolation for green channel (G)
                    let g = (g1 + g2) * 0.5;

                    // Store the pixel values in RGB
                    pixel[0] = r;  // Red channel
                    pixel[1] = g;  // Green channel
                    pixel[2] = b;  // Blue channel
                });
        });

    rgb
}

fn debayer_bggr(data: &Array2<f32>) -> Array3<f32> {
    let (h, w) = data.dim();
    let h2 = h / 2;
    let w2 = w / 2;

    let mut rgb = Array3::<f32>::zeros((h2, w2, 3));

    // Get the raw pointer to the data
    let data_ptr = data.as_slice_memory_order().unwrap();

    // Parallelize over rows
    rgb.axis_iter_mut(ndarray::Axis(0))
        .into_par_iter()
        .enumerate()
        .for_each(|(y, mut row)| {
            let y2 = y * 2;

            // Get a mutable slice of the row
            let row_slice = row.as_slice_mut().expect("Failed to get mutable slice");

            // Iterate over each pixel in the row using par_chunks_mut
            row_slice
                .par_chunks_mut(3)  // Each chunk represents a pixel with 3 values (RGB)
                .enumerate()
                .for_each(|(x, pixel)| {
                    let x2 = x * 2;

                    // Access the BGGR pattern using raw pointers
                    let b = data_ptr[y2 * w + x2];
                    let g1 = data_ptr[y2 * w + x2 + 1];
                    let g2 = data_ptr[(y2 + 1) * w + x2];
                    let r = data_ptr[(y2 + 1) * w + x2 + 1];

                    // Apply bilinear interpolation for green channel (G)
                    let g = (g1 + g2) * 0.5;

                    // Store the pixel values in RGB
                    pixel[0] = r;  // Red channel
                    pixel[1] = g;  // Green channel
                    pixel[2] = b;  // Blue channel
                });
        });

    rgb
}

fn debayer_grbg(data: &Array2<f32>) -> Array3<f32> {
    let (h, w) = data.dim();
    let h2 = h / 2;
    let w2 = w / 2;

    let mut rgb = Array3::<f32>::zeros((h2, w2, 3));

    // Get the raw pointer to the data
    let data_ptr = data.as_slice_memory_order().unwrap();

    // Parallelize over rows
    rgb.axis_iter_mut(ndarray::Axis(0))
        .into_par_iter()
        .enumerate()
        .for_each(|(y, mut row)| {
            let y2 = y * 2;

            // Get a mutable slice of the row
            let row_slice = row.as_slice_mut().expect("Failed to get mutable slice");

            // Iterate over each pixel in the row using par_chunks_mut
            row_slice
                .par_chunks_mut(3)  // Each chunk represents a pixel with 3 values (RGB)
                .enumerate()
                .for_each(|(x, pixel)| {
                    let x2 = x * 2;

                    // Access the GRBG pattern using raw pointers
                    let g1 = data_ptr[y2 * w + x2];
                    let r = data_ptr[y2 * w + x2 + 1];
                    let b = data_ptr[(y2 + 1) * w + x2];
                    let g2 = data_ptr[(y2 + 1) * w + x2 + 1];

                    // Apply bilinear interpolation for green channel (G)
                    let g = (g1 + g2) * 0.5;

                    // Store the pixel values in RGB
                    pixel[0] = r;  // Red channel
                    pixel[1] = g;  // Green channel
                    pixel[2] = b;  // Blue channel
                });
        });

    rgb
}

fn debayer_gbrg(data: &Array2<f32>) -> Array3<f32> {
    let (h, w) = data.dim();
    let h2 = h / 2;
    let w2 = w / 2;

    let mut rgb = Array3::<f32>::zeros((h2, w2, 3));

    // Get the raw pointer to the data
    let data_ptr = data.as_slice_memory_order().unwrap();

    // Parallelize over rows
    rgb.axis_iter_mut(ndarray::Axis(0))
        .into_par_iter()
        .enumerate()
        .for_each(|(y, mut row)| {
            let y2 = y * 2;

            // Get a mutable slice of the row
            let row_slice = row.as_slice_mut().expect("Failed to get mutable slice");

            // Iterate over each pixel in the row using par_chunks_mut
            row_slice
                .par_chunks_mut(3)  // Each chunk represents a pixel with 3 values (RGB)
                .enumerate()
                .for_each(|(x, pixel)| {
                    let x2 = x * 2;

                    // Access the GBRG pattern using raw pointers
                    let g1 = data_ptr[y2 * w + x2];
                    let b = data_ptr[y2 * w + x2 + 1];
                    let r = data_ptr[(y2 + 1) * w + x2];
                    let g2 = data_ptr[(y2 + 1) * w + x2 + 1];

                    // Apply bilinear interpolation for green channel (G)
                    let g = (g1 + g2) * 0.5;

                    // Store the pixel values in RGB
                    pixel[0] = r;  // Red channel
                    pixel[1] = g;  // Green channel
                    pixel[2] = b;  // Blue channel
                });
        });

    rgb
}

pub fn debayer_image(data: &Array2<f32>, pattern: BayerPattern) -> Array3<f32> {
    match pattern {
        BayerPattern::RGGB => debayer_rggb(data),
        BayerPattern::BGGR => debayer_bggr(data),
        BayerPattern::GRBG => debayer_grbg(data),
        BayerPattern::GBRG => debayer_gbrg(data),
        _ => panic!("Unsupported Bayer pattern"),
    }
}