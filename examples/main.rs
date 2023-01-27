use bokeh::{params::*, Blur};
use image::{io::Reader as ImageReader, GenericImageView};

fn main() {
    let input_path = "inputs/M35.jpg";
    let output_path = "output.png";

    let mut img = ImageReader::open(input_path).unwrap().decode().unwrap();
    let (x, y) = img.dimensions();
    let l = (x * y) as usize;
    let mut mask = vec![true; l / 2];
    mask.extend_from_slice(&vec![false; l / 2]);
    img.bokeh_blur_with_mask(&mask, 10.0, 3.0, &KERNEL9_PARAM_SET);
    img.save(output_path).unwrap();
}
