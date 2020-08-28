use image::{Rgb};

pub enum Colorizer {
  Saw,
  Greyscale,
}

fn sawtooth(n: u32, max: u32, offset: f64, step: f64) -> f64 {
  let pi = std::f64::consts::PI;
  let adjusted_n = offset + (n as f64 / max as f64);
  let npi = adjusted_n * pi;
  return (step as f64 * pi * (npi.sin()).asin()).abs();
}

fn saw_colorize(shade: f64, frame: u32, frames: u32, params: &Vec<f64>) -> Rgb<u8> {
  let color = |off: f64| (shade - sawtooth(frame, frames, off, params[3])).abs() as u8;
  let rgb: [u8; 3] = [color(params[0]), color(params[1]), color(params[2])];
  return Rgb(rgb);
}

fn grey_colorize(shade: f64, params: &Vec<f64>) -> Rgb<u8>{
  let s = ((params[0] * 2.0) - 255.0 + shade) as u8;
  return Rgb([s, s, s]);
}

pub fn colorize(color: &Colorizer, shade: f64, frame: u32, frames: u32, params: &Vec<f64>) -> Rgb<u8> {
  match *color {
    Colorizer::Saw => return saw_colorize(shade, frame, frames, params),
    Colorizer::Greyscale => return grey_colorize(shade, params)
  }
}