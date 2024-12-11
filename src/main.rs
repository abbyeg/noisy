mod core;
mod utils;

use core::white::WhiteNoise;
use utils::noise_image::NoiseImageBuilder;


fn main() {
  NoiseImageBuilder::new(WhiteNoise::default())
    .set_filename("example_images/white-noise.png")
    .set_size(500, 400)
    .build()
    .write_to_file();
}
