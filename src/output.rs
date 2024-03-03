use image::codecs::gif::GifEncoder;
use image::Frame;
use image::RgbaImage;
use std::fs::File;
use std::path::Path;
use std::path::PathBuf;

pub fn write_gif(output: String, frames: Vec<Option<RgbaImage>>, allow_overwrite: bool) {
    let path_exists = Path::new(&output).exists();
    if path_exists && !allow_overwrite {
        println!("Path {} exists, use -a to allow overwriting", output);
    } else {
        let file_out = match File::create(output.clone()) {
            Ok(file) => file,
            Err(e) => std::panic::panic_any(e),
        };
        let mut encoder = GifEncoder::new_with_speed(file_out, 10);
        encoder
            .set_repeat(image::codecs::gif::Repeat::Infinite)
            .unwrap();
        match encoder.encode_frames(frames.into_iter().map(|f| Frame::new(f.unwrap()))) {
            Ok(_) => println!("Wrote {}", output.clone()),
            Err(e) => std::panic::panic_any(e),
        };
    }
}

pub fn write_frames(
    format: String,
    frames: Vec<Option<RgbaImage>>,
    out_path: String,
    verbose: bool,
    allow_overwrite: bool,
) {
    for (frame_num, frame) in frames.iter().enumerate() {
        let num_frames_digits = (frames.len().checked_ilog10().unwrap_or(0) + 1) as usize;
        let file_name = format!(
            "{:0width$}.{}",
            frame_num,
            format,
            width = num_frames_digits
        );
        let path: PathBuf = [out_path.clone(), file_name.clone()].iter().collect();
        let path_exists = Path::new(&path).exists();
        if path_exists && !allow_overwrite {
            println!(
                "Frame {} exists, skipping. Use -a to allow overwriting",
                file_name
            );
        } else {
            frame.as_ref().unwrap().save(&path).unwrap();
            if verbose {
                println!("Wrote frame {} to {}", frame_num, file_name.clone());
            }
        }
    }
}
