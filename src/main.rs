use clap::Clap;
use std::path::PathBuf;
use std::thread;
mod color;
mod crystal;
mod frame;
mod input;

fn write_frames_thread(start: u32, end: u32, thread_num: u32) -> thread::JoinHandle<()> {
    return thread::spawn(move || {
        println!(
            "Spawned thread {} to write frames {} to {}",
            thread_num, start, end
        );
        let opts: input::Opts = input::Opts::parse();
        let colorizer = input::get_colorizer(&opts);
        for f in start..end {
            let frame = input::get_frame(&opts, f);
            let file_name = format!("{:06}.{}", frame.frame, opts.image_format);
            let path: PathBuf = [&opts.output, &file_name].iter().collect();
            let frame_image = crystal::gen(&colorizer, &frame);
            frame_image.save(&path).unwrap();
            println!("thread {}: wrote image {:?}", thread_num, &path);
        }
    });
}

fn main() {
    let opts: input::Opts = input::Opts::parse();
    let frames = opts.frames;
    println!(
        "Writing {} frames at {}x{} with angles {} and scale {}",
        opts.frames, opts.width, opts.height, opts.angles, opts.scale
    );
    if frames < opts.threads {
        let frame_thread = write_frames_thread(0, frames, 1);
        frame_thread.join().unwrap();
    } else {
        let mut threads: Vec<thread::JoinHandle<_>> = Vec::new();
        for thread_num in 0..opts.threads {
            let frames_chunk = (frames as f32 / opts.threads as f32).ceil() as u32;
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
