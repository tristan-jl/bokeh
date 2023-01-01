mod complex;
mod gaussian;
pub mod params;

use std::f32;

use image::DynamicImage;

pub use self::complex::bokeh_blur;
pub use self::gaussian::gaussian_blur;
use self::params::KernelParamSet;

pub trait Blur {
    fn gaussian_blur(&mut self, r: f32, kernel_radius: usize);
    fn bokeh_blur(&mut self, r: f64, kernel_radius: usize, param_set: &KernelParamSet);
}

impl Blur for DynamicImage {
    fn gaussian_blur(&mut self, r: f32, kernel_radius: usize) {
        gaussian_blur(self, r, kernel_radius)
    }

    fn bokeh_blur(&mut self, r: f64, kernel_radius: usize, param_set: &KernelParamSet) {
        bokeh_blur(self, r, kernel_radius, param_set)
    }
}
