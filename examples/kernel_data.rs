use bokeh::{kernel_gaussian_components, params::*};
use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut args = env::args();
    args.next();
    let out_dir = args.next().unwrap_or("plots".to_string());

    for (p, params) in [
        KERNEL1_PARAM_SET,
        KERNEL2_PARAM_SET,
        KERNEL3_PARAM_SET,
        KERNEL4_PARAM_SET,
        KERNEL5_PARAM_SET,
        KERNEL6_PARAM_SET,
        KERNEL7_PARAM_SET,
        KERNEL8_PARAM_SET,
        KERNEL9_PARAM_SET,
    ]
    .iter()
    .enumerate()
    {
        for kernel_size in [1, 5, 10, 50, 100] {
            let kernels = kernel_gaussian_components(params, kernel_size as f64);

            let mut output = vec![0.0; kernels[0].len()];
            for (n, kernel) in kernels.iter().enumerate() {
                for (m, k) in kernel.iter().enumerate() {
                    output[m] += params.real_component(n) * k.re + params.imag_component(n) * k.im;
                }

                let mut file = File::create(format!("{}/{}_{}.json", out_dir, p + 1, kernel_size))?;
                write!(file, "{:?}", output)?;
            }
        }
    }

    Ok(())
}
