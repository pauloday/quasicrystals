use image::Rgba;
use std::f64::consts::PI;

pub trait Colorizer {
    fn colorize(&self, shade: f64, frame: u32, frames: u32) -> Rgba<u8>;
}

pub struct Composite {
    pub red: ((f64, f64), (u8, u8)),
    pub green: ((f64, f64), (u8, u8)),
    pub blue: ((f64, f64), (u8, u8)),
}

pub fn map_range(from_range: (f64, f64), to_range: (u8, u8), s: f64) -> u8 {
    let mapped = to_range.1 as f64
        + ((s - from_range.0) * (to_range.1 - to_range.0) as f64 / (from_range.1 - from_range.0));
    return if s > from_range.1 { 0 } else { mapped as u8 };
}

impl Colorizer for Composite {
    fn colorize(&self, shade: f64, _: u32, _: u32) -> Rgba<u8> {
        let rgb: [u8; 4] = [
            map_range(self.red.0, self.red.1, shade),
            map_range(self.green.0, self.green.1, shade),
            map_range(self.blue.0, self.blue.1, shade),
            1,
        ];
        return Rgba(rgb);
    }
}

pub struct Sawtooth {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
    pub scalar: f64,
}

impl Colorizer for Sawtooth {
    fn colorize(&self, shade: f64, frame: u32, frames: u32) -> Rgba<u8> {
        let c = |offset: f64| {
            let adjusted_frame = offset + (frame as f64 / frames as f64);
            let frame_pi = adjusted_frame * PI;
            let sawtooth = (self.scalar * PI * (frame_pi.sin()).asin()).abs();
            return (shade * 255.0 - sawtooth).abs() as u8;
        };
        let rgb: [u8; 4] = [c(self.red), c(self.green), c(self.blue), 1];
        return Rgba(rgb);
    }
}

pub struct Greyscale {
    pub brightness: f64,
}

impl Colorizer for Greyscale {
    fn colorize(&self, shade: f64, _: u32, _: u32) -> Rgba<u8> {
        let s = ((self.brightness * 2.0) - 255.0 + shade * 255.0) as u8;
        return Rgba([s, s, s, 1]);
    }
}
