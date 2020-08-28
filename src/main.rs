use std::str::FromStr;
use std::path::PathBuf;
use std::thread;
use clap::{Clap, ArgGroup};
mod crystal;
mod color;

#[derive(Clap)]
#[clap(version = "1.2", author = "Paul O'Day <https://github.com/ertdfgcb/quasicrystals>")]
#[clap(group = ArgGroup::new("colorizer").required(true))]
struct Opts {
  #[clap(about = "Image width in pixels")]
  width: u32,
  #[clap(about = "Image height in pixels")]
  height: u32,
  #[clap(about = "Proportional angles between waves (e.g. 1,2 -> 0°, 240°). If only one number is given it will use that many waves evenly rotated")]
  angles: String,
  #[clap(about = "Scaling factor, lower is more zoomed in")]
  scale: u32,
  #[clap(long, about = "Greyscale colorization, takes a brightness (0 = all black, 255 = all white)", group = "colorizer")]
  grey: Option<String>,
  #[clap(long, about = "Sawtooth colorization, takes r,g,b,s where rgb = offset on saw wave and s = step (e.g. 0,0.25,0.5,51)", group = "colorizer")]
  saw: Option<String>,
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

fn parse_csl<T>(params: &String) -> Vec<T>
where
    T: FromStr,
    T::Err: std::fmt::Debug,
{
  let param_strings: Vec<&str> = (*params.split(",").collect::<Vec<&str>>()).to_vec();
  return param_strings.iter().map(|s| s.parse::<T>().unwrap()).collect();
}

fn parse_angles(angles_string: &String) -> Vec<f64>{
  let angles = parse_csl::<u32>(angles_string);
  if angles.len() == 1 {
    return crystal::custom_angles(vec![1; angles[0] as usize]);
  }
  return crystal::custom_angles(angles);
}

fn frame_phase(frame: u32, frames: u32) -> f64{
  let pi = std::f64::consts::PI;
  return ((2.0 * pi) / frames as f64) * frame as f64;
}

fn get_colorizer() -> (color::Colorizer, String) {
  let opts: Opts = Opts::parse();
  if opts.saw != None {
    return (color::Colorizer::Saw, opts.saw.unwrap_or("".to_string()));
  } else {
    return (color::Colorizer::Greyscale, opts.grey.unwrap_or("".to_string()));
  }
}

fn write_frames_thread(start: u32, end: u32, thread_num: u32) -> thread::JoinHandle<()> {
  return thread::spawn(move || {
    println!("spawned thread {} to write frames {} to {}", thread_num, start, end);
    let opts: Opts = Opts::parse();
    let frames = opts.frames;
    let angles: Vec<f64> = parse_angles(&opts.angles);
    let crystal_params: crystal::Params = crystal::Params {
      scale: opts.scale,
      height: opts.height,
      width: opts.width,
      x_offset: opts.x_offset,
      y_offset: opts.y_offset
    };
    let (colorizer, params) = get_colorizer();
    let colorize_params = parse_csl(&params);
    for frame in start..end {
      let colorize = |s| color::colorize(&colorizer, s, frame, frames, &colorize_params);
      let phase = frame_phase(frame, frames);
      let frame_image = crystal::gen(colorize, phase, &angles, &crystal_params);
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
  println!("Writing {} frames at {}x{} with angles {} and scale {}",
    opts.frames, opts.width, opts.height, opts.angles, opts.scale);
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