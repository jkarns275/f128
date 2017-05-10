use libc::c_int;
use libc::c_longlong;
use f128_t::f128;

macro_rules! import_into_f128_gen {
    ($($t:ident)*) => ($(
        {
            pub fn concat_idents!($t, _to_i128)(x: $t) -> i128;
            pub fn concat_idents!(f128_to_, $t)(x: i128) -> $t;
        }
    )*)
}

#[link_args = "-l quadmath"]
#[link(name = "f128", kind = "static")]
extern {
    // Elementary methods
    pub fn f128_add(x: i128, y: i128) -> i128;
    pub fn f128_sub(x: i128, y: i128) -> i128;
    pub fn f128_div(x: i128, y: i128) -> i128;
    pub fn f128_mul(x: i128, y: i128) -> i128;
    pub fn f128_modulo(x: i128, y: i128) -> i128;

    pub fn qtostr(s: *mut u8, size: usize, fmt: *const i8, arg: i128) -> c_int;

    pub fn strtoflt128_f(c: *const i8) -> i128;

    pub fn usize_to_f128	(x: usize) -> i128;
    pub fn f128_to_usize	(x: i128) -> usize;
    pub fn u8_to_f128	(x: u8) -> i128;
    pub fn f128_to_u8	(x: i128) -> u8;
    pub fn u16_to_f128	(x: u16) -> i128;
    pub fn f128_to_u16	(x: i128) -> u16;
    pub fn u32_to_f128	(x: u32) -> i128;
    pub fn f128_to_u32	(x: i128) -> u32;
    pub fn u64_to_f128	(x: u64) -> i128;
    pub fn f128_to_u64	(x: i128) -> u64;
    pub fn u128_to_f128	(x: u128) -> i128;
    pub fn f128_to_u128	(x: i128) -> u128;
    pub fn isize_to_f128	(x: isize) -> i128;
    pub fn f128_to_isize	(x: i128) -> isize;
    pub fn i8_to_f128	(x: i8) -> i128;
    pub fn f128_to_i8	(x: i128) -> i8;
    pub fn i16_to_f128	(x: i16) -> i128;
    pub fn f128_to_i16	(x: i128) -> i16;
    pub fn i32_to_f128	(x: i32) -> i128;
    pub fn f128_to_i32	(x: i128) -> i32;
    pub fn i64_to_f128	(x: i64) -> i128;
    pub fn f128_to_i64	(x: i128) -> i64;
    pub fn i128_to_f128	(x: i128) -> i128;
    pub fn f128_to_i128	(x: i128) -> i128;
    pub fn f32_to_f128	(x: f32) -> i128;
    pub fn f128_to_f32	(x: i128) -> f32;
    pub fn f64_to_f128	(x: f64) -> i128;
    pub fn f128_to_f64	(x: i128) -> f64;



    pub fn acosq(x: i128) -> i128;
    pub fn acosn_f (a: i128) -> i128;
    pub fn acoshq_f (a: i128) -> i128;
    pub fn asinq_f (a: i128) -> i128;
    pub fn asinhq_f (a: i128) -> i128;
    pub fn atanq_f (a: i128) -> i128;
    pub fn atanhq_f (a: i128) -> i128;
    pub fn atan2q_f (a: i128, b: i128) -> i128;
    pub fn cbrtq_f (a: i128) -> i128;
    pub fn ceilq_f (a: i128) -> i128;
    pub fn copysignq_f (a: i128, b: i128) -> i128;
    pub fn coshq_f (a: i128) -> i128;
    pub fn cosq_f (a: i128) -> i128;
    pub fn erfq_f (a: i128) -> i128;
    pub fn erfcq_f (a: i128) -> i128;
    pub fn expq_f (a: i128) -> i128;
    pub fn expm1q_f (a: i128) -> i128;
    pub fn fabsq_f (a: i128) -> i128;
    pub fn fdimq_f (a: i128, b: i128) -> i128;
    pub fn finiteq_f (a: i128) -> c_int;
    pub fn floorq_f (a: i128) -> i128;
    pub fn fmaq_f (a: i128, b: i128, c: i128) -> i128;
    pub fn fmaxq_f (a: i128, b: i128) -> i128;
    pub fn fminq_f (a: i128, b: i128) -> i128;
    pub fn fmodq_f (a: i128, b: i128) -> i128;
    pub fn frexpq_f (a: i128, b: *mut c_int) -> i128;
    pub fn hypotq_f (a: i128, b: i128) -> i128;
    pub fn isinfq_f (a: i128) -> c_int;
    pub fn ilogbq_f (a: i128) -> c_int;
    pub fn isnanq_f (a: i128) -> c_int;
    pub fn j0q_f (a: i128) -> i128;
    pub fn j1q_f (a: i128) -> i128;
    pub fn jnq_f (a: c_int, b: i128) -> i128;
    pub fn ldexpq_f (a: i128, b: c_int) -> i128;
    pub fn lgammaq_f (a: i128) -> i128;
    pub fn llrintq_f (a: i128) -> c_longlong;
    pub fn llroundq_f (a: i128) -> c_longlong;
    pub fn logbq_f (a: i128) -> i128;
    pub fn logq_f (a: i128) -> i128;
    pub fn log10q_f (a: i128) -> i128;
    pub fn log2q_f (a: i128) -> i128;
    pub fn log1pq_f (a: i128) -> i128;
    pub fn lrintq_f (a: i128) -> c_longlong;
    pub fn lroundq_f (a: i128) -> c_longlong;
    pub fn modfq_f (a: i128, b: *mut i128) -> i128;
    pub fn nanq_f (a: *mut u8) -> i128;
    pub fn nearbyintq_f (a: i128) -> i128;
    pub fn nextafterq_f (a: i128, b: i128) -> i128;
    pub fn powq_f (a: i128, b: i128) -> i128;
    pub fn remainderq_f (a: i128, b: i128) -> i128;
    pub fn remquoq_f (a: i128, b: i128, c: *mut c_int) -> i128;
    pub fn rintq_f (a: i128) -> i128;
    pub fn roundq_f (a: i128) -> i128;
    pub fn scalblnq_f (a: i128, b: c_int) -> i128;
    pub fn scalbnq_f (a: i128, b: c_int) -> i128;
    pub fn signbitq_f (a: i128) -> c_int;
    pub fn sincosq_f (a: i128, b: *mut i128, c: *mut i128) -> i128;
    pub fn sinhq_f (a: i128) -> i128;
    pub fn sinq_f (a: i128) -> i128;
    pub fn sqrtq_f (a: i128) -> i128;
    pub fn tanq_f (a: i128) -> i128;
    pub fn tanhq_f (a: i128) -> i128;
    pub fn tgammaq_f (a: i128) -> i128;
    pub fn truncq_f (a: i128) -> i128;
    pub fn y0q_f (a: i128) -> i128;
    pub fn y1q_f (a: i128) -> i128;
    pub fn ynq_f (a: c_int, b: i128) -> i128;
}
