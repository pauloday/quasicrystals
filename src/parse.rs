use std::f64::consts::PI;
use std::str::FromStr;
use std::fmt::Debug;
use crate::Image;
use crate::input::Opts;
use crate::color::Colorizer;
use crate::color::Greyscale;
use crate::color::Sawtooth;
use crate::color::Composite;

fn parse_sep<T: FromStr>(sep_str: &str, sep: char) -> Vec<T>
    where <T as FromStr>::Err: Debug
{
    return sep_str
        .split(sep)
        .map(|s| s.parse::<T>().unwrap())
        .collect();
}

fn parse_number(num_string: &str) -> f64 {
    if num_string.contains("/") {
        let parts: Vec<f64> = parse_sep::<f64>(num_string, '/');
        return parts[0] / parts[1];
    } else {
        return num_string.parse::<f64>().unwrap() / 100.0;
    }
}

// to convert an angle of rotation for a proportion:
// - split circle into S even sections
// - for each prop, sum it with all the props before for # of S to the end of that prop
// - multiply # of S by 2pi for angle in rads
fn parse_proportions(props: Vec<f64>) -> Vec<f64> {
    let circle_sects = PI / props.iter().sum::<f64>();
    let mut sums = vec![props[0]];
    for p in &props[1..] {
        let sum_to_p = *p + sums.last().unwrap();
        sums.push(sum_to_p);
    }
    let r = sums.iter().map(|s| s * circle_sects).collect();
    return r;
}

fn parse_angle(string: &str, frame: u32, frames: u32) -> f64 {
    let stages: Vec<f64> = (*string.split(':')
        .map(|s| parse_number(s))
        .collect::<Vec<f64>>()).to_vec();
    // if string with no dash is passed, we can skip the animation parsing
    if string.contains(":") {
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
    // determine if this is an animated angle, a set of props/percents, or just n even angles
    if angles_string.parse::<u32>().is_ok() {
        // this is just a number that represents n even angles
        let n = angles_string.parse::<u32>().unwrap();
        return parse_proportions(vec![1.0; n as usize]);
    } 
    // otherwise this is a list of proportions/percents, possibly animated
    let angles: Vec<f64> = (*angles_string.split(',')
        .map(|a| parse_angle(a, frame, frames))
        .collect::<Vec<f64>>()).to_vec();
    if percent {
        return angles.iter().map(|p| p * PI).collect();
    } else {
        return parse_proportions(angles);
    }
}

pub fn parse_image(opts: &Opts, frame: u32) -> Image {
    return Image {
        phases: opts.speed,
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

fn parse_composite_ranges(opts: &str) -> ((f64, f64), (u8, u8)) {
    if opts.contains("/") {
        let in_out_strs = opts
            .split('/')
            .collect::<Vec<&str>>();
        let in_vals = parse_sep::<f64>(in_out_strs[0], '-');
        let out_vals = parse_sep::<u8>(in_out_strs[1], '-');
        return ((in_vals[0], in_vals[1]), (out_vals[0], out_vals[1]));
    }
    let in_vals = parse_sep::<f64>(opts, '-');
    return ((in_vals[0], in_vals[1]), (0u8, 255u8));
}

pub fn parse_colorizer(opts: &Opts) -> Box<dyn Colorizer> {
    let colorizer = &opts.colorizer[0];
        match &colorizer[..] {
        "composite" => {
            let params = (&opts.colorizer[1].split(',')
                          .map(parse_composite_ranges)
                          .collect::<Vec<((f64, f64), (u8, u8))>>()).to_vec();
            return Box::new(Composite {
                red: params[0],
                green: params[1],
                blue: params[2],
            })

        }
        "sawtooth" => {
            let params = parse_sep::<f64>(&opts.colorizer[1], ',');
            return Box::new(Sawtooth {
                red: params[0],
                green: params[1],
                blue: params[2],
                scalar: params[3],
            })
        }
        "greyscale" => {
            let bright = &opts.colorizer[1].parse::<f64>().unwrap();
            return Box::new(Greyscale {
                brightness: *bright,
            })
        }
        _ => {
            return Box::new(Composite {
                red: ((0.0, 0.4), (0, 255)),
                green: ((0.3, 0.7), (0, 255)),
                blue: ((0.6, 1.0), (0, 255))
            })
        }
    }
}
