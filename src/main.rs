use bokeh::{params::*, Blur};
use image::io::Reader as ImageReader;
use std::env;

fn main() {
    let mut args = env::args();
    args.next();
    let input_path = args.next().unwrap_or_else(|| "input.png".to_owned());
    let output_path = args.next().unwrap_or_else(|| "output.png".to_owned());

    let mut img = ImageReader::open(input_path).unwrap().decode().unwrap();
    img.bokeh_blur(5.0, 150, 3.0, &KERNEL9_PARAM_SET);
    img.save(output_path).unwrap();
}
