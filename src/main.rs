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
  #[clap(about = "Number of frames to generate")]
  frames: u32,
  #[clap(about = "Number of rotated waves to stack")]
  order: u32,
  #[clap(about = "Scaling factor, lower is more zoomed in")]
  scale: u32,
  #[clap(about = "Output path", default_value = "./")]
  output: String,
  #[clap(about = "Ouput format", default_value = "jpg")]
  format: String
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
    height: opts.height
  };
  for frame in 0..frames {
    let colorize = |s| color::saw_colorize(s, frame, frames);
    let phase = frame_phase(frame, frames);
    let frame_image = crystal::gen(colorize, phase, &crystal_params);
    let path = format!("{}/{:06}.{}", &opts.output, frame, &opts.format);
    frame_image.save(&path).unwrap();
    println!("wrote image {}", &path);
  }
}