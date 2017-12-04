use libc::c_int;
use libc::c_longlong;
use f128_t::f128;


#[link_args = "-lquadmath"]
#[link(name = "f128", kind = "static")]
extern "C" {
    // Elementary methods
    pub fn f128_add(x: [u8; 16], y: [u8; 16]) -> [u8; 16];
    pub fn f128_sub(x: [u8; 16], y: [u8; 16]) -> [u8; 16];
    pub fn f128_div(x: [u8; 16], y: [u8; 16]) -> [u8; 16];
    pub fn f128_mul(x: [u8; 16], y: [u8; 16]) -> [u8; 16];
    pub fn f128_modulo(x: [u8; 16], y: [u8; 16]) -> [u8; 16];

    pub fn qtostr(s: *mut u8, size: usize, fmt: *const i8, arg: [u8; 16]) -> c_int;

    pub fn strtoflt128_f(c: *const i8) -> [u8; 16];

    pub fn usize_to_f128	(x: usize) -> [u8; 16];
    pub fn f128_to_usize	(x: [u8; 16]) -> usize;
    pub fn u8_to_f128	(x: u8) -> [u8; 16];
    pub fn f128_to_u8	(x: [u8; 16]) -> u8;
    pub fn u16_to_f128	(x: u16) -> [u8; 16];
    pub fn f128_to_u16	(x: [u8; 16]) -> u16;
    pub fn u32_to_f128	(x: u32) -> [u8; 16];
    pub fn f128_to_u32	(x: [u8; 16]) -> u32;
    pub fn u64_to_f128	(x: u64) -> [u8; 16];
    pub fn f128_to_u64	(x: [u8; 16]) -> u64;
    pub fn u128_to_f128	(x: [u8; 16]) -> [u8; 16];
    pub fn f128_to_u128	(x: [u8; 16]) -> [u8; 16];
    pub fn isize_to_f128	(x: [u8; 16]) -> [u8; 16];
    pub fn f128_to_isize	(x: [u8; 16]) -> [u8; 16];
    pub fn i8_to_f128	(x: i8) -> [u8; 16];
    pub fn f128_to_i8	(x: [u8; 16]) -> i8;
    pub fn i16_to_f128	(x: i16) -> [u8; 16];
    pub fn f128_to_i16	(x: [u8; 16]) -> i16;
    pub fn i32_to_f128	(x: i32) -> [u8; 16];
    pub fn f128_to_i32	(x: [u8; 16]) -> i32;
    pub fn i64_to_f128	(x: i64) -> [u8; 16];
    pub fn f128_to_i64	(x: [u8; 16]) -> i64;
    pub fn i128_to_f128	(x: [u8; 16]) -> [u8; 16];
    pub fn f128_to_i128(x: [u8; 16]) -> [u8; 16];
    pub fn f32_to_f128	(x: f32) -> [u8; 16];
    pub fn f128_to_f32	(x: [u8; 16]) -> f32;
    pub fn f64_to_f128	(x: f64) -> [u8; 16];
    pub fn f128_to_f64	(x: [u8; 16]) -> f64;

    pub fn acosq(x: [u8; 16]) -> [u8; 16];
    pub fn acosn_f (a: [u8; 16]) -> [u8; 16];
    pub fn acoshq_f (a: [u8; 16]) -> [u8; 16];
    pub fn asinq_f (a: [u8; 16]) -> [u8; 16];
    pub fn asinhq_f (a: [u8; 16]) -> [u8; 16];
    pub fn atanq_f (a: [u8; 16]) -> [u8; 16];
    pub fn atanhq_f (a: [u8; 16]) -> [u8; 16];
    pub fn atan2q_f (a: [u8; 16], b: [u8; 16]) -> [u8; 16];
    pub fn cbrtq_f (a: [u8; 16]) -> [u8; 16];
    pub fn ceilq_f (a: [u8; 16]) -> [u8; 16];
    pub fn copysignq_f (a: [u8; 16], b: [u8; 16]) -> [u8; 16];
    pub fn coshq_f (a: [u8; 16]) -> [u8; 16];
    pub fn cosq_f (a: [u8; 16]) -> [u8; 16];
    pub fn erfq_f (a: [u8; 16]) -> [u8; 16];
    pub fn erfcq_f (a: [u8; 16]) -> [u8; 16];
    pub fn expq_f (a: [u8; 16]) -> [u8; 16];
    pub fn expm1q_f (a: [u8; 16]) -> [u8; 16];
    pub fn fabsq_f (a: [u8; 16]) -> [u8; 16];
    pub fn fdimq_f (a: [u8; 16], b: [u8; 16]) -> [u8; 16];
    pub fn finiteq_f (a: [u8; 16]) -> c_int;
    pub fn floorq_f (a: [u8; 16]) -> [u8; 16];
    pub fn fmaq_f (a: [u8; 16], b: [u8; 16], c: [u8; 16]) -> [u8; 16];
    pub fn fmaxq_f (a: [u8; 16], b: [u8; 16]) -> [u8; 16];
    pub fn fminq_f (a: [u8; 16], b: [u8; 16]) -> [u8; 16];
    pub fn fmodq_f (a: [u8; 16], b: [u8; 16]) -> [u8; 16];
    pub fn frexpq_f (a: [u8; 16], b: *mut c_int) -> [u8; 16];
    pub fn hypotq_f (a: [u8; 16], b: [u8; 16]) -> [u8; 16];
    pub fn isinfq_f (a: [u8; 16]) -> c_int;
    pub fn ilogbq_f (a: [u8; 16]) -> c_int;
    pub fn isnanq_f (a: [u8; 16]) -> c_int;
    pub fn j0q_f (a: [u8; 16]) -> [u8; 16];
    pub fn j1q_f (a: [u8; 16]) -> [u8; 16];
    pub fn jnq_f (a: c_int, b: [u8; 16]) -> [u8; 16];
    pub fn ldexpq_f (a: [u8; 16], b: c_int) -> [u8; 16];
    pub fn lgammaq_f (a: [u8; 16]) -> [u8; 16];
    pub fn llrintq_f (a: [u8; 16]) -> c_longlong;
    pub fn llroundq_f (a: [u8; 16]) -> c_longlong;
    pub fn logbq_f (a: [u8; 16]) -> [u8; 16];
    pub fn logq_f (a: [u8; 16]) -> [u8; 16];
    pub fn log10q_f (a: [u8; 16]) -> [u8; 16];
    pub fn log2q_f (a: [u8; 16]) -> [u8; 16];
    pub fn log1pq_f (a: [u8; 16]) -> [u8; 16];
    pub fn lrintq_f (a: [u8; 16]) -> c_longlong;
    pub fn lroundq_f (a: [u8; 16]) -> c_longlong;
    pub fn modfq_f (a: [u8; 16], b: *mut [u8; 16]) -> [u8; 16];
    pub fn nanq_f (a: *mut u8) -> [u8; 16];
    pub fn nearbyintq_f (a: [u8; 16]) -> [u8; 16];
    pub fn nextafterq_f (a: [u8; 16], b: [u8; 16]) -> [u8; 16];
    pub fn powq_f (a: [u8; 16], b: [u8; 16]) -> [u8; 16];
    pub fn remainderq_f (a: [u8; 16], b: [u8; 16]) -> [u8; 16];
    pub fn remquoq_f (a: [u8; 16], b: [u8; 16], c: *mut c_int) -> [u8; 16];
    pub fn rintq_f (a: [u8; 16]) -> [u8; 16];
    pub fn roundq_f (a: [u8; 16]) -> [u8; 16];
    pub fn scalblnq_f (a: [u8; 16], b: c_int) -> [u8; 16];
    pub fn scalbnq_f (a: [u8; 16], b: c_int) -> [u8; 16];
    pub fn signbitq_f (a: [u8; 16]) -> c_int;
    pub fn sincosq_f (a: [u8; 16], b: *mut [u8; 16], c: *mut [u8; 16]) -> [u8; 16];
    pub fn sinhq_f (a: [u8; 16]) -> [u8; 16];
    pub fn sinq_f (a: [u8; 16]) -> [u8; 16];
    pub fn sqrtq_f (a: [u8; 16]) -> [u8; 16];
    pub fn tanq_f (a: [u8; 16]) -> [u8; 16];
    pub fn tanhq_f (a: [u8; 16]) -> [u8; 16];
    pub fn tgammaq_f (a: [u8; 16]) -> [u8; 16];
    pub fn truncq_f (a: [u8; 16]) -> [u8; 16];
    pub fn y0q_f (a: [u8; 16]) -> [u8; 16];
    pub fn y1q_f (a: [u8; 16]) -> [u8; 16];
    pub fn ynq_f (a: c_int, b: [u8; 16]) -> [u8; 16];
}
