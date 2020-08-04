extern crate image;

use image::{Rgb, RgbImage};

fn wave(rot: f64, phase: f64, x: f64, y: f64) -> f64 {
  let (srot, crot) = rot.sin_cos();
  let inner = (crot * x as f64) + (srot * y as f64) + phase;
  return (inner.cos() + 1.0) / 2.0;
}

fn combine(waves: Vec<f64>) -> f64 {
  let sum = waves.iter().sum::<f64>();
  let wrapped = sum % 1.0;
  if sum as i64 % 2 == 0 {
    return wrapped;
  }
  return 1.0 - wrapped;
}

fn angles(n: u32) -> Vec<f64> {
  let pi = std::f64::consts::PI;
  let partitions = 0..n;
  let part_to_angle = |p| p as f64 * (pi / n as f64);
  return partitions.into_iter().map(part_to_angle).collect();
}

fn scaled_point(scale: u32, size: u32, point: u32) -> f64 {
  return scale as f64 * ((2.0 * point as f64 / (size as f64 - 1.0)) - 1.0);
}

pub struct Params {
  pub order: u32,
  pub scale: u32,
  pub width: u32,
  pub height: u32
}

pub fn gen<F: Fn(f64) -> Rgb<u8>>(colorize: F, phase: f64, params: &Params) -> RgbImage {
  let Params { order, scale, width, height } = *params;
  let angs = angles(order);
  let mut imgbuf = RgbImage::new(width, height);
  for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
    let max_dim = width.max(height);
    let scaled_x = scaled_point(scale, max_dim, x);
    let scaled_y = scaled_point(scale, max_dim, y);
    let part_wave = |rot: &f64| wave(*rot, phase, scaled_x, scaled_y);
    let waves = angs.iter().map(part_wave);
    let stacked = combine(waves.collect());
    let clamped = (1 as f64).min(stacked.max(0 as f64));
    let shade = clamped * 255.0;
    *pixel = colorize(shade);
  }
  return imgbuf;
}
