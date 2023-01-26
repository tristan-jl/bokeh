//! # Bokeh
//! A Rust implementation of image-blurring, focussing on disc-shaped kernels to produce a bokeh
//! lens-effect.
//!
//! Draws heavily on the work done [here](https://github.com/mikepound/convolve) by Mike Pound.

mod complex;
pub mod params;

use self::params::KernelParamSet;

#[cfg(feature = "image")]
use image::DynamicImage;

pub use self::complex::bokeh_blur;
pub use self::complex::bokeh_blur_with_mask;
#[cfg(feature = "image")]
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

#[cfg(feature = "image")]
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

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Image<'a> {
    pub pixels: &'a mut [[f64; 4]],
    w: usize,
    h: usize,
}

impl<'a> Image<'a> {
    pub fn new(pixels: &'a mut [[f64; 4]], w: usize, h: usize) -> Self {
        Self { pixels, w, h }
    }
}

impl<'a> Blur for Image<'a> {
    fn bokeh_blur(&mut self, r: f64, kernel_radius: usize, gamma: f64, param_set: &KernelParamSet) {
        bokeh_blur(
            self.pixels,
            self.w,
            self.h,
            r,
            kernel_radius,
            gamma,
            param_set,
        )
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
            self.pixels,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::params::KERNEL9_PARAM_SET;

    macro_rules! image {
        ([$($f:expr),+]) => {{
            [$([$f, $f, $f, $f],)+]
        }}
    }

    #[test]
    fn blurs() {
        let mut pixels = image!([0., 0., 0., 0., 255., 0., 0., 0., 0.]);
        let mut img = Image::new(&mut pixels, 3, 3);

        img.bokeh_blur(1.0, 1, 3.0, &KERNEL9_PARAM_SET);

        assert_eq!(
            img.pixels,
            image!([
                5.837985890991395,
                149.12251807109067,
                5.837985890991395,
                149.12251807109067,
                149.12252112455516,
                149.12251807109067,
                5.837985890991395,
                149.12251807109067,
                5.837985890991395
            ])
        );
    }

    #[test]
    fn blurs_with_mask() {
        let mut pixels = image!([0., 0., 0., 0., 255., 0., 0., 0., 0.]);
        let mask = [false, true, false, true, false, true, false, true, false];
        let mut img = Image::new(&mut pixels, 3, 3);

        img.bokeh_blur_with_mask(&mask, 1.0, 1, 3.0, &KERNEL9_PARAM_SET);

        assert_eq!(
            img.pixels,
            image!([
                0.,
                149.12251807109067,
                0.,
                149.12251807109067,
                255.,
                149.12251807109067,
                0.,
                149.12251807109067,
                0.
            ])
        );
    }
}
