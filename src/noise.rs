use rand::{
  Rng,
  rngs::StdRng,
  SeedableRng
};

pub struct RandomNoise {
  rng: StdRng,
}

impl RandomNoise {
  pub fn new(seed: u64) -> Self {
    let rng = StdRng::seed_from_u64(seed);
    Self {
      rng
    }
  }
  
  pub fn calc(&mut self) -> f64 {
    self.rng.gen_range(0.0..255.0)
  }
}

impl Default for RandomNoise {
  fn default() -> Self {
    let rng = StdRng::seed_from_u64(0);
    Self {
      rng
    }
  }
}
