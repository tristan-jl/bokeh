use std::f32;

use image::{DynamicImage, GenericImage, GenericImageView, Pixel};

fn gaussian(x: f32, r: f32) -> f32 {
    ((2.0 * f32::consts::PI).sqrt() * r).recip() * (-x.powi(2) / (2.0 * r.powi(2))).exp()
}

/*
fn complex_gaussian(a:f32,b:32,c:f32,d:f32) -> f32 {
    C
}
*/

pub fn gaussian_kernel(r: f32, kernel_radius: usize) -> Vec<f32> {
    let mut kernel = vec![0f32; 1 + 2 * kernel_radius];
    let mut sum = 0f32;
    for i in -(kernel_radius as isize)..=(kernel_radius as isize) {
        let val = gaussian(i as f32, r);
        kernel[(i + kernel_radius as isize) as usize] = val;
        sum += val;
    }
    for val in kernel.iter_mut() {
        *val /= sum;
    }

    debug_assert!(
        kernel.iter().fold(0.0, |acc, &i| acc + i) - 1.0 < 0.000001,
        "diff {}",
        (kernel.iter().fold(0.0, |acc, &i| acc + i)) - 1.0,
    );

    kernel
}

#[inline(always)]
fn horizontal_filter(
    input: &mut DynamicImage,
    output: &mut DynamicImage,
    kernel: &[f32],
    w: u32,
    h: u32,
) {
    let half_width = kernel.len() / 2;
    for i in (half_width as u32)..(w - half_width as u32) {
        for j in 0..h {
            let (mut r, mut g, mut b, mut a) = (0.0, 0.0, 0.0, 0.0);
            for (n, k) in kernel.iter().enumerate() {
                let x = (i as i32 - half_width as i32 + n as i32) as u32;
                let pixel = unsafe { input.unsafe_get_pixel(x, j) };

                r += pixel.0[0] as f32 * k;
                g += pixel.0[1] as f32 * k;
                b += pixel.0[2] as f32 * k;
                a += pixel.0[3] as f32 * k;
            }

            debug_assert!((input.get_pixel(i, j).0[3] as f32 - a) < 0.00001);

            unsafe {
                output.unsafe_put_pixel(
                    i,
                    j,
                    *Pixel::from_slice(&[r as u8, g as u8, b as u8, a as u8]),
                )
            };
        }
    }
}

#[inline(always)]
fn vertical_filter(
    input: &mut DynamicImage,
    output: &mut DynamicImage,
    kernel: &[f32],
    w: u32,
    h: u32,
) {
    let half_width = kernel.len() / 2;
    for i in 0..w {
        for j in (half_width as u32)..(h - half_width as u32) {
            let (mut r, mut g, mut b, mut a) = (0.0, 0.0, 0.0, 0.0);
            for (n, k) in kernel.iter().enumerate() {
                let y = (j as i32 - half_width as i32 + n as i32) as u32;
                let pixel = unsafe { input.unsafe_get_pixel(i, y) };

                r += pixel.0[0] as f32 * k;
                g += pixel.0[1] as f32 * k;
                b += pixel.0[2] as f32 * k;
                a += pixel.0[3] as f32 * k;
            }

            debug_assert!((input.get_pixel(i, j).0[3] as f32 - a) < 0.00001);

            unsafe {
                output.unsafe_put_pixel(
                    i,
                    j,
                    *Pixel::from_slice(&[r as u8, g as u8, b as u8, a as u8]),
                )
            };
        }
    }
}

pub fn gaussian_blur(img: &mut DynamicImage, r: f32, kernel_radius: usize) {
    let (w, h) = img.dimensions();
    let kernel = gaussian_kernel(r, kernel_radius);
    let mut intermediate = DynamicImage::new_rgb8(w, h);

    horizontal_filter(img, &mut intermediate, &kernel, w, h);
    vertical_filter(&mut intermediate, img, &kernel, w, h);
}

pub trait GaussianBlur {
    fn gaussian_blur(&mut self, r: f32, kernel_radius: usize);
}

impl GaussianBlur for DynamicImage {
    fn gaussian_blur(&mut self, r: f32, kernel_radius: usize) {
        gaussian_blur(self, r, kernel_radius)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kernel() {
        assert_eq!(
            gaussian_kernel(1.0, 9),
            vec![
                0.00013383062,
                0.0044318615,
                0.053991128,
                0.24197145,
                0.39894348,
                0.24197145,
                0.053991128,
                0.0044318615,
                0.00013383062
            ]
        );
    }
}
