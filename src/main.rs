use std::sync::mpsc::channel;
use std::thread;
use std::sync::mpsc::Sender;
use std::path::Path;
use image::RgbaImage;
use clap::Parser;
use crate::parse::{parse_image, parse_colorizer};
use crate::input::Opts;

mod color;
mod crystal;
mod input;
mod output;
mod parse;

/*
this defines a frame without colorization
every parameter needed for generating one should be included
the colorizer will take this object as well
*/
struct Image {
    pub phases: f64,
    pub frame: u32,
    pub frames: u32,
    pub scale: u32,
    pub width: u32,
    pub height: u32,
    pub x_offset: u32,
    pub y_offset: u32,
    pub angles: Vec<f64>,
}

/*
this defines a generated frame with the frame number
used for multithreading to keep the order of the frames independant of generation order
*/
struct OutFrame {
    pub frame: RgbaImage,
    pub number: u32,
}

fn gen_frames_thread(start: u32,
                     end: u32,
                     thread_num: u32,
                     sender: Sender<OutFrame>,
                     verbose: bool)
    -> thread::JoinHandle<()> {
    return thread::spawn(move || {
        if verbose {
            println!(
                "Spawned thread {} to write frames {} to {}",
                thread_num, start, end
            );
        }
        let opts: Opts = input::Opts::parse();
        let colorizer = parse_colorizer(&opts);
        for f in start..end {
            let frame = parse_image(&opts, f);
            let frame_image = crystal::gen(&colorizer, &frame);
            let out_frame = OutFrame {
                frame: frame_image,
                number: frame.frame,
            };
            sender.send(out_frame).unwrap();
            if verbose {
                println!("Rendered frame {} on thread {}", f, thread_num);
            }
        }
    });
}

fn spawn_thread(frames: u32,
                threads: u32,
                thread_num: u32,
                sender: Sender<OutFrame>,
                verbose: bool)
    -> thread::JoinHandle<()> {
    let frames_chunk = (frames as f32 / threads as f32).ceil() as u32;
    let start = frames_chunk * thread_num;
    let end = frames_chunk * (thread_num + 1);
    let sender_clone = sender.clone();
    if thread_num == threads - 1 {
        // on the last thread we use frames as end, to account for rounding when splitting frames
        return gen_frames_thread(
            start,
            frames,
            thread_num,
            sender_clone,
            verbose
        );
    } else {
        return gen_frames_thread(
            start,
            end,
            thread_num,
            sender_clone,
            verbose
        );
    }
}

fn main() {
    let opts: input::Opts = input::Opts::parse();
    let frames = opts.frames;
    let num_threads = if frames < opts.threads {
        frames
    } else {
        opts.threads
    };
    let (sender, receiver) = channel();
    let mut opt_frames: Vec<Option<RgbaImage>> = vec![None; frames as usize];
    if opts.image_format == "gif" { 
        if opts.output == "./" {
            println!("Must set output flag when writing gifs");
            return;
        }
        if Path::new(&opts.output).exists() {
            println!("File {} exists, exiting", opts.output);
            return;
        }
    }
    println!(
        "Generating {} frames at {}x{} with angles {} and scale {} using {} threads",
        frames, opts.width, opts.height, opts.angles, opts.scale, num_threads
    );
    if frames == 1 || num_threads == 1 {
        let frame_thread = gen_frames_thread(
            0,
            frames,
            1,
            sender.clone(),
            opts.verbose
        );
        frame_thread.join().unwrap();
    } else {
        let mut threads: Vec<thread::JoinHandle<_>> = Vec::new();
        for thread_num in 0..num_threads {
            threads.push(spawn_thread(
                    frames,
                    num_threads,
                    thread_num,
                    sender.clone(),
                    opts.verbose
            ));
        }
        for thread in threads {
            thread.join().unwrap();
        }
        while let Ok(result) = receiver.try_recv() {
            opt_frames[result.number as usize] = Some(result.frame);
        }
        if opts.image_format == "gif" || opts.output.ends_with(".gif") {
            output::write_gif(opts.output, opt_frames, opts.allow_overwrite);
        } else {
           output::write_frames(
               opts.image_format,
               opt_frames,
               opts.output.clone(),
               opts.verbose,
               opts.allow_overwrite
            );
        }
    }
}
