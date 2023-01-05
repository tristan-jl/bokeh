use image::{DynamicImage, GenericImage, GenericImageView, Pixel};
use num::Complex;

use crate::params::KernelParamSet;

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

/// Build all the gaussian kernels and normalise w.r.t. params, ie so that after all the kernels
/// are applied the pixel remains the same brightnes
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
    w: u32,
    h: u32,
) -> Vec<ComplexPixel> {
    debug_assert!(input.len() == (w * h) as usize);
    let (w, h) = (w as usize, h as usize);
    let mut output = vec![[Complex::new(0.0, 0.0); 4]; w * h];

    let half_width = kernel.len() / 2;
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
    w: u32,
    h: u32,
) -> Vec<ComplexPixel> {
    debug_assert!(input.len() == (w * h) as usize);
    let (w, h) = (w as usize, h as usize);
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

/// Blurs an image using an approximation of a disc-shaped kernel to produce a Bokeh lens effect
pub fn bokeh_blur(
    img: &mut DynamicImage,
    r: f64,
    kernel_radius: usize,
    gamma: f64,
    param_set: &KernelParamSet,
) {
    let (w, h) = img.dimensions();
    let kernels = complex_gaussian_kernels(param_set, r, kernel_radius);

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

    for (n, [r, g, b, a]) in kernels
        .iter()
        .map(|kernel| {
            let temp = horizontal_filter(&input, kernel, w, h);
            vertical_filter(&temp, kernel, w, h)
        })
        .enumerate()
        .fold(vec![[0.0; 4]; (w * h) as usize], |mut acc, (n, x)| {
            for (a, y) in acc.iter_mut().zip(x.iter()) {
                let re = param_set.real_component(n);
                let im = param_set.imag_component(n);
                a[0] += re * y[0].re + im * y[0].im;
                a[1] += re * y[1].re + im * y[1].im;
                a[2] += re * y[2].re + im * y[2].im;
                a[3] += re * y[3].re + im * y[3].im;
            }
            acc
        })
        .into_iter()
        .enumerate()
    {
        // Clamp any values from floating point ops - ensure the cast to u8 is ok
        let r = r.powf(1.0 / gamma).clamp(0.0, 255.0) as u8;
        let g = g.powf(1.0 / gamma).clamp(0.0, 255.0) as u8;
        let b = b.powf(1.0 / gamma).clamp(0.0, 255.0) as u8;
        let a = a.powf(1.0 / gamma).clamp(0.0, 255.0) as u8;

        // Safety: definitely in bounds due to iteration ranges
        unsafe {
            img.unsafe_put_pixel(
                n as u32 % w,
                n as u32 / w,
                *Pixel::from_slice(&[r, g, b, a]),
            )
        }
    }
}
