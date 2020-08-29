// all the code for parsing a command line options object into frame/colorizers
use crate::color::{Colorizer, Greyscale, Sawtooth};
use crate::crystal::custom_angles;
use crate::frame::Frame;
use clap::Clap;
use std::str::FromStr;

#[derive(Clap)]
#[clap(
    version = "1.3",
    author = "Paul O'Day <https://github.com/ertdfgcb/quasicrystals>"
)]
pub struct Opts {
    #[clap(about = "Image width in pixels")]
    pub width: u32,
    #[clap(about = "Image height in pixels")]
    pub height: u32,
    #[clap(
        about = "Proportional angles between waves (e.g. 1,2 -> 0°, 240°). If only one number is given it will use that many waves evenly rotated"
    )]
    pub angles: String,
    #[clap(about = "Scaling factor, lower is more zoomed in")]
    pub scale: u32,
    #[clap(short, long, about = r"Colorizer, one of:
- 'greyscale <brighness>'
greyscale colors, brightness 0 will make it all black and 255 will make it all white. 127 is neutral
- 'sawtooth <r>,<g>,<b>,<s>'
use sawtooth waves to map a shade to color values. r,g,b are the offsets for the wave for that color from 0-0.5 (after 0.5 it loops). s is a saturation factor", min_values = 2, default_values = &["sawtooth", "0,0.25,0.5,51"])]
    pub colorizer: Vec<String>,
    #[clap(
        short,
        long,
        about = "Number of frames to generate",
        default_value = "1"
    )]
    pub frames: u32,
    #[clap(short, long, about = "Output path", default_value = "./")]
    pub output: String,
    #[clap(short, long, about = "Ouput format", default_value = "jpg")]
    pub image_format: String,
    #[clap(short, long, about = "X viewport offset", default_value = "0")]
    pub x_offset: u32,
    #[clap(short, long, about = "Y viewport offset", default_value = "0")]
    pub y_offset: u32,
    #[clap(short, long, about = "Number of threads to use", default_value = "1")]
    pub threads: u32,
}

fn parse_csl<T>(params: &String) -> Vec<T>
where
    T: FromStr,
    T::Err: std::fmt::Debug,
{
    let param_strings: Vec<&str> = (*params.split(",").collect::<Vec<&str>>()).to_vec();
    return param_strings
        .iter()
        .map(|s| s.parse::<T>().unwrap())
        .collect();
}

fn parse_angles(angles_string: &String) -> Vec<f64> {
    let angles = parse_csl::<u32>(angles_string);
    if angles.len() == 1 {
        return custom_angles(vec![1; angles[0] as usize]);
    }
    return custom_angles(angles);
}

pub fn get_frame(opts: &Opts, frame: u32) -> Frame {
    return Frame {
        frame: frame,
        frames: opts.frames,
        scale: opts.scale,
        width: opts.width,
        height: opts.height,
        x_offset: opts.x_offset,
        y_offset: opts.y_offset,
        angles: parse_angles(&opts.angles),
    };
}

pub fn get_colorizer(opts: &Opts) -> Box<dyn Colorizer> {
    let colorizer = &opts.colorizer[0];
    let params = parse_csl::<f64>(&opts.colorizer[1]);
    match &colorizer[..] {
        "sawtooth" => {
            return Box::new(Sawtooth {
                red: params[0],
                green: params[1],
                blue: params[2],
                scalar: params[3],
            })
        }
        "greyscale" => {
            return Box::new(Greyscale {
                brightness: params[0],
            })
        }
        _ => {
            return Box::new(Sawtooth {
                red: 0.0,
                green: 0.25,
                blue: 0.5,
                scalar: 51.0,
            })
        }
    }
}
