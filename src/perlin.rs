use rand::{
  Rng,
  rngs::StdRng,
  SeedableRng
};

use image::{ImageBuffer, Rgb};

use crate::permutation_table::Hasher;
use crate::permutation_table::PermutationTable;
use crate::permutation_table::PERM_TABLE_SIZE;

pub struct Perlin {
  perm: PermutationTable,
  g1: Vec<f64>,         // 1D gradients
  g2: Vec<[f64; 2]>,    // 2D gradients
  g3: Vec<[f64; 3]>,
}

impl Perlin {
  pub const DEFAULT_SEED: u64 = 0;

  pub fn new(seed: u64) -> Self {
    let mut rng = StdRng::seed_from_u64(seed);

    let perm = PermutationTable::new();

    let g1: Vec<f64> = (0..PERM_TABLE_SIZE)
      .map(|_| rng.gen_range(-1.0..1.0))
      .collect();

    let mut g2: Vec<[f64; 2]> = vec![[0.0; 2]; PERM_TABLE_SIZE];
    for i in 0..PERM_TABLE_SIZE {
      let vec: [f64; 2] = [rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0)];
      g2[i] = normalize2(vec);
    }

    let mut g3: Vec<[f64; 3]> = vec![[0.0; 3]; PERM_TABLE_SIZE];
    for i in 0..PERM_TABLE_SIZE {
      let vec = [
        rng.gen_range(-1.0..1.0),
        rng.gen_range(-1.0..1.0),
        rng.gen_range(-1.0..1.0),
      ];
      g3[i] = normalize3(vec);
    }

    Self { perm, g1, g2, g3 }
  }

  // 1D noise utilizes unit lines
  pub fn noise1(&self, arg: f64) -> f64 {
    let floor = arg.floor() as usize & (PERM_TABLE_SIZE - 1);
    let ceil = (floor + 1) as usize & (PERM_TABLE_SIZE - 1);
    let rel = arg - floor as f64;
    
    let u = s_curve(rel);

    let g0 = rel * self.g1[self.perm[floor as usize] as usize];
    let g1 = (rel - 1.0) * self.g1[self.perm[ceil as usize] as usize];

    lerp(u, g0, g1)
  }

  pub fn noise2(&self, point: [f64; 2]) -> f64 {
    // Find bottom left coord for unit square containing point
    let x_floor = point[0].floor() as usize & 255;
    let y_floor = point[1].floor() as usize & 255;

    // relative position x, y in square
    let xrel = point[0] - point[0].floor();
    let yrel = point[1] - point[1].floor();

    // compute fade curves
    let u = s_curve(xrel);
    let v = s_curve(yrel);

    // Hash val of each corner of square
    // bottom left
    let b00 = self.perm.hash(&[x_floor, y_floor]);
    // // bottom right
    let b10 = self.perm.hash(&[x_floor+1, y_floor]);
    // // top left
    let b01 = self.perm.hash(&[x_floor, y_floor+1]);
    // // top right
    let b11 = self.perm.hash(&[x_floor+1, y_floor+1]);

    let g00 = self.g2[b00][0] * xrel + self.g2[b00][1] * yrel;
    let g10 = self.g2[b10][0] * (xrel - 1.0) + self.g2[b10][1] * yrel;
    let g01 = self.g2[b01][0] * xrel + self.g2[b01][1] * (yrel - 1.0);
    let g11 = self.g2[b11][0] * (xrel - 1.0) + self.g2[b11][1] * (yrel - 1.0);

    // println!("{} {} {} {} ", g00, g10, g01, g11);

    let x_lerp = lerp(u, g00, g10);
    let y_lerp = lerp(u, g01, g11);

    lerp(v, x_lerp, y_lerp)
  }

}

impl Default for Perlin {
  fn default() -> Self {
      Self::new(Self::DEFAULT_SEED)
  }
}

fn normalize2(v: [f64; 2]) -> [f64; 2] {
  let magnitude = (v[0] * v[0] + v[1] * v[1]).sqrt();
  [v[0] / magnitude, v[1] / magnitude]
}

fn normalize3(v: [f64; 3]) -> [f64; 3] {
  let magnitude = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
  [v[0] / magnitude, v[1] / magnitude, v[2] / magnitude]
}

// smoothstep function to transition from 0 to 1
fn s_curve(t: f64) -> f64 {
  t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
}

// linear interpolation
fn lerp(t: f64, a: f64, b: f64) -> f64 {
  a + t * (b - a)
}

pub fn generate_perlin_image(noise: &Perlin, width: u32, height: u32, scale: f64) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
  let mut img = ImageBuffer::new(width, height);

  for y in 0..height {
      for x in 0..width {
          // Scale the coordinates to control the noise frequency
          let nx = x as f64 / width as f64 * scale;
          let ny = y as f64 / height as f64 * scale;

          // Generate noise values for RGB channels
          let r = noise.noise2([nx, ny]);
          let g = noise.noise2([nx + 10.0, ny + 10.0]); // Offset for green
          let b = noise.noise2([nx * 1.5, ny * 1.5]);   // Scale for blue

          // Normalize noise from [-1, 1] to [0, 255]
          let r = ((r + 1.0) * 127.5).clamp(0.0, 255.0) as u8;
          let g = ((g + 1.0) * 127.5).clamp(0.0, 255.0) as u8;
          let b = ((b + 1.0) * 127.5).clamp(0.0, 255.0) as u8;

          // Set the pixel in the image
          img.put_pixel(x, y, Rgb([r, g, b]));
      }
  }

  img
}


pub fn generate_perlin_line_1d(noise: &Perlin, width: u32, scale: f64) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
  let mut img = ImageBuffer::new(width,1);

  for x in 0..width {
    // Scale the coordinates to control the noise frequency
    let nx = x as f64 / width as f64 * scale;

    // Generate noise values for RGB channels
    let r = noise.noise1(nx);
    let g = noise.noise1(nx + 10.0); // Offset for green
    let b = noise.noise1(nx * 1.5);   // Scale for blue

    // Normalize noise from [-1, 1] to [0, 255]
    let r = ((r + 1.0) * 127.5).clamp(0.0, 255.0) as u8;
    let g = ((g + 1.0) * 127.5).clamp(0.0, 255.0) as u8;
    let b = ((b + 1.0) * 127.5).clamp(0.0, 255.0) as u8;

    // Set the pixel in the image
    img.put_pixel(x, 0, Rgb([r, g, b]));
  }

  img
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_noise_1d() {
    let perlin = Perlin::default();
    assert_eq!(perlin.noise1(0.0), 0.0);
    assert_eq!(perlin.noise1(1.0), 0.0);
    assert_eq!(perlin.noise1(2.0), 0.0);
  }
}