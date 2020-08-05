use std::path::PathBuf;
use std::thread;
use clap::Clap;
mod crystal;
mod color;

#[derive(Clap)]
#[clap(version = "1.0", author = "Paul O'Day <https://github.com/ertdfgcb/quasicrystals>")]
struct Opts {
  #[clap(about = "Image width in pixels")]
  width: u32,
  #[clap(about = "Image height in pixels")]
  height: u32,
  #[clap(about = "Number of rotated waves to stack")]
  order: u32,
  #[clap(about = "Scaling factor, lower is more zoomed in")]
  scale: u32,
  #[clap(short, long, about = "Number of frames to generate", default_value = "1")]
  frames: u32,
  #[clap(short, long, about = "Output path", default_value = "./")]
  output: String,
  #[clap(short, long, about = "Ouput format", default_value = "jpg")]
  image_format: String,
  #[clap(short, long, about = "X viewport offset", default_value = "0")]
  x_offset: u32,
  #[clap(short, long, about = "Y viewport offset", default_value = "0")]
  y_offset: u32,
  #[clap(short, long, about = "Number of threads to use", default_value = "1")]
  threads: u32
}

fn frame_phase(frame: u32, frames: u32) -> f64{
  let pi = std::f64::consts::PI;
  return ((2.0 * pi) / frames as f64) * frame as f64;
}

fn write_frames_thread(start: u32, end: u32, thread_num: u32) -> thread::JoinHandle<()> {
  return thread::spawn(move || {
    println!("spawned thread {} to write frames {} to {}", thread_num, start, end);
    let opts: Opts = Opts::parse();
    let frames = opts.frames;
    let crystal_params: crystal::Params = crystal::Params {
      order: opts.order,
      scale: opts.scale,
      width: opts.width,
      height: opts.height,
      x_offset: opts.x_offset,
      y_offset: opts.y_offset
    };
    for frame in start..end {
      let colorize = |s| color::saw_colorize(s, frame, frames);
      let phase = frame_phase(frame, frames);
      let frame_image = crystal::gen(colorize, phase, &crystal_params);
      let file_name = format!("{:06}.{}", frame, opts.image_format);
      let path: PathBuf = [&opts.output, &file_name].iter().collect();
      frame_image.save(&path).unwrap();
      println!("thread {}: wrote image {:?}", thread_num, &path);
    }
  });
}


fn main() {
  let opts: Opts = Opts::parse();
  let frames = opts.frames;
  println!("Writing {} frames at {}x{} with order {} and scale {}",
    opts.frames, opts.width, opts.height, opts.order, opts.scale);
  if frames < opts.threads {
    let frame_thread = write_frames_thread(0, frames, 1);
    frame_thread.join().unwrap();
  } else {
    let mut threads: Vec<thread::JoinHandle<_>> = Vec::new();
    for thread_num in 0..opts.threads { 
      let frames_chunk = (frames as f32/ opts.threads as f32).ceil() as u32;
      let start = frames_chunk * thread_num;
      let end = frames_chunk * (thread_num + 1);
      if thread_num == opts.threads - 1 {
        // on the last thread we use frames as end, to account for rounding
        threads.push(write_frames_thread(start, frames, thread_num));
      } else {
        threads.push(write_frames_thread(start, end, thread_num));
      }
    }
    for thread in threads {
      thread.join().unwrap();
    }
  }
}