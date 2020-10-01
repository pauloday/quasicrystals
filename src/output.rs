use crate::crystal;
use crate::input;
use clap::Clap;
use image::gif::Encoder;
use image::Frame;
use std::fs::File;
use std::path::PathBuf;
use std::thread;

/*
this defines a frame without colorization
every parameter needed for generating one should be included
the colorizer will take this object as well
*/
pub struct Image {
    pub phases: f64,
    pub frame: u32,
    pub frames: u32,
    pub scale: u32,
    pub width: u32,
    pub height: u32,
    pub x_offset: u32,
    pub y_offset: u32,
    pub angles: Vec<f64>,
}
/*j
fn write_gif(frames: Vec<Frame>, path: &String) -> ImageResult<()> {
    let file_out = File::open(path)?;
    let mut encoder = Encoder::new(file_out);
    return encoder.encode_frames(frames.into_iter());
}
*/

pub fn write_frames_thread(start: u32, end: u32, thread_num: u32) -> thread::JoinHandle<()> {
    return thread::spawn(move || {
        println!(
            "Spawned thread {} to write frames {} to {}",
            thread_num, start, end
        );
        let opts: input::Opts = input::Opts::parse();
        let colorizer = input::get_colorizer(&opts);
        let mut frames: Vec<Frame> = Vec::new();
        for f in start..end {
            let frame = input::parse_image(&opts, f);
            let frame_image = crystal::gen(&colorizer, &frame);
            if opts.image_format == "gif" {
                frames.push(Frame::new(frame_image));
            } else {
                let file_name = format!("{:06}.{}", frame.frame, opts.image_format);
                let path: PathBuf = [&opts.output, &file_name].iter().collect();
                frame_image.save(&path).unwrap();
            }
            println!("thread {}: rendered image {:?}", thread_num, f);
        }
        if opts.image_format == "gif" {
            println!("writing gif (very slowly)...");
            let file_out = match File::create(opts.output) {
                Ok(file) => file,
                Err(e) => panic!(e),
            };
            let mut encoder = Encoder::new(file_out);
            match encoder.encode_frames(frames.into_iter()) {
                Ok(_) => println!("wrote gif"),
                Err(e) => panic!(e),
            };
        }
    });
}
