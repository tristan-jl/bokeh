//! # Bokeh
//! A Rust implementation of image-blurring, focussing on disc-shaped kernels to produce a bokeh
//! lens-effect.
//!
//! Draws heavily on the work done [here](https://github.com/mikepound/convolve) by Mike Pound.

#![deny(missing_docs)]
mod complex;
mod gaussian;
pub mod params;

use std::f32;

use image::DynamicImage;

pub use self::complex::bokeh_blur;
pub use self::gaussian::gaussian_blur;
use self::params::KernelParamSet;

/// A trait that allows the blurring of images
pub trait Blur {
    /// Blurs the image using a Gaussian kernel
    fn gaussian_blur(&mut self, r: f32, kernel_radius: usize);
    /// Blurs the image with an approximation of a disc-shaped kernel to produce a bokeh effect
    fn bokeh_blur(&mut self, r: f64, kernel_radius: usize, gamma: f64, param_set: &KernelParamSet);
}

impl Blur for DynamicImage {
    fn gaussian_blur(&mut self, r: f32, kernel_radius: usize) {
        gaussian_blur(self, r, kernel_radius)
    }

    fn bokeh_blur(&mut self, r: f64, kernel_radius: usize, gamma: f64, param_set: &KernelParamSet) {
        bokeh_blur(self, r, kernel_radius, gamma, param_set)
    }
}
