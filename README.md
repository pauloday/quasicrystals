# Quasicrystals

![Animation](https://github.com/ertdfgcb/quasicrystals/blob/master/animation.gif)

A program that animates quasicrystals on a plane as seen [here](http://mainisusuallyafunction.blogspot.com/2011/10/quasicrystals-as-sums-of-waves-in-plane.html). Generates them as frames of a animation, that can be combined into a gif image or whatever later. I would recomend Imagemagick for gifs and ffmpeg for videos.

I added colorization by mapping the shade for each pixel to a sawtooth wave for each channel. The waves are shifted so each channel is at a different point of the wave for each frame. The wave goes through one full period over the course of a whole animation.

There's a (slow, blocky) Clojure version in the Clojure branch. I might update it eventually but for now I'm focusing on speeding up the Rust implementation.