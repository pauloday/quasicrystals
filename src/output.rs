use std::path::Path;
use std::fs::File;
use std::path::PathBuf;
use image::RgbaImage;
use image::codecs::gif::GifEncoder;
use image::Frame;


pub fn write_gif(output: String, frames: Vec<Option<RgbaImage>>) {
    if !Path::new(&output).exists() {
        println!("writing gif...");
        let file_out = match File::create(output) {
            Ok(file) => file,
            Err(e) => std::panic::panic_any(e),
        };
        let mut encoder = GifEncoder::new_with_speed(file_out, 10);
        encoder
            .set_repeat(image::codecs::gif::Repeat::Infinite)
            .unwrap();
        match encoder.encode_frames(frames.into_iter().map(|f| Frame::new(f.unwrap()))) {
            Ok(_) => println!("wrote gif"),
            Err(e) => std::panic::panic_any(e),
        };
    }
}

pub fn write_frames(format: String, frames: Vec<Option<RgbaImage>>, out_path: String) {
    for (frame_num, frame) in frames.iter().enumerate() {
        let file_name = format!("{:06}.{}", frame_num, format);
        let path: PathBuf = [out_path.clone(), file_name].iter().collect();
        frame.as_ref().unwrap().save(&path).unwrap();
    }
}
