use std::ops::*;
use ffi;
use ffi::*;
use std::convert::{ From, Into };
use std::iter::*;
use std::hash::{ Hash, Hasher };
use std::mem;
use std::slice;
use std::str;
use std::io::Write;
use std::ffi::CString;
use std::ffi::NulError;
use f128_derive::*;
use std::num::FpCategory;
use num_traits::*;
use libc::c_int;

macro_rules! f128_from_x {
    ($x: ty, $n: expr, $it: expr) => ({
        // 32 is ascii space, so this buff will be filled with spaces after the number
        let mut buf: [u8; $n] = [32u8; $n];
        write!(&mut buf[..], "{}", $it).expect("Failed to write integer to buffer");
        f128::parse(str::from_utf8(&buf[..]).unwrap()).ok()
    })
}

#[derive(Clone, Copy)]
pub struct f128(pub(crate) [u8; 16]);

pub trait To16Bytes {
    fn to_arr(&self) -> [u8; 16];
    fn to_u128(&self) -> u128;
    fn to_i128(&self) -> i128;
}

impl f128 {
    pub const RADIX: u32 = 128;
    pub const MANTISSA_DIGITS: u32 = 112;

    pub const MAX_10_EXP: u32 = 4932;
    pub const MAX_EXP: u32 = 16383;
    pub const MIN_10_EXP: i32 = -4931;
    pub const MIN_EXP: i32 = -16382;
    pub const ZERO: f128 = f128([0; 16]);


