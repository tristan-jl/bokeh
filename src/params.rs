//! Parameters for the bokeh kernels
//!
//! These are used to approximate a disc shaped kernel through summing Gaussian
//! kernels
//!
//! Generated by Mike Pound, found [here](https://github.com/mikepound/convolve/blob/7f579ada8ab8c426cc157bf5f200a94dfdb50830/complex_kernels.py) and [here](https://github.com/mikepound/convolve/issues/2)
use paste::paste;

const KERNEL1_PARAMS: [f64; 4] = [0.862325, 1.624835, 0.767583, 1.862321];
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
const KERNEL7_PARAMS: [f64; 4 * 7] = [
    5.635755002716984,
    2.0161846499938942,
    -127.67050821204298,
    189.13366250400748,
    6.2265180958586,
    6.010948636588568,
    255.34251414243556,
    37.55094949608352,
    6.189230711552051,
    8.269383035533139,
    -132.2590521372958,
    -101.7059257653572,
    4.972166727344845,
    12.050001393751478,
    -0.1843113559893084,
    27.06823846423038,
    4.323578237784037,
    16.00101043380645,
    5.837168074459592,
    0.3359847314948253,
    3.6920668221834534,
    19.726797144782385,
    0.010115759114852045,
    -1.091291088554394,
    2.2295702188720004,
    23.527764286361837,
    -0.07655024461742256,
    0.01001768577317681,
];
const KERNEL8_PARAMS: [f64; 4 * 8] = [
    6.6430131554059075,
    2.33925731610851,
    -665.7557728544768,
    445.83362839529286,
    8.948432332999396,
    5.775418437190626,
    1130.5906034230607,
    15.626805026300797,
    6.513143649767612,
    8.05507417830653,
    -419.50196449095665,
    -9.275778572724292,
    6.245927989258722,
    12.863350894308521,
    -100.85574814870866,
    79.1599400003683,
    6.713191682126933,
    17.072272272191718,
    36.65346659449611,
    118.71908139892597,
    7.071814347005397,
    18.719212513078034,
    21.63902100281763,
    -77.52385953960055,
    4.932882961391405,
    22.545463415981025,
    -1.9683109176118303,
    3.0163201264848736,
    3.456372395841802,
    26.088356168016503,
    0.19835893874241894,
    0.08089803872063023,
];
const KERNEL9_PARAMS: [f64; 4 * 9] = [
    7.393797857697906,
    2.4737002456790207,
    -1796.6881230069646,
    631.9043430000561,
    13.246479495224113,
    6.216076882495199,
    3005.0995149934884,
    169.0878309991149,
    7.303628653874887,
    7.783952969919921,
    -1058.5279460078423,
    459.6898389991668,
    8.154742557454425,
    13.430399706116823,
    -1720.108330007715,
    810.6026949975844,
    8.381657431347698,
    14.90360902110027,
    1568.5705749924186,
    285.01830799719926,
    6.866935986644192,
    20.281841043506173,
    90.55436499314388,
    -59.610040004419275,
    9.585395987559902,
    21.80265398520623,
    -93.26089100639886,
    -111.18596800373774,
    5.4836869943565825,
    25.89243600015612,
    5.110650995956478,
    0.009999997374460896,
    5.413819000655994,
    28.96548499880915,
    0.2499879943861626,
    -0.8591239990799346,
];

/// Utility struct for holding and retrieving kernel parameters
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
    pub(crate) const fn real_component(&self, index: usize) -> f64 {
        self.params[4 * index + 2]
    }
    pub(crate) const fn imag_component(&self, index: usize) -> f64 {
        self.params[4 * index + 3]
    }
    pub(crate) const fn num_kernels(&self) -> usize {
        self.params.len() / 4
    }
}

macro_rules! param_set {
    ($n:expr) => {
        paste! {
            #[doc=concat!("Parameter set for the ", $n ,"-component kernel")]
            pub const [<KERNEL $n _PARAM_SET>]: KernelParamSet<'static> = KernelParamSet {
                params: &[<KERNEL $n _PARAMS>],
            };

        }
    };
}

param_set!(1);
param_set!(2);
param_set!(3);
param_set!(4);
param_set!(5);
param_set!(6);
param_set!(7);
param_set!(8);
param_set!(9);
