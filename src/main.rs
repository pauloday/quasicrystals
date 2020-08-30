use clap::Clap;
use std::thread;
mod color;
mod crystal;
mod output;
mod input;

fn main() {
    let opts: input::Opts = input::Opts::parse();
    let frames = opts.frames;
    println!(
        "Writing {} frames at {}x{} with angles {} and scale {}",
        opts.frames, opts.width, opts.height, opts.angles, opts.scale
    );
    if frames < opts.threads {
        let frame_thread = output::write_frames_thread(0, frames, 1);
        frame_thread.join().unwrap();
    } else {
        let mut threads: Vec<thread::JoinHandle<_>> = Vec::new();
        for thread_num in 0..opts.threads {
            let frames_chunk = (frames as f32 / opts.threads as f32).ceil() as u32;
            let start = frames_chunk * thread_num;
            let end = frames_chunk * (thread_num + 1);
            if thread_num == opts.threads - 1 {
                // on the last thread we use frames as end, to account for rounding
                threads.push(output::write_frames_thread(start, frames, thread_num));
            } else {
                threads.push(output::write_frames_thread(start, end, thread_num));
            }
        }
        for thread in threads {
            thread.join().unwrap();
        }
    }
}