    #[cfg(target_endian = "big")]
    pub const SIGN_BIT: f128 = f128([0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    #[cfg(target_endian = "little")]
    pub const SIGN_BIT: f128 = f128([0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80]);

    #[cfg(target_endian = "big")]
    pub const EXPONENT_BITS: f128 = f128([0x7f, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    #[cfg(target_endian = "little")]
    pub const EXPONENT_BITS: f128 = f128([0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff, 0x7f]);

    #[cfg(target_endian = "big")]
    pub const FRACTION_BITS: f128 = f128([0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
    #[cfg(target_endian = "little")]
    pub const FRACTION_BITS: f128 = f128([0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00]);

    #[cfg(target_endian = "big")]
    pub const MIN: f128 = f128([0xff, 0xfe, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff]);
    #[cfg(target_endian = "little")]
    pub const MIN: f128 = f128([0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xfe, 0xff]);

    #[cfg(target_endian = "big")]
    pub const MIN_POSITIVE: f128 = f128([0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01]);
    #[cfg(target_endian = "little")]
    pub const MIN_POSITIVE: f128 = f128([0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);

    #[cfg(target_endian = "big")]
    pub const ONE: f128 = f128([0x3f, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    #[cfg(target_endian = "little")]
    pub const ONE: f128 = f128([0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff, 0x3f]);

    #[cfg(target_endian = "big")]
    pub const TWO: f128 = f128([0x40, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    #[cfg(target_endian = "little")]
    pub const TWO: f128 = f128([0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40]);

    #[cfg(target_endian = "big")]
    pub const E: f128 = f128([0x40, 0x00, 0x5b, 0xf0, 0xa8, 0xb1, 0x45, 0x76, 0x95, 0x35, 0x5f, 0xb8, 0xac, 0x40, 0x4e, 0x7a]);
    #[cfg(target_endian = "little")]
    pub const E: f128 = f128([0x7a, 0x4e, 0x40, 0xac, 0xb8, 0x5f, 0x35, 0x95, 0x76, 0x45, 0xb1, 0xa8, 0xf0, 0x5b, 0x00, 0x40]);

    #[cfg(target_endian = "big")]
    pub const PI: f128 = f128([0x40, 0x00, 0x92, 0x1f, 0xb5, 0x44, 0x42, 0xd1, 0x84, 0x69, 0x89, 0x8c, 0xc5, 0x17, 0x01, 0xb8]);
    #[cfg(target_endian = "little")]
    pub const PI: f128 = f128([0xb8, 0x01, 0x17, 0xc5, 0x8c, 0x89, 0x69, 0x84, 0xd1, 0x42, 0x44, 0xb5, 0x1f, 0x92, 0x00, 0x40]);

    #[cfg(target_endian = "little")]
    pub const INFINITY: f128 = f128([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xff, 0x7f]);
    #[cfg(target_endian = "big")]
    pub const INFINITY: f128 = f128([0x7F, 0xFF, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);

    #[cfg(target_endian = "big")]
    pub const NAN: f128 = f128([0x7F, 0xFF, 0xFF,  0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFFu8, 0xFFu8, 0xFF ]);
    #[cfg(target_endian = "little")]
    pub const NAN: f128 = f128([ 0xFF, 0xFF, 0xFF,  0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFFu8, 0xFFu8, 0x7F ]);

    #[cfg(target_endian = "little")]
    pub const NEG_INFINITY: f128 = f128([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xff, 0xff]);
    #[cfg(target_endian = "big")]
    pub const NEG_INFINITY: f128 = f128([0xff, 0xff, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);

    pub(crate) fn from_arr(d: [u8; 16]) -> Self { f128(d) }

    #[inline(always)]
    pub fn from_raw_u128(d: u128) -> Self { f128::from_arr(unsafe { mem::transmute::<u128, [u8; 16]>(d) }) }
    #[inline(always)]
    pub fn from_raw_i128(d: i128) -> Self { f128::from_arr(unsafe { mem::transmute::<i128, [u8; 16]>(d) }) }

    #[inline(always)]
    pub fn inner_as_i128(self) -> i128 { unsafe { mem::transmute::<[u8; 16], i128>(self.0) } }

    #[inline(always)]
    pub fn inner_as_u128(&self) -> u128 { unsafe { mem::transmute::<[u8; 16], u128>(self.0) } }

    #[inline(always)]
    pub fn from_bits<T: To16Bytes>(x: &To16Bytes) -> Self {
        f128( x.to_arr() )
    }

    #[inline(always)]
    pub fn new<T: Into<f128>>(a: T) -> Self { a.into() }

    pub fn to_string(&self) -> String {
        self.to_string_fmt("%.36Qg").unwrap()
    }

    pub fn to_string_fmt<T: AsRef<str>>(&self, fmt: T) -> Option<String> {
        let mut buf = [0u8; 128];
        let cstr;
        match CString::new(fmt.as_ref()) {
            Ok(e) => cstr = e,
            Err(_) => return None
        };
        let n = unsafe { qtostr((&mut buf).as_mut_ptr(), 128, cstr.as_ptr(), self.inner()) };
        let mut v = Vec::with_capacity(n as usize);
        for i in 0..n {
            v.push(buf[i as usize]);
        }
        Some(String::from_utf8(v).unwrap())
    }

    #[inline(always)]
    pub fn inner(&self) -> [u8; 16] { self.0 }

    #[inline(always)]
    pub fn into_inner(self) -> [u8; 16] { self.0 }

    pub fn parse<T: AsRef<str>>(s: T) -> Result<Self, NulError> {
        let cstr = CString::new(s.as_ref())?;
        let result = unsafe { strtoflt128_f(cstr.as_ptr()) };

        Ok(unsafe { f128(strtoflt128_f(cstr.as_ptr()))})
    }

    pub fn exp_bits(&self) -> u32 {
        ((self.inner_as_u128() & f128::EXPONENT_BITS.inner_as_u128()) >> 112) as u32
    }

    pub fn fract_bits(&self) -> u128 {
        self.inner_as_u128() & f128::FRACTION_BITS.inner_as_u128()
    }
}

impl Zero for f128 {
    fn is_zero(&self) -> bool { self.0 == f128::ZERO.0 }
    fn zero() -> Self { f128::ZERO }
}

impl One for f128 {
    fn one() -> Self { f128::ONE }
}

impl ToPrimitive for f128 {
    fn to_i64(&self)    -> Option<i64> { Some(unsafe { f128_to_i64(self.inner()) }) }
    fn to_u64(&self)    -> Option<u64> { Some(unsafe { f128_to_u64(self.inner()) }) }
    fn to_isize(&self)  -> Option<isize> { Some(unsafe { f128_to_i64(self.inner()) as isize }) }
    fn to_i8(&self)     -> Option<i8> { Some(unsafe { f128_to_i8(self.inner()) }) }
    fn to_i16(&self)    -> Option<i16> { Some(unsafe { f128_to_i16(self.inner()) }) }
    fn to_i32(&self)    -> Option<i32> { Some(unsafe { f128_to_i32(self.inner()) }) }
    fn to_usize(&self)  -> Option<usize> { Some(unsafe { f128_to_u64(self.inner()) as usize }) }
    fn to_u8(&self)     -> Option<u8> { Some(unsafe { f128_to_u8(self.inner()) }) }
    fn to_u16(&self)    -> Option<u16> { Some(unsafe { f128_to_u16(self.inner()) }) }
    fn to_u32(&self)    -> Option<u32> { Some(unsafe { f128_to_u32(self.inner()) }) }
    fn to_f32(&self)    -> Option<f32> { Some(unsafe { f128_to_f32(self.inner()) }) }
    fn to_f64(&self)    -> Option<f64> { Some(unsafe { f128_to_f64(self.inner()) }) }
    fn to_i128(&self)   -> Option<i128> { Some(unsafe { mem::transmute::<[u8; 16], i128>(f128_to_i128(self.inner())) }) }
    fn to_u128(&self)   -> Option<u128> { Some(unsafe { mem::transmute::<[u8; 16], u128>(f128_to_u128(self.inner())) }) }
}

impl FromPrimitive for f128 {
    fn from_i64(n: i64) -> Option<Self> {
        f128_from_x!(i64, 32, n)
    }
    fn from_u64(n: u64) -> Option<Self> {
        f128_from_x!(u64, 32, n)
    }
    fn from_isize(n: isize) -> Option<Self> {
        f128_from_x!(isize, 32, n)
    }
    fn from_i8(n: i8) -> Option<Self> {
        f128_from_x!(i8, 4, n)
    }
    fn from_i16(n: i16) -> Option<Self> {
        f128_from_x!(i16, 16, n)
    }
    fn from_i32(n: i32) -> Option<Self> {
        f128_from_x!(i32, 16, n)
    }
    fn from_usize(n: usize) -> Option<Self> {
        f128_from_x!(usize, 32, n)
    }
    fn from_u8(n: u8) -> Option<Self> {
        f128_from_x!(u8, 4, n)
    }
    fn from_u16(n: u16) -> Option<Self> {
        f128_from_x!(u16, 16, n)
    }
    fn from_u32(n: u32) -> Option<Self> {
        f128_from_x!(u32, 16, n)
    }
    fn from_f32(n: f32) -> Option<Self> {
        // 32 is ascii space, so this buff will be filled with spaces after the number
        let mut buf: [u8; 64] = [32u8; 64];
        write!(&mut buf[..], "{:E}", n).expect("Failed to convert integer to string.");
        f128::parse(str::from_utf8(&buf[..]).unwrap()).ok()
    }
    fn from_f64(n: f64) -> Option<Self> {
        // 32 is ascii space, so this buff will be filled with spaces after the number
        let mut buf: [u8; 64] = [32u8; 64];
        write!(&mut buf[..], "{:E}", n).expect("Failed to convert integer to string.");
        f128::parse(str::from_utf8(&buf[..]).unwrap()).ok()
    }
    fn from_u128(n: u128) -> Option<Self> {
        f128_from_x!(u128, 64, n)
    }
    fn from_i128(n: i128) -> Option<Self> {
        f128_from_x!(i128, 64, n)
    }
}

impl Num for f128 {
    type FromStrRadixErr = ();
    fn from_str_radix(s: &str, radix: u32) -> Result<Self, ()> {
        unimplemented!()
    }
}

impl NumCast for f128 {
    fn from<T: ToPrimitive>(n: T) -> Option<Self> {
        match <f64 as NumCast>::from(n) {
            Some(i) => f128::from_f64(i),
            None => None
        }
    }
}

impl Float for f128 {
    fn nan() -> Self { f128::NAN }

    fn infinity() -> Self { f128::INFINITY }

    #[inline(always)]
    #[cfg(target_endian = "little")]
    fn neg_infinity() -> Self { f128([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xff, 0xff]) }
    #[inline(always)]
    #[cfg(target_endian = "big")]
    fn neg_infinity() -> Self { f128([0xff, 0xff, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) }

    #[inline(always)]
    #[cfg(target_endian = "little")]
    fn neg_zero() -> Self { f128([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x80]) }
    #[inline(always)]
    #[cfg(target_endian = "big")]
    fn neg_zero() -> Self { f128([0x80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) }


    #[cfg(target_endian = "little")]
    #[inline(always)]
    fn min_value() -> f128 {
        f128([0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xfe, 0xff])
    }
    #[cfg(target_endian = "big")]
    #[inline(always)]
    fn min_value() -> f128 {
        f128([0xff, 0xfe, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff])
    }

    #[cfg(target_endian = "little")]
    #[inline(always)]
    fn max_value() -> f128 {
        f128([0x7f, 0xfe, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff])
    }
    #[cfg(target_endian = "big")]
    #[inline(always)]
    fn max_value() -> f128 {
        f128([0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xfe, 0x7f])
    }

    fn min_positive_value() -> f128 { f128::MIN_POSITIVE }

    fn is_finite(self) -> bool {
        !self.is_infinite() && !self.is_nan()
    }

    fn is_infinite(self) -> bool {
        // It's fine to compare the bits here since there is only 1 bit pattern that is inf, and one
        // that is -inf.
        let res = (self.inner_as_u128() & 0x7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFu128);
        res == f128::EXPONENT_BITS.inner_as_u128()
    }

    fn is_nan(self) -> bool {
        (self.inner_as_u128() & 0x7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFu128) > f128::EXPONENT_BITS.inner_as_u128()
    }

    fn is_normal(self) -> bool {
        let exp = self.exp_bits();
        exp >= 0x0001u32 && exp <= 0x7FFEu32
    }

    fn classify(self) -> FpCategory {
        let x = (self.is_normal(), self.is_finite(), self.is_nan());
        match x {
            (true, true, false) => FpCategory::Normal,
            (false, true, false) => FpCategory::Subnormal,
            (_, _, true) => FpCategory::Nan,
            (_, false, _) => FpCategory::Infinite,
            _ => unreachable!()
        }
    }

    fn floor(self) -> Self {
        f128::from_arr(unsafe { floorq_f(self.0) })
    }

    fn ceil(self) -> Self {
        f128::from_arr(unsafe { ceilq_f(self.0) })
    }

    fn round(self) -> Self {
        f128::from_arr(unsafe { roundq_f(self.0) })
    }

    fn trunc(self) -> Self {
        f128::from_arr(unsafe { truncq_f(self.0) })
    }

    fn fract(self) -> Self {
        let mut x: c_int = 0;
        f128::from_arr(unsafe { frexpq_f(self.0, &mut x) })
    }

    #[cfg(target_endian = "big")]
    fn abs(mut self) -> Self {
        self.0[0] &= 0x7F;
        self
    }
    #[cfg(target_endian = "little")]
    fn abs(mut self) -> Self {
        self.0[15] &= 0x7F;
        self
    }

    fn signum(self) -> Self {
        match self.0[0] & 0x80 {
            0 => f128::INFINITY,
            1 => f128::NEG_INFINITY,
            _ => unreachable!()
        }
    }

    fn is_sign_negative(self) -> bool {
        match self.0[0] & 0x80 {
            0 => false,
            1 => true,
            _ => unreachable!()
        }
    }

    fn is_sign_positive(self) -> bool {
        match self.0[0] & 0x80 {
            1 => false,
            0 => true,
            _ => unreachable!()
        }
    }

    fn mul_add(self, a: f128, b: f128) -> f128 {
        f128::from_arr(unsafe { fmaq_f(self.0, a.0, b.0) })
    }

    fn recip(self) -> f128 {
        f128::ONE / self
    }

    fn powi(self, n: i32) -> f128 {
        let mut i = self.clone();
        if (n < 0) {
            for _ in n..0 {
                i /= self;
            }
        } else {
            for _ in 0..n {
                i *= self;
            }
        }
        i
    }

    fn powf(self, n: f128) -> f128 {
        f128::from_arr(unsafe { powq_f(self.0, n.0) })
    }

    fn sqrt(self) -> f128 {
        f128::from_arr(unsafe { sqrtq_f(self.0) })
    }

    fn exp(self) -> f128 {
        f128::from_arr(unsafe { expq_f(self.0) })
    }

    fn exp2(self) -> f128 {
        // TODO: Change this to a constant two
        (f128::ONE * f128::from_u8(2).unwrap()).powf(self)
    }

    fn ln(self) -> f128 { f128::from_arr( unsafe { logq_f(self.0) }) }

    fn log(self, base: f128) -> f128 {
        // Change of base formula
        let numr = self.ln();
        let denm = base.ln();
        numr / denm
    }

    fn log2(self) -> f128 { f128::from_arr( unsafe { log2q_f(self.0) }) }

    fn log10(self) -> f128 { f128::from_arr( unsafe { log10q_f(self.0) }) }

    fn max(self, other: f128) -> f128 {
        unsafe {
            let a = mem::transmute::<f128, i128>(self);
            let b = mem::transmute::<f128, i128>(other);
            mem::transmute::<i128, f128>(if a > b { a } else { b })
        }
    }

    fn min(self, other: f128) -> f128 {
        unsafe {
            let a = mem::transmute::<f128, i128>(self);
            let b = mem::transmute::<f128, i128>(other);
            mem::transmute::<i128, f128>(if a > b { b } else { a })
        }
    }

    fn abs_sub(self, other: f128) -> f128 { (self - other).abs() }

    fn cbrt(self) -> f128 { f128::from_arr( unsafe { cbrtq_f(self.0) }) }

    fn hypot(self, other: f128) -> f128 { f128::from_arr( unsafe { hypotq_f(self.0, other.0) }) }

    fn sin(self) -> f128 { f128::from_arr( unsafe { sinq_f(self.0) }) }

    fn cos(self) -> f128 { f128::from_arr( unsafe { cosq_f(self.0) }) }

    fn tan(self) -> f128 { f128::from_arr( unsafe { tanq_f(self.0) }) }

    fn asin(self) -> f128 { f128::from_arr( unsafe { asinq_f(self.0) }) }

    fn acos(self) -> f128 { f128::from_arr( unsafe { acosq_f(self.0) }) }

    fn atan(self) -> f128 { f128::from_arr( unsafe { atanq_f(self.0) }) }

    fn atan2(self, other: f128) -> f128 { f128::from_arr( unsafe { atan2q_f(self.0, other.0) }) }

    fn sin_cos(self) -> (f128, f128) { (self.sin(), self.cos()) }

    fn exp_m1(self) -> f128 { f128::from_arr( unsafe { expm1q_f(self.0) }) }

    fn ln_1p(self) -> f128 { f128::from_arr( unsafe { log1pq_f(self.0) }) }

    fn sinh(self) -> f128 { f128::from_arr( unsafe { sinhq_f(self.0) }) }

    fn cosh(self) -> f128 { f128::from_arr( unsafe { coshq_f(self.0) }) }

    fn tanh(self) -> f128 { f128::from_arr( unsafe { tanhq_f(self.0) }) }

    fn asinh(self) -> f128 { f128::from_arr( unsafe { asinhq_f(self.0) }) }

    fn acosh(self) -> f128 { f128::from_arr( unsafe { acoshq_f(self.0) }) }

    fn atanh(self) -> f128 { f128::from_arr( unsafe { atanhq_f(self.0) }) }

    fn integer_decode(self) -> (u64, i16, i8) {
        unimplemented!("This function cannot be accurately implemented with num v0.2.6 - the mantissa type needs to be upped to u128.")
    }


}
