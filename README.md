# Quasicrystals

![Animation](https://github.com/ertdfgcb/quasicrystals/blob/master/crystal.gif)

Longer, larger animation [here](https://www.youtube.com/watch?v=80SDg1xT0sE).

A program that animates quasicrystals on a plane as seen [here](http://mainisusuallyafunction.blogspot.com/2011/10/quasicrystals-as-sums-of-waves-in-plane.html). Generates them as frames of a animation, that can be combined into a gif image or whatever later. I would recomend Imagemagick for gifs and ffmpeg for videos.

I added colorization by mapping the shade for each pixel to a sawtooth wave for each channel. The waves are shifted so each channel is at a different point of the wave for each frame. The wave goes through one full period over the course of a whole animation.

There's a (slow, blocky) Clojure version in the Clojure branch. I might update it eventually but for now I'm focusing on speeding up the Rust implementation.

## Usage
First, compile it:

`cargo build --release`

Then run it (this will generate the frames for the included gif example):

`./target/release/quasicrystals 200 200 30 6 32`

Pass --help for usage documentation:
```
$ quasicrystals --help

quasicrystals 1.0
Paul O'Day <https://github.com/ertdfgcb/quasicrystals>

USAGE:
    quasicrystals <width> <height> <frames> <order> <scale> [ARGS]

ARGS:
    <width>     Image width in pixels
    <height>    Image height in pixels
    <frames>    Number of frames to generate
    <order>     Number of rotated waves to stack
    <scale>     Scaling factor, lower is more zoomed in
    <output>    Output path [default: ./]
    <format>    Ouput format [default: jpg]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
```