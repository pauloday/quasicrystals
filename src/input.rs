// all the code for parsing a command line options object into frame/colorizers
use crate::color::{Colorizer, Greyscale, Sawtooth};
use crate::crystal::{percent_angles, proportion_angles};
use crate::output::Image;
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
    #[clap(about = "Proportional angles between waves (e.g. 1,2 -> 0°, 240°).\
        With the -p flag, the angles will be percents (either 50 or 1/2).
        If only one number is given it will use that many waves evenly rotated.\
        You can also add animation to them like '0-5-10`,10-5-0';\
        this will make an animation where the angles smoothly go from 0,100 to 50,50 then 100,0")]
    pub angles: String,
    #[clap(about = "Scaling factor, lower is more zoomed in")]
    pub scale: u32,
    #[clap(short, long, long_about = r"Colorizer, one of:
- 'greyscale <brighness>'
greyscale colors, brightness 0 will make it all black and 255 will make it all white. 127 is neutral
- 'sawtooth <r>,<g>,<b>,<s>'
use sawtooth waves to map a shade to color values.\
r,g,b are the offsets for the wave for that color from 0-0.5 (after 0.5 it loops).\
s is a saturation factor", min_values = 2, default_values = &["sawtooth", "0,0.25,0.5,51"])]
    pub colorizer: Vec<String>,
    #[clap(
        short,
        long,
        about = "Number of frames to generate",
        default_value = "1"
    )]
    pub frames: u32,
    #[clap(
        short,
        long,
        about = "Output path, if gif is used this must be called with the filename",
        default_value = "./"
    )]
    pub output: String,
    #[clap(short, long, about = "Ouput format", default_value = "jpg")]
    pub image_format: String,
    #[clap(short, long, about = "X viewport offset", default_value = "0")]
    pub x_offset: u32,
    #[clap(short, long, about = "Y viewport offset", default_value = "0")]
    pub y_offset: u32,
    #[clap(short, long, about = "Number of threads to use", default_value = "1")]
    pub threads: u32,
    #[clap(short, long, about = "Treat angles as percents (i.e. 0-100). Fractions can also be used (i.e. 1/6)")]
    pub percent: bool,
    #[clap(short, long, about = "Number of phases waves go through", default_value = "1")]
    pub speed: String,
}

fn parse_list<T>(params: &String, sep: char) -> Vec<T>
where
    T: FromStr,
    T::Err: std::fmt::Debug,
{
    let param_strings: Vec<&str> = (*params.split(sep).collect::<Vec<&str>>()).to_vec();
    return param_strings
        .iter()
        .map(|s| s.parse::<T>().unwrap())
        .collect();
}

fn parse_number(num_string: &String) -> f64 {
    if num_string.contains("/") {
        let parts: Vec<f64> = num_string
            .split("/")
            .map(|s| s.parse::<f64>().unwrap())
            .collect();
        return (parts[0] / parts[1]) * 100.0;
    } else {
        return num_string.parse::<f64>().unwrap();
    }
}

fn parse_animation(string: &String, frame: u32, frames: u32) -> f64 {
    let stages: Vec<f64> = parse_list::<String>(string, '-')
        .iter()
        .map(|s| parse_number(s))
        .collect();
    if string.contains("-") {
        // first we find # frames per stage transition
        let trans_frames: f64 = frames as f64 / (stages.len() as f64 - 1.0);
        // then we find the stage we're transitioning from
        let from_stage = stages
            .get((frame as f64 / trans_frames).floor() as usize)
            .unwrap();
        // and the one we're going to
        let to_stage = stages
            .get((frame as f64 / trans_frames).ceil() as usize)
            .unwrap();
        // then we figure out what percent through the transition we are
        let trans_prog = (frame as f64 % trans_frames) / trans_frames;
        // finally we can calculate the transitionary value
        let ret = (trans_prog * (to_stage - from_stage)) + from_stage;
        return ret;
    }
    return stages[0];
}

fn parse_angles(angles_string: &String, frame: u32, frames: u32, percent: bool) -> Vec<f64> {
    let angles: Vec<f64> = parse_list::<String>(angles_string, ',')
        .iter()
        .map(|a| parse_animation(&a, frame, frames))
        .collect();
    if percent {
        return percent_angles(angles);
    } else if angles.len() == 1 {
        return proportion_angles(vec![1.0; angles[0] as usize]);
    } else {
        return proportion_angles(angles);
    }
}

pub fn parse_image(opts: &Opts, frame: u32) -> Image {
    return Image {
        phases: parse_animation(&opts.speed, frame, opts.frames),
        frame: frame,
        frames: opts.frames,
        scale: opts.scale,
        width: opts.width,
        height: opts.height,
        x_offset: opts.x_offset,
        y_offset: opts.y_offset,
        angles: parse_angles(&opts.angles, frame, opts.frames, opts.percent),
    };
}

pub fn get_colorizer(opts: &Opts) -> Box<dyn Colorizer> {
    let colorizer = &opts.colorizer[0];
    let params = parse_list::<f64>(&opts.colorizer[1], ',');
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
