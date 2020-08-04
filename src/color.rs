use image::{Rgb};

fn sawtooth(n: u32, max: u32, offset: f64, step: u32) -> f64 {
  let pi = std::f64::consts::PI;
  let adjusted_n = offset + (n as f64 / max as f64);
  let npi = adjusted_n * pi;
  return (step as f64 * pi * (npi.sin()).asin()).abs();
}

pub fn saw_colorize(shade: f64, frame: u32, frames: u32) -> Rgb<u8> {
  let color = |off| (shade - sawtooth(frame, frames, off, 51)).abs() as u8;
  let r = color(0.0);
  let g = color(0.25);
  let b = color(0.5);
  return Rgb([r, g, b]);
}