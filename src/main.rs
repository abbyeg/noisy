mod perlin;
mod permutation_table;
use perlin::Perlin;

fn main() {
  let pn = Perlin::default();
  // let noise_img =  perlin::generate_perlin_line_1d(&pn, 600, 5.0);
  // noise_img.save("perlin_noise_n1.png").expect("Failed to save image");
  let noise_img =  perlin::generate_perlin_image(&pn, 600, 500, 5.0);
  noise_img.save("perlin_noise_n2.png").expect("Failed to save image");
}
