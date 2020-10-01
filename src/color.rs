use image::Rgba;

pub trait Colorizer {
    fn colorize(&self, shade: f64, frame: u32, frames: u32) -> Rgba<u8>;
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
            let pi = std::f64::consts::PI;
            let adjusted_frame = offset + (frame as f64 / frames as f64);
            let frame_pi = adjusted_frame * pi;
            let sawtooth = (self.scalar * pi * (frame_pi.sin()).asin()).abs();
            return (shade - sawtooth).abs() as u8;
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
        let s = ((self.brightness * 2.0) - 255.0 + shade) as u8;
        return Rgba([s, s, s, 1]);
    }
}

// map shade to a space between color1 and color2
pub struct TwoTone {
    pub red1: f64,
    pub green1: f64,
    pub blue1: f64,
    pub red2: f64,
    pub green2: f64,
    pub blue2: f64,
}

impl Colorizer for TwoTone {
    fn colorize(&self, shade: f64, _: u32, _: u32) -> Rgba<u8>{
        // use shade as a percent between r1 and r2, g1 and g2, b1 and b2
        let mapping = |c1, c2| {
            let space = c2 - c1;
            let dist = space * (shade / 255.0);
            return (c1 + dist) as u8;
        };
        let red = mapping(self.red1, self.red2);
        let green = mapping(self.green1, self.green2);
        let blue = mapping(self.blue1, self.blue2);
        // println!("{},{}", shade / 255.0, red);
        return Rgba([red, green, blue, 1]);
    }
}