use rand::{
  Rng,
  rngs::StdRng,
  SeedableRng
};

use image::{ImageBuffer, Rgb};

use crate::permutation_table::PermutationTable;

const B: usize = 0x100;
const BM: usize = 0xff;
const N: f64 = 0x1000 as f64;

pub struct Perlin {
  perm: PermutationTable,
  g1: Vec<f64>,         // 1D gradients
  g2: Vec<[f64; 2]>,    // 2D gradients
  g3: Vec<[f64; 3]>,
}


impl Perlin {
//   pub const DEFAULT_SEED: u64 = 0;

  pub fn new(seed: u64) -> Self {
//     let mut rng = StdRng::seed_from_u64(seed);

    let perm = PermutationTable::new();

    let g1: Vec<f64> = (0..B)
      .map(|_| (rng.gen_range(0.0..2.0) - 1.0))
      .collect();

//     let mut g2: Vec<[f64; 2]> = vec![[0.0; 2]; B];
//     for i in 0..B {
//       let vec: [f64; 2] = [rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0)];
//       g2[i] = normalize2(vec);
//     }

//     let mut g3: Vec<[f64; 3]> = vec![[0.0; 3]; B];
//     for i in 0..B {
//       let vec = [
//         rng.gen_range(-1.0..1.0),
//         rng.gen_range(-1.0..1.0),
//         rng.gen_range(-1.0..1.0),
//       ];
//       g3[i] = normalize3(vec);
//     }

    Self { perm, g1, g2, g3 }
  }

  // Computes the dot product between the offset vector (x, y)
  // and the gradient vector implicitly encoded by the input hash.
  // The hash is mapped to one of 16 possible gradient directions.
  fn gradient(&self, hash: u8, x: f64, y: f64) -> f64 {
    let h = (hash & 15) as i32;
    let u = if h < 8 { x } else { y };
    let v = if h < 4 { y } else if h == 12 || h == 14 { x } else { 0.0 };

    // calculate sign of 
    let mut res = if (h & 1) == 0 { u } else { -u };
    res += if (h & 2) == 0 { v } else { -v };
    res
  }
//   pub fn noise1(&self, arg: f64) -> f64 {
//     let t = arg + N;
//     let bx0 = (t as usize) & BM;
//     let bx1 = (bx0 + 1) & BM;
//     let rx0 = t.fract();
//     let rx1 = rx0 - 1.0;
    
//     let sx = s_curve(rx0);

//     let u = rx0 * self.g1[self.perm[bx0] as f64];
//     let v = rx1 * self.g1[self.perm[bx1]];

//     lerp(sx, u, v)
//   }

  pub fn noise2(&self, point: [f64; 2]) -> f64 {
    // Find bottom left coord for unit square containing point
    let x_floor = point[0].floor() as i32 & 255;
    let y_floor = point[1].floor() as i32 & 255;

    // local position x, y in square
    let xrel = point[0] - point[0].floor();
    let yrel = point[1] - point[1].floor();

    // compute fade curves
    let u = s_curve(xrel);
    let v = s_curve(yrel);

    // Hash val of each corner of square
    // bottom left
    let b00 = self.perm[(self.perm[x_floor as usize] + y_floor as u8) as usize] as u8;
    // bottom right
    let b10 = self.perm[(self.perm[(x_floor + 1) as usize] + y_floor as u8) as usize] as u8;
    // top left
    let b01 = self.perm[(self.perm[x_floor as usize] + (y_floor as u8 + 1)) as usize] as u8;
    // top right
    let b11 = self.perm[(self.perm[(x_floor + 1) as usize] + (y_floor as u8 + 1)) as usize] as u8;

    // calculate all gradients, pass in point position relative to grid point 
    let g00 = self.gradient(b00, xrel, yrel);
    let g10 = self.gradient(b10, xrel - 1.0, yrel);
    let g01 = self.gradient(b10, xrel, yrel - 1.0);
    let g11 = self.gradient(b10, xrel - 1.0, yrel - 1.0);

    let x_lerp: f64 = lerp(u, g00, g10);
    let y_lerp = lerp(u, g)

    lerp(sy, a, b)
  }

}

// impl Default for Perlin {
//   fn default() -> Self {
//       Self::new(Self::DEFAULT_SEED)
//   }
// }

// fn normalize2(v: [f64; 2]) -> [f64; 2] {
//   let magnitude = (v[0] * v[0] + v[1] * v[1]).sqrt();
//   [v[0] / magnitude, v[1] / magnitude]
// }

// fn normalize3(v: [f64; 3]) -> [f64; 3] {
//   let magnitude = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
//   [v[0] / magnitude, v[1] / magnitude, v[2] / magnitude]
// }

// smoothstep function to transition from 0 to 1
fn s_curve(t: f64) -> f64 {
  t * t * (3. - (2. * t))
}

// linear interpolation
fn lerp(t: f64, a: f64, b: f64) -> f64 {
  a + t * (b - a)
} 

// pub fn generate_perlin_image(noise: &Perlin, width: u32, height: u32, scale: f64) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
//   let mut img = ImageBuffer::new(width, height);

//   for y in 0..height {
//       for x in 0..width {
//           // Scale the coordinates to control the noise frequency
//           let nx: f64 = x as f64 / width as f64 * scale;
//           let ny = y as f64 / height as f64 * scale;

//           // Generate noise values for RGB channels
//           let r = noise.noise2([nx, ny]);
//           let g = noise.noise2([nx + 10.0, ny + 10.0]); // Offset for green
//           let b = noise.noise2([nx * 1.5, ny * 1.5]);   // Scale for blue

//           // Normalize noise from [-1, 1] to [0, 255]
//           let r = ((r + 1.0) * 127.5).clamp(0.0, 255.0) as u8;
//           let g = ((g + 1.0) * 127.5).clamp(0.0, 255.0) as u8;
//           let b = ((b + 1.0) * 127.5).clamp(0.0, 255.0) as u8;

//           // Set the pixel in the image
//           img.put_pixel(x, y, Rgb([r, g, b]));
//       }
//   }

//   img
// }

// #[cfg(test)]
// mod tests {
//   use super::*;

//   #[test]
//   fn test_noise1() {
//     calculate_noise();
//   }
// }