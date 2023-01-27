use crate::params::KernelParamSet;
use num::Complex;
use rayon::prelude::*;

#[cfg(feature = "image")]
use image::{DynamicImage, GenericImageView, Pixel};

type ComplexPixel = [Complex<f64>; 4];

/// _UNNORMALISED_ complex gaussian kernel
fn complex_gaussian_kernel(r: f64, kernel_radius: usize, a: f64, b: f64) -> Vec<Complex<f64>> {
    let kernel_size = 1 + kernel_radius * 2;
    let mut kernel: Vec<Complex<f64>> = vec![Complex::new(0.0, 0.0); kernel_size];

    for i in -(kernel_radius as isize)..=(kernel_radius as isize) {
        let ax = i as f64 * r / kernel_radius as f64;
        let ax2 = ax * ax;
        let exp_a = (-a * ax2).exp();
        let val = Complex::new(exp_a * (b * ax2).cos(), exp_a * (b * ax2).sin());
        kernel[(i + kernel_radius as isize) as usize] = val;
    }

    kernel
}

/// Build all the gaussian kernels and normalise w.r.t. params, ie so that after
/// all the kernels are applied the pixel remains the same brightnes
fn complex_gaussian_kernels(
    params: &KernelParamSet,
    r: f64,
    kernel_radius: usize,
) -> Vec<Vec<Complex<f64>>> {
    let mut kernels = (0..params.num_kernels())
        .map(|i| complex_gaussian_kernel(r, kernel_radius, params.a(i), params.b(i)))
        .collect::<Vec<_>>();

    let sum = kernels
        .iter()
        .enumerate()
        .fold(0.0, |acc, (n, k)| {
            acc + {
                let mut s = 0.0;
                for i in k {
                    for j in k {
                        s += params.real_component(n) * (i.re * j.re - i.im * j.im)
                            + params.imag_component(n) * (i.re * j.im + i.im * j.re)
                    }
                }
                s
            }
        })
        .sqrt();

    for kernel in kernels.iter_mut() {
        for elem in kernel.iter_mut() {
            *elem /= sum;
        }
    }

    // Check normalisation
    debug_assert!(
        {
            let mut s = 0.0;
            for (n, k) in kernels.iter().enumerate() {
                for i in k {
                    for j in k {
                        s += params.real_component(n) * (i.re * j.re - i.im * j.im)
                            + params.imag_component(n) * (i.re * j.im + i.im * j.re)
                    }
                }
            }
            s
        } - 1.0
            < 0.000000001,
        "Kernel doesn't sum to 1: {}",
        {
            let mut s = 0.0;
            for (n, k) in kernels.iter().enumerate() {
                for i in k {
                    for j in k {
                        s += params.real_component(n) * (i.re * j.re - i.im * j.im)
                            + params.imag_component(n) * (i.re * j.im + i.im * j.re)
                    }
                }
            }
            s
        }
    );

    kernels
}

fn horizontal_filter(
    input: &[ComplexPixel],
    kernel: &[Complex<f64>],
    w: usize,
    h: usize,
) -> Vec<ComplexPixel> {
    debug_assert!(input.len() == (w * h) as usize);
    let mut output = vec![[Complex::new(0.0, 0.0); 4]; w * h];

    let half_width = kernel.len() / 2;
    debug_assert!(w >= half_width);
    debug_assert!(h >= half_width);
    for j in 0..h {
        for i in half_width..(w - half_width) {
            let mut out_pixel = [Complex::default(); 4];
            for (n, k) in kernel.iter().enumerate() {
                let x = i as isize - half_width as isize + n as isize;
                debug_assert!(x >= 0);
                let x = x as usize;

                for (out_subpixel, in_subpixel) in
                    out_pixel.iter_mut().zip(input[(j * w) + x].iter())
                {
                    *out_subpixel += in_subpixel * k;
                }
            }

            output[(j * w) + i] = out_pixel;
        }

        for i in (0..half_width).chain((w - half_width)..w) {
            let mut out_pixel = [Complex::default(); 4];
            for (n, k) in kernel.iter().enumerate() {
                let x = i as isize - half_width as isize + n as isize;
                if x < 0 || x >= w as isize {
                    continue;
                }
                let x = x as usize;

                for (out_subpixel, in_subpixel) in
                    out_pixel.iter_mut().zip(input[(j * w) + x].iter())
                {
                    *out_subpixel += in_subpixel * k;
                }
            }

            output[(j * w) + i] = out_pixel;
        }
    }

    output
}

