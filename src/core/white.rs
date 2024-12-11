use rand::{rngs::StdRng, Rng, SeedableRng};

use crate::utils::noise_image::NoiseFn;

pub struct WhiteNoise {
  rng: StdRng
}

impl WhiteNoise {
  pub fn new(seed: u64) -> Self {
    Self {
      rng:  StdRng::seed_from_u64(seed)
    }
  }
}

impl Default for WhiteNoise {
  fn default() -> Self {
    WhiteNoise {
        rng:  StdRng::seed_from_u64(0)
      }
  }
}

impl NoiseFn for WhiteNoise {
    fn calc(&mut self, _point: [f64; 2]) -> f64 {
      self.rng.gen_range(0.0..255.0)
    }
}
