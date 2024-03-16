use crate::color::Colorizer;
use crate::color::Composite;
use crate::input::Opts;
use crate::Image;
use std::f64::consts::TAU;
use std::fmt::Debug;

use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammer.pest"]
pub struct InputParser;

fn parse_number(num_string: &str) -> f64 {
    let parsed = InputParser::parse(Rule::number, num_string).unwrap().as_str();
    if parsed.contains("/") {
        let parts: Vec<f64> = parsed
            .split("/")
            .map(|s| s.parse::<f64>().unwrap())
            .collect();
        return parts[0] / parts[1];
    } else {
        return parsed.parse::<f64>().unwrap();
    }
}

fn parse_map<T>(parse_str: &str, rule: Rule, parser: impl Fn(&str) -> T) -> Vec<T> {
    let parsed = InputParser::parse(rule, parse_str)
        .expect("parsing failed")
        .next().unwrap().into_inner();
    return parsed
        .map(|f| parser(f.as_str()))
        .collect::<Vec<T>>();
}

fn interpolate_animation(stages: Vec<f64>, frame: u32, frames: u32) -> f64 {
    if stages.len() == 1 {
        return stages[0];
    }
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
    return (trans_prog * (to_stage - from_stage)) + from_stage;
}

fn parse_anim(anim_str: &str, frame: u32, frames: u32) -> f64 {
    let stages = parse_map::<f64>(anim_str, Rule::animation, parse_number);
    return interpolate_animation(stages, frame, frames);
}

fn parse_row(row_str: &str, frame: u32, frames: u32) -> Vec<f64> {
    return parse_map::<f64>(row_str, Rule::row, |a| parse_anim(a, frame, frames));
}

fn parse_range(range_str: &str, frame: u32, frames: u32) -> Vec<f64> {
    return parse_map::<f64>(range_str, Rule::range, |a| parse_anim(a, frame, frames));
}

fn parse_mapping(map_str: &str, frame: u32, frames: u32) -> Vec<Vec<f64>> {
    return parse_map::<Vec<f64>>(map_str, Rule::mapping, |a| parse_range(a, frame, frames));
}

pub fn parse_colors(color_str: &str, frame: u32, frames: u32) -> Box<dyn Colorizer> {
    let colors = parse_map(color_str, Rule::colors, |a| parse_mapping(a, frame, frames));
    let mapping = |r: usize| {
        let in_range = (colors[r][0][0], colors[r][0][1]);
        let out_range = if colors[r].len() == 1 {
            (0u8, 255u8)
        } else {
            (colors[r][1][0] as u8, colors[r][1][1] as u8)
        };
        return (in_range, out_range);
    };
    return Box::new(Composite {
                red: mapping(0),
                green: mapping(1),
                blue: mapping(2),
            })
}

// to convert an angle of rotation for a proportion:
// - split circle into S even sections
// - for each prop, sum it with all the props before for # of S to the end of that prop
// - multiply # of S by 2pi for angle in rads
fn proportions_to_angles(props: Vec<f64>) -> Vec<f64> {
    let circle_sects = TAU / props.iter().sum::<f64>();
    let mut sums = vec![props[0]];
    for p in &props[1..] {
        let sum_to_p = *p + sums.last().unwrap();
        sums.push(sum_to_p);
    }
    return sums.iter().map(|s| s * circle_sects).collect();
}

fn parse_angles(angles_string: &String, frame: u32, frames: u32, percent: bool) -> Vec<f64> {
    // determine if this is an animated angle, a set of props/percents, or just n even angles
    if angles_string.parse::<u32>().is_ok() {
        // this is just a number that represents n even angles
        let n = angles_string.parse::<u32>().unwrap();
        return proportions_to_angles(vec![1.0; n as usize]);
    }
    // otherwise this is a list of proportions or percents, possibly animated
    let angles = parse_row(angles_string, frame, frames);
    if percent {
        return angles.iter().map(|p| (p / 100.0) * TAU).collect();
    } else {
        return proportions_to_angles(angles);
    }
}

pub fn parse_image(opts: &Opts, frame: u32) -> Image {
    return Image {
        phases: opts.speed,
        frame,
        frames: opts.frames,
        scale: opts.scale,
        width: opts.width,
        height: opts.height,
        x_offset: opts.x_offset,
        y_offset: opts.y_offset,
        angles: parse_angles(&opts.angles, frame, opts.frames, opts.percent),
    };
}
