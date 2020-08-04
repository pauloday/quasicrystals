use std::path::PathBuf;
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
  y_offset: u32
}

fn frame_phase(frame: u32, frames: u32) -> f64{
  let pi = std::f64::consts::PI;
  return ((2.0 * pi) / frames as f64) * frame as f64;
}

fn main() {
  let opts: Opts = Opts::parse();
  let frames = opts.frames;
  let crystal_params = crystal::Params {
    order: opts.order,
    scale: opts.scale,
    width: opts.width,
    height: opts.height,
    x_offset: opts.x_offset,
    y_offset: opts.y_offset
  };
  println!("Writing {} frames at {}x{} with order {} and scale {}",
    opts.frames, opts.width, opts.height, opts.order, opts.scale);
  for frame in 0..frames {
    let colorize = |s| color::saw_colorize(s, frame, frames);
    let phase = frame_phase(frame, frames);
    let frame_image = crystal::gen(colorize, phase, &crystal_params);
    let file_name = format!("{:06}.{}", frame, &opts.image_format);
    let path: PathBuf = [&opts.output, &file_name].iter().collect();
    frame_image.save(&path).unwrap();
    println!("wrote image {:?}", &path);
  }
}