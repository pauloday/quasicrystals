use image::Rgb;

pub trait Colorizer {
    fn colorize(&self, shade: f64, frame: u32, frames: u32) -> Rgb<u8>;
}

pub struct Sawtooth {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
    pub scalar: f64,
}

impl Colorizer for Sawtooth {
    fn colorize(&self, shade: f64, frame: u32, frames: u32) -> Rgb<u8> {
        let c = |offset: f64| {
            let pi = std::f64::consts::PI;
            let adjusted_frame = offset + (frame as f64 / frames as f64);
            let frame_pi = adjusted_frame * pi;
            let sawtooth = (self.scalar * pi * (frame_pi.sin()).asin()).abs();
            return (shade - sawtooth).abs() as u8;
        };
        let rgb: [u8; 3] = [c(self.red), c(self.green), c(self.blue)];
        return Rgb(rgb);
    }
}

pub struct Greyscale {
    pub brightness: f64,
}

impl Colorizer for Greyscale {
    fn colorize(&self, shade: f64, _: u32, _: u32) -> Rgb<u8> {
        let s = ((self.brightness * 2.0) - 255.0 + shade) as u8;
        return Rgb([s, s, s]);
    }
}
