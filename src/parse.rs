use std::str::FromStr;
use std::f64::consts::PI;
use crate::Image;
use crate::input::Opts;
use crate::color::Colorizer;
use crate::color::Greyscale;
use crate::color::Sawtooth;

fn parse_number(num_string: &str) -> f64 {
    if num_string.contains("/") {
        let parts: Vec<f64> = num_string
            .split("/")
            .map(|s| s.parse::<f64>().unwrap())
            .collect();
        return parts[0] / parts[1];
    } else {
        return num_string.parse::<f64>().unwrap() / 100.0;
    }
}

fn parse_percents(props: Vec<f64>) -> Vec<f64> {
    return props.iter().map(|p| p * PI).collect();
}

fn parse_proportions(props: Vec<f64>) -> Vec<f64> {
    let denominator: f64 = props.iter().sum();
    let percents = props.iter().map(|p| p / denominator).collect();
    return parse_percents(percents);
}

fn parse_animation(string: &str, frame: u32, frames: u32) -> f64 {
    let stages: Vec<f64> = (*string.split('-')
        .map(|s| parse_number(s))
        .collect::<Vec<f64>>()).to_vec();
    // if string with no dash is passed, we can skip the animation parsing
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
    let angles: Vec<f64> = (*angles_string.split(',')
        .map(|a| parse_animation(a, frame, frames))
        .collect::<Vec<f64>>()).to_vec();
    if percent {
        return parse_percents(angles);
    } else if angles.len() == 1 {
        return parse_proportions(vec![1.0; angles[0] as usize]);
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

pub fn parse_colorizer(opts: &Opts) -> Box<dyn Colorizer> {
    let colorizer = &opts.colorizer[0];
    let params = (&opts.colorizer[1].split(',')
                  .map(|s| s.parse::<f64>().unwrap())
                 .collect::<Vec<f64>>()).to_vec();
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
