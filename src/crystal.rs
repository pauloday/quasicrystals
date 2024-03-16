extern crate image;
use crate::Image;
use image::RgbaImage;
use std::f64::consts::TAU;

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

fn scaled_point(scale: u32, size: u32, point: u32) -> f64 {
    return scale as f64 * ((2.0 * point as f64 / (size as f64 - 1.0)) - 1.0);
}

fn frame_phase(frame: u32, frames: u32, phases: f64) -> f64 {
    return (TAU / frames as f64) * frame as f64 * phases;
}

pub fn gen(f: &Image) -> RgbaImage {
    let mut imgbuf = RgbaImage::new(f.width, f.height);
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let max_dim = f.width.max(f.height);
        let scaled_x = scaled_point(f.scale, max_dim, x + f.x_offset);
        let scaled_y = scaled_point(f.scale, max_dim, y + f.y_offset);
        let phase = frame_phase(f.frame, f.frames, f.phases);
        let part_wave = |rot: &f64| wave(*rot, phase, scaled_x, scaled_y);
        let waves = f.angles.iter().map(part_wave);
        let stacked = combine(waves.collect());
        let clamped = (1 as f64).min(stacked.max(0 as f64));
        let shade = clamped; // * 255.0;
        *pixel = f.colorizer.colorize(shade, f.frame, f.frames);
    }
    return imgbuf;
}
