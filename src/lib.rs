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
//! To view the shapes of the kernels for different numbers of compoents and for
//! different kernel radii, see [here](https://github.com/tristan-jl/bokeh/blob/master/docs/kernel_shapes.png).
//! As can be see from this after 4/5 components there are very much diminishing
//! returns in using more. Each additional component used increases the number
//! of convolutions carried out on the image, i.e. using 8 components is 2 times
//! slower than using 4.
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
//! Using the [`image`](image) library (requires the default `image` feature):
//! ```no_run
//! use bokeh::{params::KERNEL9_PARAM_SET, Blur};
//! use image::{io::Reader as ImageReader, ImageError};
//!
//! # fn main() -> Result<(), ImageError> {
//! // read the image
//! let mut img = ImageReader::open("myimage.jpg")?.decode()?;
//! // as the `bokeh::Blur` trait is imported
//! img.bokeh_blur(1.0, &KERNEL9_PARAM_SET, 3.0);
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
//! bokeh_blur(&mut pixels, 3, 3, 1.0, &KERNEL9_PARAM_SET, 3.0);
//!
//! // pixels now blurred
//! assert_eq!(
//!     vec![
//!         1.6428886692061846,
//!         14.80242203513296,
//!         1.6428886692061846,
//!         14.802422035132915,
//!         254.93338630375473,
//!         14.802422035132915,
//!         1.6428886692061846,
//!         14.80242203513296,
//!         1.6428886692061846
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
//! img.bokeh_blur(1.0, &KERNEL9_PARAM_SET, 3.0);
//!
//! assert_eq!(
//!     vec![
//!         1.6428886692061846,
//!         14.80242203513296,
//!         1.6428886692061846,
//!         14.802422035132915,
//!         254.93338630375473,
//!         14.802422035132915,
//!         1.6428886692061846,
//!         14.80242203513296,
//!         1.6428886692061846
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
//! img.bokeh_blur_with_mask(&mask, 1.0, &KERNEL9_PARAM_SET, 3.0);
//!
//! assert_eq!(
//!     vec![
//!         0.,
//!         14.80242203513296,
//!         0.,
//!         14.802422035132915,
//!         255.,
//!         14.802422035132915,
//!         0.,
//!         14.80242203513296,
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
#![deny(missing_docs)]

mod complex;
pub mod params;

use self::params::KernelParamSet;

#[cfg(feature = "image")]
use image::DynamicImage;

pub use self::complex::bokeh_blur;
pub use self::complex::bokeh_blur_with_mask;
#[cfg(feature = "image")]
pub use self::complex::dynamic_image;
pub use self::complex::kernel_gaussian_components;

/// A trait that allows the blurring of images
pub trait Blur {
    /// Blurs the image using an approximation of a disc-shaped kernel to
    /// produce a Bokeh lens effect.
    ///
    /// The image is blurred by a disc-shaped kernel with radius `radius`,
    /// built from components corresponding to `param_set`. The exposure can be
    /// modified using `gamma`, set to `1.0` for no change.
    fn bokeh_blur(&mut self, radius: f64, param_set: &KernelParamSet, gamma: f64);

    /// Blurs the selected parts of an image using an approximation of a
    /// disc-shaped kernel to produce a Bokeh lens effect.
    ///
    /// Takes a `mask` of the same length as the image where `true`'s correspond
    /// to the convolved image and `false`'s corresponsed to the original.
    /// The image is blurred by a disc-shaped kernel with radius `radius`,
    /// built from components corresponding to `param_set`. The exposure can be
    /// modified using `gamma`, set to `1.0` for no change.
    fn bokeh_blur_with_mask<'a>(
        &mut self,
        mask: impl IntoIterator<Item = &'a bool>,
        radius: f64,
        param_set: &KernelParamSet,
        gamma: f64,
    );
}

#[cfg(feature = "image")]
impl Blur for DynamicImage {
    fn bokeh_blur(&mut self, radius: f64, param_set: &KernelParamSet, gamma: f64) {
        dynamic_image::bokeh_blur(self, radius, param_set, gamma)
    }

    fn bokeh_blur_with_mask<'a>(
        &mut self,
        mask: impl IntoIterator<Item = &'a bool>,
        radius: f64,
        param_set: &KernelParamSet,
        gamma: f64,
    ) {
        dynamic_image::bokeh_blur_with_mask(self, mask, radius, param_set, gamma)
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
/// Utility wrapper struct representing an image
pub struct Image<'a> {
    /// Image's pixels
    pub pixels: &'a mut [[f64; 4]],
    w: usize,
    h: usize,
}

impl<'a> Image<'a> {
    /// Creates a new `Image` containing an exclusive reference to a slice of
    /// pixels
    pub fn new(pixels: &'a mut [[f64; 4]], w: usize, h: usize) -> Self {
        Self { pixels, w, h }
    }
}

impl<'a> Blur for Image<'a> {
    fn bokeh_blur(&mut self, radius: f64, param_set: &KernelParamSet, gamma: f64) {
        bokeh_blur(self.pixels, self.w, self.h, radius, param_set, gamma)
    }

    fn bokeh_blur_with_mask<'b>(
        &mut self,
        mask: impl IntoIterator<Item = &'b bool>,
        radius: f64,
        param_set: &KernelParamSet,
        gamma: f64,
    ) {
        bokeh_blur_with_mask(self.pixels, mask, self.w, self.h, radius, param_set, gamma)
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

        img.bokeh_blur(1.0, &KERNEL9_PARAM_SET, 3.0);

        assert_eq!(
            img.pixels,
            image!([
                1.6428886692061846,
                14.80242203513296,
                1.6428886692061846,
                14.802422035132915,
                254.93338630375473,
                14.802422035132915,
                1.6428886692061846,
                14.80242203513296,
                1.6428886692061846
            ])
        );
    }

    #[test]
    fn blurs_with_mask() {
        let mut pixels = image!([0., 0., 0., 0., 255., 0., 0., 0., 0.]);
        let mask = [false, true, false, true, false, true, false, true, false];
        let mut img = Image::new(&mut pixels, 3, 3);

        img.bokeh_blur_with_mask(&mask, 1.0, &KERNEL9_PARAM_SET, 3.0);

        assert_eq!(
            img.pixels,
            image!([
                0.,
                14.80242203513296,
                0.,
                14.802422035132915,
                255.,
                14.802422035132915,
                0.,
                14.80242203513296,
                0.
            ])
        );
    }
}
