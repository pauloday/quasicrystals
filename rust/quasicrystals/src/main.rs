extern crate image;

use image::{Rgb, RgbImage};

fn wave(rot: f64, phase: f64, x: u32, y: u32) -> f64 {
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

fn angles(n: i64) -> Vec<f64> {
  let pi = std::f64::consts::PI;
  let partitions = 0..n;
  let part_to_angle = |p| p as f64 * (pi / n as f64);
  return partitions.into_iter().map(part_to_angle).collect();
}

fn place_pixels(img: &mut RgbImage, pixel: Rgb<u8>, scale: u32, x: u32, y: u32) {
  for x_offset in 0..scale {
    for y_offset in 0..scale {
      img.put_pixel(x + x_offset, y + y_offset, pixel);
    }
  }
}

fn crystal<F: Fn(f64) -> Rgb<u8>>(
  colorize: F,
  phase: f64,
  order: i64,
  scale: u32,
  x_max: u32,
  y_max: u32) -> RgbImage {
    let angs = angles(order);
    let mut imgbuf = RgbImage::new(x_max, y_max);
    for x in 0..(x_max / scale) {
      for y in 0..(y_max / scale) {
        let part_wave = |rot: &f64| wave(*rot, phase, x, y);
        let waves = angs.iter().map(part_wave);
        let stacked = combine(waves.collect());
        let clamped = (1 as f64).min(stacked.max(0 as f64));
        let shade = clamped * 255.0;
        let pixel = colorize(shade);
        place_pixels(&mut imgbuf, pixel, scale, x * scale, y * scale);
      }
    }
  return imgbuf;
}

fn sawtooth(n: i64, max: i64, offset: f64, step: i64) -> f64 {
  let pi = std::f64::consts::PI;
  let adjusted_n = offset + (n as f64 / max as f64);
  let npi = adjusted_n * pi;
  return (step as f64 * pi * (npi.sin()).asin()).abs();
}

fn saw_colorize(shade: f64, frame: i64, frames: i64) -> Rgb<u8> {
  let color = |off| (shade - sawtooth(frame, frames, off, 51)).abs() as u8;
  let r = color(0.0);
  let g = color(0.25);
  let b = color(0.5);
  return Rgb([r, g, b]);
}

fn main() {
  let frames = 10;
  let colorize = |s| saw_colorize(s, 1, frames);
  // let colorize = |s| Rgb([s as u8, s as u8, s as u8]);
  let frame = crystal(colorize, 1.0, 6, 3, 640, 360);
  frame.save("test.png").unwrap();
}