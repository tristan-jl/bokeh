//! # Bokeh
//! A Rust implementation of image-blurring, focussing on disc-shaped kernels to produce a bokeh
//! lens-effect.
//!
//! Draws heavily on the work done [here](https://github.com/mikepound/convolve) by Mike Pound.

mod complex;
pub mod params;

use self::params::KernelParamSet;
use image::DynamicImage;

pub use self::complex::bokeh_blur;
pub use self::complex::bokeh_blur_with_mask;
pub use self::complex::dynamic_image;

/// A trait that allows the blurring of images
pub trait Blur {
    /// Blurs the image with an approximation of a disc-shaped kernel to produce a bokeh effect
    fn bokeh_blur(&mut self, r: f64, kernel_radius: usize, gamma: f64, param_set: &KernelParamSet);
    /// Blurs the image with an approximation of a disc-shaped kernel to produce a bokeh effect on
    /// areas covered by the mask
    fn bokeh_blur_with_mask<'a>(
        &mut self,
        mask: impl IntoIterator<Item = &'a bool>,
        r: f64,
        kernel_radius: usize,
        gamma: f64,
        param_set: &KernelParamSet,
    );
}

impl Blur for DynamicImage {
    fn bokeh_blur(&mut self, r: f64, kernel_radius: usize, gamma: f64, param_set: &KernelParamSet) {
        dynamic_image::bokeh_blur(self, r, kernel_radius, gamma, param_set)
    }

    fn bokeh_blur_with_mask<'a>(
        &mut self,
        mask: impl IntoIterator<Item = &'a bool>,
        r: f64,
        kernel_radius: usize,
        gamma: f64,
        param_set: &KernelParamSet,
    ) {
        dynamic_image::bokeh_blur_with_mask(self, mask, r, kernel_radius, gamma, param_set)
    }
}

#[derive(Debug)]
pub struct Image<'a> {
    img: &'a mut [[f64; 4]],
    w: usize,
    h: usize,
}

impl<'a> Image<'a> {
    pub fn new(img: &'a mut [[f64; 4]], w: usize, h: usize) -> Self {
        Self { img, w, h }
    }
}

impl<'a> Blur for Image<'a> {
    fn bokeh_blur(&mut self, r: f64, kernel_radius: usize, gamma: f64, param_set: &KernelParamSet) {
        bokeh_blur(self.img, self.w, self.h, r, kernel_radius, gamma, param_set)
    }

    fn bokeh_blur_with_mask<'b>(
        &mut self,
        mask: impl IntoIterator<Item = &'b bool>,
        r: f64,
        kernel_radius: usize,
        gamma: f64,
        param_set: &KernelParamSet,
    ) {
        bokeh_blur_with_mask(
            self.img,
            mask,
            self.w,
            self.h,
            r,
            kernel_radius,
            gamma,
            param_set,
        )
    }
}
