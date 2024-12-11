use std::path::PathBuf;

use image::{Rgb, RgbImage};

pub struct NoiseImage {
  width: usize,
  height: usize,
  filename: Option<PathBuf>,
  data: Vec<f64>
}

impl NoiseImage {
  pub fn new(width: usize, height: usize, filename: Option<PathBuf>, data: Vec<f64>) -> Self {
    Self {
      width,
      height,
      filename,
      data
    }
  }

  pub fn get_data_for_wasm(&self) -> Vec<u8> {
    // Every 4 indices corresponds to pixel channels R, G, B, and alpha
    let mut wasm_data: Vec<u8> = Vec::with_capacity(self.width * self.height * 4);
    self.data.iter().enumerate().map(|(idx, v)| {
      wasm_data[idx * 4] = self.data[idx] as u8;
      wasm_data[(idx * 4) + 1] = self.data[idx] as u8;
      wasm_data[(idx * 4) + 2] = self.data[idx] as u8;
      wasm_data[(idx * 4) + 3] = 255; // alpha
    });

    wasm_data
  }

  pub fn write_to_file(&self) {
    let mut img = RgbImage::new(self.width as u32, self.height as u32);

    for y in 0..self.height {
      for x in 0..self.width {
          let value = self.data[(y * self.width) + x];
          let rgb = Rgb([value as u8, value as u8, value as u8]);
          img.put_pixel(x as u32, y as u32, rgb);
      }
  }
    let filename = self.filename.as_ref().expect("Error: missing filename");
    img.save(filename.as_path()).expect("Error while saving filename");
  }
}

pub trait NoiseFn {
  fn calc(&mut self, point: [f64; 2]) -> f64;
}

pub struct NoiseImageBuilder<F>
where
    F: NoiseFn,
{
  noise_fn: F, // Generic callable
  width: usize,
  height: usize,
  filename: Option<PathBuf>,
  data: Vec<f64>,
}

impl<F> NoiseImageBuilder<F>
where
    F: NoiseFn,
{
  pub fn new(noise_fn: F) -> Self {
    let default_dim = 150;
    Self {
      noise_fn,
      width: default_dim,
      height: default_dim,
      filename: None,
      data: Vec::with_capacity(default_dim * default_dim),
    }
  }

  pub fn set_size(self, width: usize, height: usize) -> Self {
    Self {
      width,
      height,
      data: Vec::with_capacity(width * height),
      ..self
    }
  }

  pub fn set_filename<P: Into<PathBuf>>(self, filename: P) -> Self {
    Self {
      filename: Some(filename.into()),
      ..self
    }
  }

  pub fn build(mut self) -> NoiseImage {
    self.data.clear();
    for x in 0..self.width {
      for y in 0..self.height {
        let point = [x as f64, y as f64];
        let value = self.noise_fn.calc(point);
        self.data.push(value);
      }
    }
    NoiseImage::new(self.width, self.height, self.filename, self.data)
  }
}