fn vertical_filter(
    input: &[ComplexPixel],
    kernel: &[Complex<f64>],
    w: usize,
    h: usize,
) -> Vec<ComplexPixel> {
    debug_assert!(input.len() == (w * h) as usize);
    let mut output = vec![[Complex::new(0.0, 0.0); 4]; w * h];

    let half_width = kernel.len() / 2;
    for i in 0..w {
        for j in half_width..(h - half_width) {
            let mut out_pixel = [Complex::default(); 4];
            for (n, k) in kernel.iter().enumerate() {
                let y = j as isize - half_width as isize + n as isize;
                debug_assert!(y >= 0);
                let y = y as usize;

                for (o, p) in out_pixel.iter_mut().zip(input[(y * w) + i].iter()) {
                    *o += p * k;
                }
            }

            output[(j * w) + i] = out_pixel;
        }

        for j in (0..half_width).chain((h - half_width)..h) {
            let mut out_pixel = [Complex::default(); 4];
            for (n, k) in kernel.iter().enumerate() {
                let y = j as isize - half_width as isize + n as isize;
                if y < 0 || y >= h as isize {
                    continue;
                }
                let y = y as usize;

                for (o, p) in out_pixel.iter_mut().zip(input[(y * w) + i].iter()) {
                    *o += p * k;
                }
            }

            output[(j * w) + i] = out_pixel;
        }
    }

    output
}

struct ComplexImage {
    pixels: Vec<ComplexPixel>,
    w: usize,
    h: usize,
}

impl ComplexImage {
    #[cfg(feature = "image")]
    pub fn from_dynamic_image(img: &DynamicImage, gamma: f64) -> Self {
        let input = img
            .pixels()
            .map(|(_, _, pixel)| {
                let c = pixel.channels();
                debug_assert_eq!(c.len(), 4);
                [
                    Complex::new((c[0] as f64).powf(gamma), 0.0),
                    Complex::new((c[1] as f64).powf(gamma), 0.0),
                    Complex::new((c[2] as f64).powf(gamma), 0.0),
                    Complex::new((c[3] as f64).powf(gamma), 0.0),
                ]
            })
            .collect::<Vec<_>>();

        let (w, h) = img.dimensions();

        Self {
            pixels: input,
            w: w as usize,
            h: h as usize,
        }
    }

    /// From an image stored as a vector with 4 channels
    pub fn from_slice(img: &[[f64; 4]], w: usize, h: usize, gamma: f64) -> Self {
        let input = img
            .iter()
            .map(|c| {
                [
                    Complex::new((c[0] as f64).powf(gamma), 0.0),
                    Complex::new((c[1] as f64).powf(gamma), 0.0),
                    Complex::new((c[2] as f64).powf(gamma), 0.0),
                    Complex::new((c[3] as f64).powf(gamma), 0.0),
                ]
            })
            .collect::<Vec<_>>();

        Self {
            pixels: input,
            w: w as usize,
            h: h as usize,
        }
    }

    fn bokeh_blur(self, param_set: &KernelParamSet, r: f64, kernel_radius: usize) -> Vec<[f64; 4]> {
        complex_gaussian_kernels(param_set, r, kernel_radius)
            .par_iter()
            .enumerate()
            .map(|(n, kernel)| {
                let temp = horizontal_filter(&self.pixels, kernel, self.w, self.h);
                vertical_filter(&temp, kernel, self.w, self.h)
                    .iter()
                    .map(|pixel| {
                        let re = param_set.real_component(n);
                        let im = param_set.imag_component(n);
                        [
                            re * pixel[0].re + im * pixel[0].im,
                            re * pixel[1].re + im * pixel[1].im,
                            re * pixel[2].re + im * pixel[2].im,
                            re * pixel[3].re + im * pixel[3].im,
                        ]
                    })
                    .collect()
            })
            .reduce(
                || vec![[0.0; 4]; self.w * self.h],
                |mut a, b| {
                    for (x, y) in a.iter_mut().zip(b.iter()) {
                        x[0] += y[0];
                        x[1] += y[1];
                        x[2] += y[2];
                        x[3] += y[3];
                    }
                    a
                },
            )
    }
}

