# Quasicrystals

![Animation](https://github.com/ertdfgcb/quasicrystals/blob/master/crystal.gif)

Longer, larger animation [here](https://www.youtube.com/watch?v=80SDg1xT0sE).

A program that animates quasicrystals animations. I first saw [this](http://mainisusuallyafunction.blogspot.com/2011/10/quasicrystals-as-sums-of-waves-in-plane.html) article in 2011 and I thought it was super cool, but had some opportunities to make it more colorful. I was also learning Clojure, so I figured I'd rewrite it and add some color. My initial implementation was super slow, didn't scale right (the images looked pixellated), and didn't have the type of colorization I was looking for. But still I was happy with it and figured I'd come back to it at some point, eventually just forgetting about it. 9 years later I've come back and added the colorization I was after, rewritten it in Rust (for a *massive* speed increase), and now I'm looking into adjusting some other parameters to add more variation/color options.

Right now it only generates frames, you have to convert them to a GIF (I use imagemagick) or video (I use ffmpeg) yourself. But directly generating the animations is planned (issue #13).

I added colorization by mapping the shade for each pixel to a sawtooth wave for each channel. The waves are shifted so each channel is at a different point of the wave for each frame. The colorization wave goes through one full period over the course of a whole animation. I'd like to add some tuning so the specific colors can be adjusted, and some other waveforms (issue #10). If you have any cool ideas about how to convert a single byte into a color, you can add it to `color.rs`. I welcome pull requests! Or just message me, I'd probably be down to implement it myself.

I'm also planning on adding settings to make things like zoom level, viewport offset, angle between/number of waves vary over the course of the animation (issue #14).

The old, slow, blocky Clojure version from 2011 is in the Clojure branch. I might update it eventually but for now I'm focusing on the Rust implementation.

## Usage
First, compile it:

`cargo build --release`

Then run it (this will generate the frames for the included gif example):

`./target/release/quasicrystals 200 200 6 32 -f 30`

Pass --help for full usage:
```
$ quasicrystals --help
quasicrystals 1.2
Paul O'Day <https://github.com/ertdfgcb/quasicrystals>

USAGE:
    quasicrystals [OPTIONS] <width> <height> <angles> <scale> <--grey <grey>|--saw <saw>>

ARGS:
    <width>     Image width in pixels
    <height>    Image height in pixels
    <angles>    Proportional angles between waves (e.g. 1,2 -> 0°, 240°). If only one number is given it will use
                that many waves evenly rotated
    <scale>     Scaling factor, lower is more zoomed in

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -f, --frames <frames>                Number of frames to generate [default: 1]
        --grey <grey>                    Greyscale colorization, takes a brightness (0 = all black, 1 = all white)
    -i, --image-format <image-format>    Ouput format [default: jpg]
    -o, --output <output>                Output path [default: ./]
        --saw <saw>                      Sawtooth colorization, takes r,g,b,s where rgb = offset on saw wave and s =
                                         step (e.g. 0,0.25,0.5,51)
    -t, --threads <threads>              Number of threads to use [default: 1]
    -x, --x-offset <x-offset>            X viewport offset [default: 0]
    -y, --y-offset <y-offset>            Y viewport offset [default: 0]
```
