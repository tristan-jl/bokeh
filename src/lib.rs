//! A Rust implementation of image-blurring using disc-shaped kernels to produce
//! a 'Bokeh' lens-effect.
//!
//! Draws heavily on the work done [here](https://github.com/mikepound/convolve)
//! by Mike Pound.
//!
//! The disc-shaped kernels are approximated by a sum of complex Gaussian
//! kernels. As a Gaussian blur is separable, instead of applying a single
//! 2-D kernel during convolution, a 1-D kernel on each axis can be applied. To
//! form a Bokeh blur effect, a disc-shaped kernel is used. This kernel is
//! created by summing multiple complex Gaussian kernels, with the number of
//! components improving the quality of the approximation. These relative
//! weights of these components are found by attempting to minimise the
//! deviation from a perfectly shaped kernel. See more [here](https://github.com/mikepound/convolve/blob/7f579ada8ab8c426cc157bf5f200a94dfdb50830/complex_kernels.py)
//! and [here](https://github.com/mikepound/convolve/issues/2).
//!
//! Currently only images with 4 channels are supported.
//!
//! Seperate APIs are available which allow a mask to be passed. This mask
//! allows pixels of the original image to be retained. This should be
//! a iterable of [`bool`]'s, where `true`'s correspond to the convolved image
//! and `false`'s corresponsed to the original.
//!
//! # Examples
//!
//! Using the [`image`](image) library (requires the `image` feature):
//! ```no_run
//! use bokeh::{params::KERNEL9_PARAM_SET, Blur};
//! use image::{io::Reader as ImageReader, ImageError};
//!
//! # fn main() -> Result<(), ImageError> {
//! // read the image
//! let mut img = ImageReader::open("myimage.jpg")?.decode()?;
//! // as the `bokeh::Blur` trait is imported
//! img.bokeh_blur(5.0, 150, 3.0, &KERNEL9_PARAM_SET);
//! // save the image
//! img.save("output.png")?;
//! # Ok(())
//! # }
//! ```
//!
//! Using functions directly:
//! ```
//! use bokeh::{bokeh_blur, params::KERNEL9_PARAM_SET};
//!
//! // create simple 'image'
//! let mut pixels = vec![[0., 0., 0., 0.]; 9];
//! pixels[4] = [255., 255., 255., 255.];
//!
//! // blur the image using 9 components
//! bokeh_blur(&mut pixels, 3, 3, 1.0, 1, 3.0, &KERNEL9_PARAM_SET);
//!
//! // pixels now blurred
//! assert_eq!(
//!     vec![
//!         5.837985890991395,
//!         149.12251807109067,
//!         5.837985890991395,
//!         149.12251807109067,
//!         149.12252112455516,
//!         149.12251807109067,
//!         5.837985890991395,
//!         149.12251807109067,
//!         5.837985890991395
//!     ]
//!     .iter()
//!     .map(|&i| [i, i, i, i])
//!     .collect::<Vec<_>>(),
//!     pixels
//! );
//! ```
//!
//! A utility struct [`Image`] is also provided:
//! ```
//! use bokeh::{Blur, Image, params::KERNEL9_PARAM_SET};
//!
//! let mut pixels = vec![[0., 0., 0., 0.]; 9];
//! pixels[4] = [255., 255., 255., 255.];
//! // same as above but using the struct
//! let mut img = Image::new(&mut pixels, 3, 3);
//!
//! img.bokeh_blur(1.0, 1, 3.0, &KERNEL9_PARAM_SET);
//!
//! assert_eq!(
//!     vec![
//!         5.837985890991395,
//!         149.12251807109067,
//!         5.837985890991395,
//!         149.12251807109067,
//!         149.12252112455516,
//!         149.12251807109067,
//!         5.837985890991395,
//!         149.12251807109067,
//!         5.837985890991395
//!     ]
//!     .iter()
//!     .map(|&i| [i, i, i, i])
//!     .collect::<Vec<_>>(),
//!     img.pixels
//! );
//! ```
//!
//! Providing a mask:
//! ```
//! use bokeh::{Blur, Image, params::KERNEL9_PARAM_SET};
//!
//! let mut pixels = vec![[0., 0., 0., 0.]; 9];
//! pixels[4] = [255., 255., 255., 255.];
//! // creating a mask
//! let mask = vec![false, true, false, true, false, true, false, true, false];
//! let mut img = Image::new(&mut pixels, 3, 3);
//!
//! img.bokeh_blur_with_mask(&mask, 1.0, 1, 3.0, &KERNEL9_PARAM_SET);
//!
//! assert_eq!(
//!     vec![
//!         0.,
//!         149.12251807109067,
//!         0.,
//!         149.12251807109067,
//!         255.,
//!         149.12251807109067,
//!         0.,
//!         149.12251807109067,
//!         0.
//!     ]
//!     .iter()
//!     .map(|&i| [i, i, i, i])
//!     .collect::<Vec<_>>(),
//!     img.pixels
//! );
//! ```
//! In the `assert!` statement above, comparing it to the previous example, it
//! can be seen that the original pixel values are retained.
// #![deny(missing_docs)]

pub mod complex;
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
    /// Blurs the image with an approximation of a disc-shaped kernel to produce
    /// a bokeh effect
    fn bokeh_blur(&mut self, r: f64, kernel_radius: usize, gamma: f64, param_set: &KernelParamSet);
    /// Blurs the image with an approximation of a disc-shaped kernel to produce
    /// a bokeh effect on areas covered by the mask
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
