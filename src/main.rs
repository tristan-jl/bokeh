use std::env;

use bokeh::{params::*, Blur};
use image::io::Reader as ImageReader;

fn main() {
    let mut args = env::args();
    args.next();
    let input_path = args.next().unwrap_or_else(|| "input.png".to_owned());
    let output_path = args.next().unwrap_or_else(|| "output.png".to_owned());

    let mut img = ImageReader::open(input_path).unwrap().decode().unwrap();
    img.bokeh_blur(2.0, 50, &KERNEL5_PARAM_SET);
    img.save(output_path).unwrap();
}
