use bokeh::{params::*, Blur};
use image::io::Reader as ImageReader;

fn main() {
    let mut img = ImageReader::open("input.jpg").unwrap().decode().unwrap();
    img.bokeh_blur(5.0, 100, &KERNEL9_PARAM_SET);

    img.save("output.png").unwrap();
}
