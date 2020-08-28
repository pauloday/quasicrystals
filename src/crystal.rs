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

// takes a vec of proportions and returns a vec of angles for those sizes
// e.g. [1, 1] is 0, pi/2
// [1, 2] is 0, 2pi/3
// [1, 2, 1] is 0, 2pi/4, 3pi/4
pub fn custom_angles(props: Vec<u32>) -> Vec<f64> {
  let pi = std::f64::consts::PI;
  let sections: u32 = props.iter().sum();
  let prop_to_angle = |angles: Vec<f64>, prop: &u32| {
    let angle = *prop as f64 * (pi / sections as f64);
    let mut new_angles = angles.clone();
    match angles.last() {
      Some(a) => new_angles.push(angle + a),
      None => new_angles.push(angle)
    }
    return new_angles;
  };
  return props.iter().fold([].to_vec(), prop_to_angle);
}

fn scaled_point(scale: u32, size: u32, point: u32) -> f64 {
  return scale as f64 * ((2.0 * point as f64 / (size as f64 - 1.0)) - 1.0);
}

pub struct Params {
  pub scale: u32,
  pub width: u32,
  pub height: u32,
  pub x_offset: u32,
  pub y_offset: u32
}

pub fn gen<F: Fn(f64) -> Rgb<u8>>(colorize: F, phase: f64, angles: &Vec<f64>, params: &Params) -> RgbImage {
  let Params { scale, width, height, x_offset, y_offset } = *params;
  let mut imgbuf = RgbImage::new(width, height);
  for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
    let max_dim = width.max(height);
    let scaled_x = scaled_point(scale, max_dim, x + x_offset);
    let scaled_y = scaled_point(scale, max_dim, y + y_offset);
    let part_wave = |rot: &f64| wave(*rot, phase, scaled_x, scaled_y);
    let waves = angles.iter().map(part_wave);
    let stacked = combine(waves.collect());
    let clamped = (1.0).min(stacked.max(0 as f64));
    let shade = clamped * 255.0;
    *pixel = colorize(shade);
  }
  return imgbuf;
}
