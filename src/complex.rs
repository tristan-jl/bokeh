use image::{DynamicImage, GenericImage, GenericImageView, Pixel};
use num::complex::ComplexFloat;
use num::Complex;

use crate::params::KernelParamSet;

/// _UNNORMALISED_ complex gaussian kernel
fn complex_gaussian_kernel(r: f64, kernel_radius: usize, a: f64, b: f64) -> Vec<Complex<f64>> {
    let kernel_size = 1 + kernel_radius * 2;
    let mut kernel: Vec<Complex<f64>> = vec![Complex::new(0.0, 0.0); kernel_size];

    for i in -(kernel_radius as isize)..=(kernel_radius as isize) {
        let ax = i as f64 * r * (1.0 / kernel_radius as f64);
        let ax2 = ax * ax;
        let exp_a = (-a * ax2).exp();
        let val = Complex::new(exp_a * (b * ax2).cos(), exp_a * (b * ax2).sin());
        kernel[(i + kernel_radius as isize) as usize] = val;
    }

    kernel
}

fn complex_gaussian_kernels(
    params: &KernelParamSet,
    r: f64,
    kernel_radius: usize,
) -> Vec<Vec<Complex<f64>>> {
    let mut kernels = (0..params.num_kernels())
        .map(|i| complex_gaussian_kernel(r, kernel_radius, params.a(i), params.b(i)))
        .collect::<Vec<_>>();

    let mut sum = 0.0;
    for (n, k) in kernels.iter().enumerate() {
        for i in k {
            for j in k {
                sum += params.A(n) * (i.re * j.re - i.im * j.im)
                    + params.B(n) * (i.re * j.im + i.im * j.re)
            }
        }
    }
    sum = sum.sqrt();

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
                        s += params.A(n) * (i.re * j.re - i.im * j.im)
                            + params.B(n) * (i.re * j.im + i.im * j.re)
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
                        s += params.A(n) * (i.re * j.re - i.im * j.im)
                            + params.B(n) * (i.re * j.im + i.im * j.re)
                    }
                }
            }
            s
        }
    );

    kernels
}

fn horizontal_filter(
    input: &[[Complex<f64>; 4]],
    output: &mut [[Complex<f64>; 4]],
    kernel: &[Complex<f64>],
    w: u32,
    h: u32,
) {
    debug_assert!(input.len() == (w * h) as usize);
    debug_assert!(input.len() == output.len());
    let (w, h) = (w as usize, h as usize);

    let half_width = kernel.len() / 2;
    for i in half_width..(w - half_width) {
        for j in 0..h {
            let mut out_pixel = [Complex::default(); 4];
            for (n, k) in kernel.iter().enumerate() {
                let x = i as isize - half_width as isize + n as isize;
                debug_assert!(x >= 0);
                let x = x as usize;

                for (o, p) in out_pixel.iter_mut().zip(input[(j * w) + x].iter()) {
                    *o += p * k;
                }
            }

            output[(j * w) + i] = out_pixel;
        }
    }
}

fn vertical_filter(
    input: &[[Complex<f64>; 4]],
    output: &mut [[Complex<f64>; 4]],
    kernel: &[Complex<f64>],
    w: u32,
    h: u32,
) {
    debug_assert!(input.len() == (w * h) as usize);
    debug_assert!(input.len() == output.len());
    let (w, h) = (w as usize, h as usize);

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
    }
}

pub fn bokeh_blur(
    img: &mut DynamicImage,
    r: f64,
    kernel_radius: usize,
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
                Complex::new(c[0] as f64, 0.0),
                Complex::new(c[1] as f64, 0.0),
                Complex::new(c[2] as f64, 0.0),
                Complex::new(c[3] as f64, 0.0),
            ]
        })
        .collect::<Vec<_>>();

    dbg!(&kernels);

    let total_out = kernels
        .iter()
        .map(|kernel| {
            let mut temp = vec![[Complex::new(0.0, 0.0); 4]; (w * h) as usize];
            let mut output = vec![[Complex::new(0.0, 0.0); 4]; (w * h) as usize];
            horizontal_filter(&input, &mut temp, kernel, w, h);
            vertical_filter(&temp, &mut output, kernel, w, h);
            output
        })
        .collect::<Vec<_>>();

    for (n, [r, g, b, a]) in total_out
        .iter()
        .enumerate()
        .fold(vec![[0.0; 4]; (w * h) as usize], |mut acc, (n, x)| {
            for (a, y) in acc.iter_mut().zip(x.iter()) {
                a[0] += param_set.A(n) * y[0].re + param_set.B(n) * y[0].im;
                a[1] += param_set.A(n) * y[1].re + param_set.B(n) * y[1].im;
                a[2] += param_set.A(n) * y[2].re + param_set.B(n) * y[2].im;
                a[3] += param_set.A(n) * y[3].re + param_set.B(n) * y[3].im;
            }
            acc
        })
        .into_iter()
        .enumerate()
    {
        /*
        debug_assert!(
            [r, g, b, a].iter().all(|&x| (0.0..256.0).contains(&x)),
            "RGBA OOB: {} {} {} {}",
            r,
            g,
            b,
            a
        );
            */
        let r = r.clamp(0.0, 255.0) as u8;
        let g = g.clamp(0.0, 255.0) as u8;
        let b = b.clamp(0.0, 255.0) as u8;
        let a = a.clamp(0.0, 255.0) as u8;

        let x = n as u32 % w;
        let y = n as u32 / w;

        unsafe { img.unsafe_put_pixel(x, y, *Pixel::from_slice(&[r, g, b, a])) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /*
    #[test]
    fn test_complex_kernel() {
        let r = complex_gaussian(1.4, 4, 0.862325, 1.624835);
        dbg!(r.iter().fold(0.0, |acc, x| acc + x.norm()).abs());
        assert!((r.iter().fold(0.0, |acc, x| acc + x.norm()).abs() - 1.0) < 0.00000001);

        let expected = [
            Complex::new(-5.8840633e-06, -1.7689392e-06),
            Complex::new(3.2656451e-03, 3.5559295e-03),
            Complex::new(-5.4171007e-02, 2.5797081e-01),
            Complex::new(1.0000000e+00, 0.0000000e+00),
            Complex::new(-5.4171007e-02, 2.5797081e-01),
            Complex::new(3.2656451e-03, 3.5559295e-03),
            Complex::new(-5.8840633e-06, 1.7689392e-06),
        ];
        assert_eq!(r, &expected);
    }
    */
    #[test]
    fn test_complex_kernel() {
        let r = complex_gaussian_kernel(1.4, 4, 0.862325, 1.624835);

        /*
        dbg!(r
            .iter()
            .map(|i| Complex::new(i.re as f64, i.im as f64))
            .zip(r2.iter())
            .map(|(i, j)| i - j)
            .collect::<Vec<_>>());

        assert_eq!(
            r.iter()
                .map(|i| Complex::new(i.re as f64, i.im as f64))
                .collect::<Vec<_>>(),
            r2
        );
        */

        dbg!(r.iter().map(|i| i.norm()).collect::<Vec<_>>());

        let expected = [
            Complex::new(-5.8840633e-06, -1.7689392e-06),
            Complex::new(3.2656451e-03, 3.5559295e-03),
            Complex::new(-5.4171007e-02, 2.5797081e-01),
            Complex::new(1.0000000e+00, 0.0000000e+00),
            Complex::new(-5.4171007e-02, 2.5797081e-01),
            Complex::new(3.2656451e-03, 3.5559295e-03),
            Complex::new(-5.8840633e-06, 1.7689392e-06),
        ];
        assert_eq!(r, &expected);
    }
}
