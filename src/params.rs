const KERNEL1_PARAMS: [f64; 4 * 1] = [0.862325, 1.624835, 0.767583, 1.862321];
const KERNEL2_PARAMS: [f64; 4 * 2] = [
    0.886528, 5.268909, 0.411259, -0.548794, 1.960518, 1.558213, 0.513282, 4.561110,
];
const KERNEL3_PARAMS: [f64; 4 * 3] = [
    2.176490, 5.043495, 1.621035, -2.105439, 1.019306, 9.027613, -0.280860, -0.162882, 2.815110,
    1.597273, -0.366471, 10.300301,
];
const KERNEL4_PARAMS: [f64; 4 * 4] = [
    4.338459, 1.553635, -5.767909, 46.164397, 3.839993, 4.693183, 9.795391, -15.227561, 2.791880,
    8.178137, -3.048324, 0.302959, 1.342190, 12.328289, 0.010001, 0.244650,
];
const KERNEL5_PARAMS: [f64; 4 * 5] = [
    4.892608, 1.685979, -22.356787, 85.912460, 4.711870, 4.998496, 35.918936, -28.875618, 4.052795,
    8.244168, -13.212253, -1.578428, 2.929212, 11.900859, 0.507991, 1.816328, 1.512961, 16.116382,
    0.138051, -0.010000,
];
const KERNEL6_PARAMS: [f64; 4 * 6] = [
    5.143778,
    2.079813,
    -82.326596,
    111.231024,
    5.612426,
    6.153387,
    113.878661,
    58.004879,
    5.982921,
    9.802895,
    39.479083,
    -162.028887,
    6.505167,
    11.059237,
    -71.286026,
    95.027069,
    3.869579,
    14.810520,
    1.405746,
    -3.704914,
    2.201904,
    19.032909,
    -0.152784,
    -0.107988,
];

pub struct KernelParamSet<'a> {
    params: &'a [f64],
}
impl KernelParamSet<'_> {
    pub(crate) const fn a(&self, index: usize) -> f64 {
        self.params[4 * index]
    }
    pub(crate) const fn b(&self, index: usize) -> f64 {
        self.params[4 * index + 1]
    }
    #[allow(non_snake_case)]
    pub(crate) const fn A(&self, index: usize) -> f64 {
        self.params[4 * index + 2]
    }
    #[allow(non_snake_case)]
    pub(crate) const fn B(&self, index: usize) -> f64 {
        self.params[4 * index + 3]
    }
    pub(crate) const fn num_kernels(&self) -> usize {
        self.params.len() / 4
    }
}

pub const KERNEL1_PARAM_SET: KernelParamSet<'static> = KernelParamSet {
    params: &KERNEL1_PARAMS,
};
pub const KERNEL2_PARAM_SET: KernelParamSet<'static> = KernelParamSet {
    params: &KERNEL2_PARAMS,
};
pub const KERNEL3_PARAM_SET: KernelParamSet<'static> = KernelParamSet {
    params: &KERNEL3_PARAMS,
};
pub const KERNEL4_PARAM_SET: KernelParamSet<'static> = KernelParamSet {
    params: &KERNEL4_PARAMS,
};
pub const KERNEL5_PARAM_SET: KernelParamSet<'static> = KernelParamSet {
    params: &KERNEL5_PARAMS,
};
pub const KERNEL6_PARAM_SET: KernelParamSet<'static> = KernelParamSet {
    params: &KERNEL6_PARAMS,
};
