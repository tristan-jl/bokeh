use bokeh::GaussianBlur;
use image::io::Reader as ImageReader;

fn main() {
    let mut img = ImageReader::open("input.png").unwrap().decode().unwrap();
    img.gaussian_blur(50.0, 50);

    img.save("output.png").unwrap();
}