/// Blurs an image using an approximation of a disc-shaped kernel to produce a
/// Bokeh lens effect.
///
/// Takes an exclusive reference to a slice of size 4 arrays, where each array
/// element corresponds to a pixel. Each element of the array corresponds to R,
/// G, B, A. Also requires the `width` and `height` of the image. The image is
/// blurred by a disc-shaped kernel with
pub fn bokeh_blur(
    img: &mut [[f64; 4]],
    width: usize,
    height: usize,
    r: f64,
    kernel_radius: usize,
    gamma: f64,
    param_set: &KernelParamSet,
) {
    for (n, rgba) in ComplexImage::from_slice(img, width, height, gamma)
        .bokeh_blur(param_set, r, kernel_radius)
        .into_iter()
        .enumerate()
    {
        // Clamp any values from floating point ops
        img[n] = rgba.map(|i| i.powf(1.0 / gamma).clamp(0.0, 255.0));
    }
}

#[allow(clippy::too_many_arguments)]
/// Blurs an image using an approximation of a disc-shaped kernel to produce a
/// Bokeh lens effect
pub fn bokeh_blur_with_mask<'a>(
    img: &mut [[f64; 4]],
    mask: impl IntoIterator<Item = &'a bool>,
    width: usize,
    height: usize,
    r: f64,
    kernel_radius: usize,
    gamma: f64,
    param_set: &KernelParamSet,
) {
    // TODO optimisation where only convolve regions not masked, have to look at
    // places within kernel radius
    for ((n, rgba), mask_i) in ComplexImage::from_slice(img, width, height, gamma)
        .bokeh_blur(param_set, r, kernel_radius)
        .into_iter()
        .enumerate()
        .zip(mask.into_iter())
    {
        if *mask_i {
            // Clamp any values from floating point ops
            img[n] = rgba.map(|i| i.powf(1.0 / gamma).clamp(0.0, 255.0));
        }
    }
}

#[cfg(feature = "image")]
pub mod dynamic_image {
    use super::ComplexImage;
    use crate::params::KernelParamSet;
    use image::{DynamicImage, GenericImage, Pixel};

    /// Blurs an image using an approximation of a disc-shaped kernel to produce
    /// a Bokeh lens effect
    pub fn bokeh_blur(
        img: &mut DynamicImage,
        sigma: f64,
        kernel_radius: usize,
        gamma: f64,
        param_set: &KernelParamSet,
    ) {
        let w = img.width();

        for (n, rgba) in ComplexImage::from_dynamic_image(img, gamma)
            .bokeh_blur(param_set, sigma, kernel_radius)
            .into_iter()
            .enumerate()
        {
            // Safety: definitely in bounds due to iteration ranges
            unsafe {
                img.unsafe_put_pixel(
                    n as u32 % w,
                    n as u32 / w,
                    // Clamp any values from floating point ops - ensure the cast to u8 is ok
                    *Pixel::from_slice(&rgba.map(|i| i.powf(1.0 / gamma).clamp(0.0, 255.0) as u8)),
                )
            }
        }
    }

    // TODO optimisation where only convolve regions not masked, have to look at
    // places within kernel radius
    pub fn bokeh_blur_with_mask<'a>(
        img: &mut DynamicImage,
        mask: impl IntoIterator<Item = &'a bool>,
        sigma: f64,
        kernel_radius: usize,
        gamma: f64,
        param_set: &KernelParamSet,
    ) {
        let w = img.width();

        for ((n, rgba), mask_i) in ComplexImage::from_dynamic_image(img, gamma)
            .bokeh_blur(param_set, sigma, kernel_radius)
            .into_iter()
            .enumerate()
            .zip(mask.into_iter())
        {
            if *mask_i {
                // Safety: definitely in bounds due to iteration ranges
                unsafe {
                    img.unsafe_put_pixel(
                        n as u32 % w,
                        n as u32 / w,
                        // Clamp any values from floating point ops - ensure the cast to u8 is ok
                        *Pixel::from_slice(
                            &rgba.map(|i| i.powf(1.0 / gamma).clamp(0.0, 255.0) as u8),
                        ),
                    )
                }
            }
        }
    }
}
