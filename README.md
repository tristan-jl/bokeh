# Bokeh

A Rust implementation of image-blurring using disc-shaped kernels to produce
a 'Bokeh' lens-effect.

Draws heavily on the work done [here](https://github.com/mikepound/convolve)
by Mike Pound.

The disc-shaped kernels are approximated by a sum of complex Gaussian
kernels. As a Gaussian blur is separable, instead of applying a single
2-D kernel during convolution, a 1-D kernel on each axis can be applied. To
form a Bokeh blur effect, a disc-shaped kernel is used. This kernel is
created by summing multiple complex Gaussian kernels, with the number of
components improving the quality of the approximation. These relative
weights of these components are found by attempting to minimise the
deviation from a perfectly shaped kernel. See more [here](https://github.com/mikepound/convolve/blob/7f579ada8ab8c426cc157bf5f200a94dfdb50830/complex_kernels.py)
and [here](https://github.com/mikepound/convolve/issues/2).

To view the shapes of the kernels for different numbers of compoents and for
different kernel radii, see [here](https://github.com/tristan-jl/bokeh/blob/master/docs/kernel_shapes.png).
As can be see from this after 4/5 components there are very much diminishing
returns in using more. Each additional component used increases the number
of convolutions carried out on the image, i.e. using 8 components is 2 times
slower than using 4.

Currently only images with 4 channels are supported.

Seperate APIs are available which allow a mask to be passed. This mask
allows pixels of the original image to be retained. This should be
a iterable of [`bool`]'s, where `true`'s correspond to the convolved image
and `false`'s corresponsed to the original.

## Examples

Using the [`image`](image) library (requires the default `image` feature):
```rust
use bokeh::{params::KERNEL9_PARAM_SET, Blur};
use image::{io::Reader as ImageReader, ImageError};

// read the image
let mut img = ImageReader::open("myimage.jpg")?.decode()?;
// as the `bokeh::Blur` trait is imported
img.bokeh_blur(1.0, &KERNEL9_PARAM_SET, 3.0);
// save the image
img.save("output.png")?;
```

Using functions directly:
```rust
use bokeh::{bokeh_blur, params::KERNEL9_PARAM_SET};

// create simple 'image'
let mut pixels = vec![[0., 0., 0., 0.]; 9];
pixels[4] = [255., 255., 255., 255.];

// blur the image using 9 components
bokeh_blur(&mut pixels, 3, 3, 1.0, &KERNEL9_PARAM_SET, 3.0);

// pixels now blurred
assert_eq!(
    vec![
        1.6428886692061846,
        14.80242203513296,
        1.6428886692061846,
        14.802422035132915,
        254.93338630375473,
        14.802422035132915,
        1.6428886692061846,
        14.80242203513296,
        1.6428886692061846
    ]
    .iter()
    .map(|&i| [i, i, i, i])
    .collect::<Vec<_>>(),
    pixels
);
```

A utility struct [`Image`] is also provided:
```rust
use bokeh::{Blur, Image, params::KERNEL9_PARAM_SET};

let mut pixels = vec![[0., 0., 0., 0.]; 9];
pixels[4] = [255., 255., 255., 255.];
// same as above but using the struct
let mut img = Image::new(&mut pixels, 3, 3);

img.bokeh_blur(1.0, &KERNEL9_PARAM_SET, 3.0);

assert_eq!(
    vec![
        1.6428886692061846,
        14.80242203513296,
        1.6428886692061846,
        14.802422035132915,
        254.93338630375473,
        14.802422035132915,
        1.6428886692061846,
        14.80242203513296,
        1.6428886692061846
    ]
    .iter()
    .map(|&i| [i, i, i, i])
    .collect::<Vec<_>>(),
    img.pixels
);
```

Providing a mask:
```rust
use bokeh::{Blur, Image, params::KERNEL9_PARAM_SET};

let mut pixels = vec![[0., 0., 0., 0.]; 9];
pixels[4] = [255., 255., 255., 255.];
// creating a mask
let mask = vec![false, true, false, true, false, true, false, true, false];
let mut img = Image::new(&mut pixels, 3, 3);

img.bokeh_blur_with_mask(&mask, 1.0, &KERNEL9_PARAM_SET, 3.0);

assert_eq!(
    vec![
        0.,
        14.80242203513296,
        0.,
        14.802422035132915,
        255.,
        14.802422035132915,
        0.,
        14.80242203513296,
        0.
    ]
    .iter()
    .map(|&i| [i, i, i, i])
    .collect::<Vec<_>>(),
    img.pixels
);
```
In the `assert!` statement above, comparing it to the previous example, it
can be seen that the original pixel values are retained.

License: MIT
