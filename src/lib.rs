mod utils;

use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


/// Returns an updated u8 buffer
/// From JS: pass a UInt8ClampedArray for the Clamped<Vec<u8>>
/// 
/// * `ctx` - the 2d rendering context
/// * `data` - the UInt8ClampedArray data of ImageData#data
/// * `start_x` - the x coordinate on the canvas
/// * `start_x` - the y coordinate on the canvas
/// * `r` - the red in the RGB fill color
/// * `g` - the green in the RGB fill color
/// * `b` - the blue in the RGB fill color
/// * `tolerance` - the tolerance

// #[wasm_bindgen]
#[wasm_bindgen(catch)]
pub fn flood_fill(
  ctx: &CanvasRenderingContext2d, 
  mut data: Clamped<Vec<u8>>,
  start_x: u32,
  start_y: u32,
  r: u8,
  g: u8,
  b: u8,
  tolerance: u8
) -> Result<Clamped<Vec<u8>>, JsValue> {

  let canvas = ctx.canvas().unwrap();
  let width = canvas.width();
  let height = canvas.height();

  let (tr, tg, tb, _ta) = get_pixel(&data, width, start_x, start_y);

  if match_colors(r, g, b, tr, tg, tb) {
    return Ok(data);
  }

  let mut stack: Vec<u32> = vec![start_x, start_y];

  while !stack.is_empty() {
    let y = stack.pop().unwrap();
    let x = stack.pop().unwrap();

    if x >= width || y >= height {
      continue;
    }

    let (cr, cg, cb, _ca) = get_pixel(&data, width, x, y);


    if match_colors(r, g, b, cr, cg, cb) {
      continue;
    }

    let match_color = match_tolerance(tr, tg, tb, cr, cg, cb, tolerance);

    if match_color {
      set_pixel(&mut data, width, x, y, r, g, b, 255);
      stack.push(x + 1);
      stack.push(y);

      if x > 0 {
        stack.push(x - 1);
        stack.push(y);
      }

      stack.push(x);
      stack.push(y + 1);
     
      if y > 0 {
        stack.push(x);
        stack.push(y - 1);
      }
    }
  }

  return Ok(data);
}

pub fn set_pixel(data: &mut Vec<u8>, width: u32, x: u32, y: u32, r: u8, g: u8, b: u8, a: u8) {
  let offset = ((y * width + x) * 4) as usize;

  data[offset] = r;
  data[offset + 1] = g;
  data[offset + 2] = b;
  data[offset + 3] = a;
}

pub fn get_pixel(data: &Vec<u8>, width: u32, x: u32, y: u32) -> (u8, u8, u8, u8) {
  let offset = ((y * width + x) * 4) as usize;
  return (data[offset], data[offset + 1], data[offset + 2], data[offset + 3]);
}

pub fn match_colors(r1: u8, g1: u8, b1: u8, r2: u8, g2: u8, b2: u8) -> bool {
  return r1 == r2 && g1 == g2 && b1 == b2;
}

pub fn match_tolerance(r1: u8, g1: u8, b1: u8, r2: u8, g2: u8, b2: u8, tolerance: u8) -> bool {
  let r = if r1 > r2 { r1 - r2 } else { r2 - r1 };
  let g = if g1 > g2 { g1 - g2 } else { g2 - g1 };
  let b = if b1 > b2 { b1 - b2 } else { b2 - b1 };

  return r < tolerance && g < tolerance && b < tolerance;
}