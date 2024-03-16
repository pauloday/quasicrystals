use clap::Parser;

// This might be cleaner if I used the "///" style to define the longer help strings
// But to me comments affecting the output program is hearsay and I won't have it
static ANGLE_HELP: &str = concat!(
    "Number of layers defined by the angle of each layer. ",
    "If you pass a single number, it'll use that many layers evenly rotated around a circle. ",
    "Portional angles between waves can also be passed as a comma separated list (e.g. 1,2 -> 0°, 240°). ",
    "With the -p flag, the angles will be percents (either 50 or 1/2). ",
    "You can also add animation to them by passing ':' separated numbers in place of an angle, e.g. '0:5:10,10:5:0'. ",
    "This will make an animation where the angles smoothly go from 0,100 to 50,50 then 100,0."
);

static COLOR_HELP: &str = concat!(
    "<r shade range>~<r color range>,<g shade range>~<g color range>,<b shade range>~<b color range>\n",
    "    Color ranges optional, map a shade range to a color range\n",
);

static FORMAT_HELP: &str = concat!(
    "Ouput format, if gif is used you must pass an argument to output for the filename of the gif. ",
    "Or if you pass a path ending in .gif to the output with no format, it will output a gif."
);

static PHASE_HELP: &str = concat!(
    "Number of phases waves go through, more phases means the waves move faster, 0 means no movement. ",
    "But the image can still be animated by passing '-' separated numbers to the angle argument"
);

#[derive(Parser, Debug)]
#[command(
    version = "1.8",
    author = "Paul O'Day <https://github.com/ertdfgcb/quasicrystals>"
)]
pub struct Opts {
    #[arg(help = "Image width in pixels")]
    pub width: u32,

    #[arg(help = "Image height in pixels")]
    pub height: u32,

    #[arg(help = ANGLE_HELP)]
    pub angles: String,

    #[arg(help = "Scaling factor, lower is more zoomed in")]
    pub scale: u32,

    #[arg(short, long, long_help = COLOR_HELP, default_value = "0_.4,.3_.7~0_255,.6_1~0_255")]
    pub colorizer: String,

    #[arg(
        short,
        long,
        help = "Number of frames to generate",
        default_value = "1"
    )]
    pub frames: u32,

    #[arg(
        short,
        long,
        help = "Output path, if gif is used this must be called with the filename",
        default_value = "./"
    )]
    pub output: String,

    #[arg(
        short,
        long,
        help = "Allow overwriting of existing images",
        default_value = "false"
    )]
    pub allow_overwrite: bool,

    #[arg(short, long, help = FORMAT_HELP, default_value = "jpg")]
    pub image_format: String,

    #[arg(short, long, help = "X viewport offset", default_value = "0")]
    pub x_offset: u32,

    #[arg(short, long, help = "Y viewport offset", default_value = "0")]
    pub y_offset: u32,

    #[arg(short, long, help = "Number of threads to use", default_value = "1")]
    pub threads: u32,

    #[arg(
        short,
        long,
        help = "Treat angles as percents (i.e. 0-100). Fractions can also be used (i.e. 1/6)"
    )]
    pub percent: bool,

    #[arg(short, long, help = PHASE_HELP, default_value = "1")]
    pub speed: String,

    #[arg(
        short,
        long,
        help = "Print additional information while generating",
        default_value = "false"
    )]
    pub verbose: bool,
}
