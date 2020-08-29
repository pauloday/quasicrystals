/*
this defines a frame without colorization
every parameter needed for generating one should be included
the colorizer will take this object as well
it's kind of an orphan right now, I can't think of anything else to put in this module
*/
pub struct Frame {
    pub frame: u32,
    pub frames: u32,
    pub scale: u32,
    pub width: u32,
    pub height: u32,
    pub x_offset: u32,
    pub y_offset: u32,
    pub angles: Vec<f64>,
}
