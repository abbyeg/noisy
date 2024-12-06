mod utils;
mod perlin;
mod permutation_table;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn draw_pixels(canvas_width: u32, canvas_height: u32) -> Vec<u8> {
    let mut pixels = vec![0; (canvas_width*canvas_height*4) as usize];
    for y in 0..canvas_height {
        for x in 0..canvas_width {
            let idx = ((y*canvas_width + x)*4) as usize;
            let value = if x % 2 == 0 { 0 } else { 255 };
            pixels[idx] = value;
            pixels[idx + 1] = value;
            pixels[idx + 2] = value;
            pixels[idx + 3] = 255; // alpha
        }
    }
    pixels
}
