# Quasicrystals

![Animation](https://github.com/ertdfgcb/quasicrystals/blob/master/crystal.gif)

Longer, larger animation [here](https://www.youtube.com/watch?v=80SDg1xT0sE).

A program that animates quasicrystals animations. I first saw [this](http://mainisusuallyafunction.blogspot.com/2011/10/quasicrystals-as-sums-of-waves-in-plane.html) article in 2011 and I thought it was super cool, but had some opportunities to make it more colorful. I was also learning Clojure, so I figured I'd rewrite it and add some color. My initial implementation was super slow, didn't scale right (the images looked pixellated), and didn't have the type of colorization I was looking for. But still I was happy with it and figured I'd come back to it at some point, eventually just forgetting about it. 9 years later I've come back and added the colorization I was after, rewritten it in Rust (for a *massive* speed increase), and now I'm looking into adjusting some other parameters to add more variation/color options.

By default it'll output frames that you can convert into a video with ffmpeg. If you want a gif version you can set the format to gif, but you also have to set the output path to a filename.

I added colorization by mapping the shade for each pixel to a sawtooth wave for each channel. The waves are shifted so each channel is at a different point of the wave for each frame. The colorization wave goes through one full period over the course of a whole animation. I'd like to add some tuning so the specific colors can be adjusted, and some other waveforms (issue #10). If you have any cool ideas about how to convert a single byte into a color, you can add it to `color.rs`. I welcome pull requests! Or just message me, I'd probably be down to implement it myself.

The old, slow, blocky Clojure version from 2011 is in the Clojure branch. I might update it eventually but for now I'm focusing on the Rust implementation.

## Usage
First, compile it:

`cargo build --release`

Then run it (this will generate the frames for the included gif example):

`./target/release/quasicrystals 200 200 7 32 -f 30 -o ./crystal.gif`

Pass --help for full usage:
```
$ quasicrystals --help
Usage: quasicrystals [OPTIONS] <WIDTH> <HEIGHT> <ANGLES> <SCALE>

Arguments:
  <WIDTH>
          Image width in pixels

  <HEIGHT>
          Image height in pixels

  <ANGLES>
          Number of layers defined by the angle of each layer. If you pass a single number, it'll use that many layers evenly rotated around a circle. Portional angles between waves can also be passed as a comma separated list (e.g. 1,2 -> 0°, 240°). With the -p flag, the angles will be percents (either 50 or 1/2). You can also add animation to them by passing bar separated numbers in place of an angle, e.g. '0|5|10,10|5|0'. This will make an animation where the angles smoothly go from 0,100 to 50,50 then 100,0.

  <SCALE>
          Scaling factor, lower is more zoomed in

Options:
  -c, --colorizer <COLORIZER> <COLORIZER>
          Colorizer, one of:
            composite <r shade range>/<r color range>,<g shade range>/<g color range>,<b shade range>/<b color range>
              Color ranges optional, map a shade range to a color range
            greyscale <brighness>
              brightness 0 is all black and 255 is all white. 127 is neutral
            sawtooth <r>,<g>,<b>,<s>
              use sawtooth waves to map a shade to color values, r,g,b are the offsets for the wave for that color from 0-0.5, s is a saturation factor
          
          [default: composite 0-0.4,0.3-0.7,0.6-1]

  -f, --frames <FRAMES>
          Number of frames to generate
          
          [default: 1]

  -o, --output <OUTPUT>
          Output path, if gif is used this must be called with the filename
          
          [default: ./]

  -a, --allow-overwrite
          Allow overwriting of existing images

  -i, --image-format <IMAGE_FORMAT>
          Ouput format, if gif is used you must pass an argument to output for the filename of the gif. Or if you pass a path ending in .gif to the output with no format, it will output a gif.
          
          [default: jpg]

  -x, --x-offset <X_OFFSET>
          X viewport offset
          
          [default: 0]

  -y, --y-offset <Y_OFFSET>
          Y viewport offset
          
          [default: 0]

  -t, --threads <THREADS>
          Number of threads to use
          
          [default: 1]

  -p, --percent
          Treat angles as percents (i.e. 0-100). Fractions can also be used (i.e. 1/6)

  -s, --speed <SPEED>
          Number of phases waves go through, more phases means the waves move faster, 0 means no movement. But the image can still be animated by passing '-' separated numbers to the angle argument
          
          [default: 1]

  -v, --verbose
          Print additional information while generating

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```
