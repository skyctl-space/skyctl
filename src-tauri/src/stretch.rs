
use std::simd::f32x8;
use std::simd::cmp::SimdPartialOrd;
use std::simd::num::SimdFloat;

use ndarray::Array2;

#[derive(Debug, Copy, Clone)]
pub struct Stretch {
    pub target_bkg: f32,
    pub shadows_clip: f32,
}

fn median(data: &Array2<f32>) -> f32 {
    let mut v: Vec<f32> = data.iter().copied().collect();
    let mid = v.len() / 2;
    let (_, median, _) = v.select_nth_unstable_by(mid, |a, b| a.partial_cmp(b).unwrap());
    *median
}

impl Stretch {
    pub fn new(target_bkg: f32, shadows_clip: f32) -> Self {
        Self {
            target_bkg,
            shadows_clip,
        }
    }

    fn avg_dev(&self, data: &Array2<f32>) -> f32 {
        let median = median(data);
        let n = (data.len()) as f32;
        data.iter().map(|&x| (x - median).abs()).sum::<f32>() / n
    }

    fn mtf(&self, m: f32, x: f32) -> f32 {
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
            table.push(self.mtf(m, x));  // Assuming `mtf(m, x)` is your existing function
        }
    
        table
    }

    fn stretch_params(&self, data: &Array2<f32>) -> (f32, f32, f32) {
        let median = median(data);
        let avg_dev = self.avg_dev(data);

        let c0 = (median + self.shadows_clip * avg_dev).clamp(0.0, 1.0);
        let m = self.mtf(self.target_bkg, median - c0);
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
        let simd_width = 8;
        let chunks = len / simd_width;
    
        for i in 0..chunks {
            let idx = i * simd_width;
            let chunk = f32x8::from_slice(&flat[idx..idx + simd_width]);
            let mask = chunk.simd_lt(f32x8::splat(c0));
            let adj = ((chunk - f32x8::splat(c0)) / f32x8::splat(inv_c0))
                .simd_max(f32x8::splat(0.0)).simd_min(f32x8::splat(1.0));
    
            let mut interpolated = [0.0; 8];
            for j in 0..simd_width {
                interpolated[j] = Self::interpolate_mtf(&table, adj[j]);
            }
            let result = mask.select(f32x8::splat(0.0), f32x8::from_array(interpolated));
            result.copy_to_slice(&mut flat[idx..idx + simd_width]);
        }
    
        // Tail
        for i in chunks * simd_width..len {
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