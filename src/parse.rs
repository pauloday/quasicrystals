use std::str::FromStr;
use crate::Image;
use crate::input::Opts;
use crate::color::Colorizer;
use crate::color::Greyscale;
use crate::color::Sawtooth;

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

// takes a vec of proportions and returns a vec of angles for those sizes
// e.g. [1, 1] is 0, pi/2
// [1, 2] is 0, 2pi/3
// [1, 2, 1] is 0, 2pi/4, 3pi/4
pub fn proportion_angles(props: Vec<f64>) -> Vec<f64> {
    let pi = std::f64::consts::PI;
    let sections: f64 = props.iter().sum();
    let prop_to_angle = |angles: Vec<f64>, prop: &f64| {
        let angle = *prop as f64 * (pi / sections);
        let mut new_angles = angles.clone();
        match angles.last() {
            Some(a) => new_angles.push(angle + a),
            None => new_angles.push(angle),
        }
        return new_angles;
    };
    return props.iter().fold([].to_vec(), prop_to_angle);
}

pub fn percent_angles(props: Vec<f64>) -> Vec<f64> {
    let pi = std::f64::consts::PI;
    return props.iter().map(|p| p / 100 as f64 * pi).collect();
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
