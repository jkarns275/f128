use f128_t::f128;
use libc::c_int;
use libc::c_longlong;

#[repr(C, align(16))]
pub(crate) struct Wrapper([u8; 16]);

#[link(name = "f128", kind = "static")]
extern "C" {
    pub fn f128_add(x: f128, y: f128) -> f128;
    pub fn f128_sub(x: f128, y: f128) -> f128;
    pub fn f128_div(x: f128, y: f128) -> f128;
    pub fn f128_mul(x: f128, y: f128) -> f128;
    pub fn f128_modulo(x: f128, y: f128) -> f128;

    pub fn qtostr(s: *mut u8, size: usize, fmt: *const i8, arg: f128) -> c_int;

    pub fn strtoflt128_f(c: *const i8) -> f128;

    pub(crate) fn usize_to_f128(x: usize) -> f128;
    pub(crate) fn f128_to_usize(x: f128) -> usize;
    pub(crate) fn u8_to_f128(x: u8) -> f128;
    pub(crate) fn f128_to_u8(x: f128) -> u8;
    pub(crate) fn u16_to_f128(x: u16) -> f128;
    pub(crate) fn f128_to_u16(x: f128) -> u16;
    pub(crate) fn u32_to_f128(x: u32) -> f128;
    pub(crate) fn f128_to_u32(x: f128) -> u32;
    pub(crate) fn u64_to_f128(x: u64) -> f128;
    pub(crate) fn f128_to_u64(x: f128) -> u64;
    pub(crate) fn u128_to_f128(x: Wrapper) -> f128;
    pub(crate) fn f128_to_u128(x: f128) -> Wrapper;
    pub(crate) fn isize_to_f128(x: isize) -> f128;
    pub(crate) fn f128_to_isize(x: f128) -> isize;
    pub(crate) fn i8_to_f128(x: i8) -> f128;
    pub(crate) fn f128_to_i8(x: f128) -> i8;
    pub(crate) fn i16_to_f128(x: i16) -> f128;
    pub(crate) fn f128_to_i16(x: f128) -> i16;
    pub(crate) fn i32_to_f128(x: i32) -> f128;
    pub(crate) fn f128_to_i32(x: f128) -> i32;
    pub(crate) fn i64_to_f128(x: i64) -> f128;
    pub(crate) fn f128_to_i64(x: f128) -> i64;
    pub(crate) fn i128_to_f128(x: Wrapper) -> f128;
    pub(crate) fn f128_to_i128(x: f128) -> Wrapper;
    pub(crate) fn f32_to_f128(x: f32) -> f128;
    pub(crate) fn f128_to_f32(x: f128) -> f32;
    pub(crate) fn f64_to_f128(x: f64) -> f128;
    pub(crate) fn f128_to_f64(x: f128) -> f64;

    pub fn acosq_f(x: f128) -> f128;
    pub fn acosn_f(a: f128) -> f128;
    pub fn acoshq_f(a: f128) -> f128;
    pub fn asinq_f(a: f128) -> f128;
    pub fn asinhq_f(a: f128) -> f128;
    pub fn atanq_f(a: f128) -> f128;
    pub fn atanhq_f(a: f128) -> f128;
    pub fn atan2q_f(a: f128, b: f128) -> f128;
    pub fn cbrtq_f(a: f128) -> f128;
    pub fn ceilq_f(a: f128) -> f128;
    pub fn copysignq_f(a: f128, b: f128) -> f128;
    pub fn coshq_f(a: f128) -> f128;
    pub fn cosq_f(a: f128) -> f128;
    pub fn erfq_f(a: f128) -> f128;
    pub fn erfcq_f(a: f128) -> f128;
    pub fn expq_f(a: f128) -> f128;
    pub fn expm1q_f(a: f128) -> f128;
    pub fn fabsq_f(a: f128) -> f128;
    pub fn fdimq_f(a: f128, b: f128) -> f128;
    pub fn finiteq_f(a: f128) -> c_int;
    pub fn floorq_f(a: f128) -> f128;
    pub fn fmaq_f(a: f128, b: f128, c: f128) -> f128;
    pub fn fmaxq_f(a: f128, b: f128) -> f128;
    pub fn fminq_f(a: f128, b: f128) -> f128;
    pub fn fmodq_f(a: f128, b: f128) -> f128;
    pub fn frexpq_f(a: f128, b: *mut c_int) -> f128;
    pub fn hypotq_f(a: f128, b: f128) -> f128;
    pub fn isinfq_f(a: f128) -> c_int;
    pub fn ilogbq_f(a: f128) -> c_int;
    pub fn isnanq_f(a: f128) -> c_int;
    pub fn j0q_f(a: f128) -> f128;
    pub fn j1q_f(a: f128) -> f128;
    pub fn jnq_f(a: c_int, b: f128) -> f128;
    pub fn ldexpq_f(a: f128, b: c_int) -> f128;
    pub fn lgammaq_f(a: f128) -> f128;
    pub fn llrintq_f(a: f128) -> c_longlong;
    pub fn llroundq_f(a: f128) -> c_longlong;
    // UNUSED
    // pub fn logbq_f(a: f128) -> f128;
    pub fn logq_f(a: f128) -> f128;
    pub fn log10q_f(a: f128) -> f128;
    pub fn log2q_f(a: f128) -> f128;
    pub fn log1pq_f(a: f128) -> f128;
    pub fn lrintq_f(a: f128) -> c_longlong;
    pub fn lroundq_f(a: f128) -> c_longlong;
    pub fn modfq_f(a: f128, b: *mut f128) -> f128;
    pub fn nanq_f(a: *mut u8) -> f128;
    pub fn nearbyintq_f(a: f128) -> f128;
    pub fn nextafterq_f(a: f128, b: f128) -> f128;
    pub fn powq_f(a: f128, b: f128) -> f128;
    pub fn remainderq_f(a: f128, b: f128) -> f128;
    pub fn remquoq_f(a: f128, b: f128, c: *mut c_int) -> f128;
    pub fn rintq_f(a: f128) -> f128;
    pub fn roundq_f(a: f128) -> f128;
    pub fn scalblnq_f(a: f128, b: c_int) -> f128;
    pub fn scalbnq_f(a: f128, b: c_int) -> f128;
    pub fn signbitq_f(a: f128) -> c_int;
    pub fn sincosq_f(a: f128, b: *mut f128, c: *mut f128) -> f128;
    pub fn sinhq_f(a: f128) -> f128;
    pub fn sinq_f(a: f128) -> f128;
    pub fn sqrtq_f(a: f128) -> f128;
    pub fn tanq_f(a: f128) -> f128;
    pub fn tanhq_f(a: f128) -> f128;
    pub fn tgammaq_f(a: f128) -> f128;
    pub fn truncq_f(a: f128) -> f128;
    pub fn y0q_f(a: f128) -> f128;
    pub fn y1q_f(a: f128) -> f128;
    pub fn ynq_f(a: c_int, b: f128) -> f128;

    pub fn gtq(lhs: f128, rhs: f128) -> u8;
    pub fn gteq(lhs: f128, rhs: f128) -> u8;
    pub fn eqq(lhs: f128, rhs: f128) -> u8;
    pub fn neqq(lhs: f128, rhs: f128) -> u8;
    pub fn ltq(lhs: f128, rhs: f128) -> u8;
    pub fn lteq(lhs: f128, rhs: f128) -> u8;

}
