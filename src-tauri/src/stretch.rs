use std::simd::f32x8;
use std::simd::cmp::SimdPartialOrd;
use std::simd::num::SimdFloat;

use ndarray::Array2;

const SIMD_WIDTH: usize = 8; // Number of elements in a SIMD vector

#[derive(Debug, Copy, Clone)]
pub struct Stretch {
    pub target_bkg: f32,
    pub shadows_clip: f32,
}

fn median(data: &Array2<f32>) -> f32 {
    let len = data.len();
    if len == 0 {
        return 0.0; // Handle empty data gracefully
    }

    let mid = len / 2;

    // Use a slice reference directly for the data
    let mut v: Vec<f32> = data.iter().copied().collect();
    
    // Perform a partial sort to find the median without fully sorting
    let (_, median, _) = v.select_nth_unstable_by(mid, |a, b| a.partial_cmp(b).unwrap());

    *median
}

pub fn calc_channel_stats(data: &Array2<f32>) -> (f32, f32, f32, f32, f32) {
    let median_val = median(data);  // Calculate median only once

    let len = data.len();
    let n = len as f32;
   
    // Flatten the data to 1D for SIMD processing
    let data_flat: Vec<f32> = data.iter().copied().collect();

    let mut sum = 0.0;
    let mut min_val = f32::MAX;
    let mut max_val = f32::MIN;
    let mut sum_val = 0.0;

    // Process data in chunks of SIMD width
    let mut i = 0;

    // Iterate over chunks of SIMD width
    while i + SIMD_WIDTH <= len {
        let chunk = f32x8::from_slice(&data_flat[i..i + SIMD_WIDTH]);
        let dev = (chunk - f32x8::splat(median_val)).abs();
        sum += dev.reduce_sum();
        min_val = min_val.min(chunk.reduce_min());
        max_val = max_val.max(chunk.reduce_max());
        sum_val += chunk.reduce_sum();
        i += SIMD_WIDTH;
    }

    // Handle remaining elements if len is not a multiple of SIMD width
    while i < len {
        let v = data_flat[i];
        sum += (v - median_val).abs();
        min_val = min_val.min(v);
        max_val = max_val.max(v);
        sum_val += v;
        i += 1;
    }

    // Calculate the average absolute deviation
    let avg_dev = sum / n;
    let avg = sum_val / n;

    (median_val, avg_dev, min_val, max_val, avg)
}

fn mtf(m: f32, x: f32) -> f32 {
    if x == 0.0 {
        0.0
    } else if x == 1.0 {
        1.0
    } else if x == m {
        0.5
    } else {
        let num = (m - 1.0) * x;
        let denom = ((2.0 * m) - 1.0) * x - m;
        num / denom
    }
}

impl Stretch {
    pub fn new(target_bkg: f32, shadows_clip: f32) -> Self {
        Self {
            target_bkg,
            shadows_clip,
        }
    }

    // Midtones Transfer Function

    //     MTF(m, x) = {
    //         0                for x == 0,
    //         1/2              for x == m,
    //         1                for x == 1,

    //         (m - 1)x
    //         --------------   otherwise.
    //         (2m - 1)x - m
    //     }

    //     See the section "Midtones Balance" from
    //     https://pixinsight.com/doc/tools/HistogramTransformation/HistogramTransformation.html

    //     Args:
    //         m (float): midtones balance parameter
    //                    a value below 0.5 darkens the midtones
    //                    a value above 0.5 lightens the midtones
    //         x (np.array): the data that we want to copy and transform.


    fn interpolate_mtf(table: &[f32], x: f32) -> f32 {
        let scaled = x * (table.len() - 1) as f32;
        let idx = scaled.floor() as usize;
        let next = (idx + 1).min(table.len() - 1);
        let frac = scaled - idx as f32;
        table[idx] * (1.0 - frac) + table[next] * frac
    }

    fn generate_mtf_lookup_table(&self, m: f32, steps: usize) -> Vec<f32> {
        let mut table = Vec::with_capacity(steps);
    
        // Generate values for each step in the range [0.0, 1.0]
        for i in 0..steps {
            let x = i as f32 / (steps as f32 - 1.0);  // Normalize to [0.0, 1.0]
            table.push(mtf(m, x));  // Assuming `mtf(m, x)` is your existing function
        }
    
        table
    }

    fn stretch_params(&self, data: &Array2<f32>) -> (f32, f32, f32) {
        let (median, avg_dev, _, _, _) = calc_channel_stats(data);

        let c0 = (median + self.shadows_clip * avg_dev).clamp(0.0, 1.0);
        let m = mtf(self.target_bkg, median - c0);
        (c0, 1.0, m)
    }

    pub fn stretch(&self, data: &Array2<f32>) -> Array2<f32> {
        let max_val = data.iter().copied().fold(f32::MIN, f32::max);
        let mut flat: Vec<f32> = data.iter().map(|x| x / max_val).collect();
        let reshaped = Array2::from_shape_vec(data.raw_dim(), flat.clone()).unwrap();
        let (c0, _, m) = self.stretch_params(&reshaped);
        let table = self.generate_mtf_lookup_table(m, 65536);
        let inv_c0 = 1.0 - c0;
    
        let len = flat.len();
        let chunks = len / SIMD_WIDTH;
    
        for i in 0..chunks {
            let idx = i * SIMD_WIDTH;
            let chunk = f32x8::from_slice(&flat[idx..idx + SIMD_WIDTH]);
            let mask = chunk.simd_lt(f32x8::splat(c0));
            let adj = ((chunk - f32x8::splat(c0)) / f32x8::splat(inv_c0))
                .simd_max(f32x8::splat(0.0)).simd_min(f32x8::splat(1.0));
    
            let mut interpolated = [0.0; 8];
            for j in 0..SIMD_WIDTH {
                interpolated[j] = Self::interpolate_mtf(&table, adj[j]);
            }
            let result = mask.select(f32x8::splat(0.0), f32x8::from_array(interpolated));
            result.copy_to_slice(&mut flat[idx..idx + SIMD_WIDTH]);
        }
    
        // Tail
        for i in chunks * SIMD_WIDTH..len {
            let x = flat[i];
            flat[i] = if x < c0 {
                0.0
            } else {
                Self::interpolate_mtf(&table, ((x - c0) / inv_c0).clamp(0.0, 1.0))
            };
        }
    
        Array2::from_shape_vec(data.raw_dim(), flat).unwrap()
    }

}