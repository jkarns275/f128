use std::ops::*;
use ffi;
use ffi::*;
use std::convert::{ From, Into };
use std::iter::*;
use std::hash::{ Hash, Hasher };
use std::mem;
use std::slice;
use std::ffi::CString;
use std::ffi::NulError;
use num::*;
use f128_derive::*;
use std::num::FpCategory;
use libc::c_int;

#[derive(Clone, Copy)]
pub struct f128(pub [u8; 16]);

pub trait To16Bytes {
    fn to_arr(&self) -> [u8; 16];
    fn to_u128(&self) -> u128;
    fn to_i128(&self) -> i128;
}

impl f128 {
    pub const RADIX: u32 = 128;
    pub const MANTISSA_DIGITS: u32 = 112;
    pub const INFINITY: f128 = f128::infinity();
    pub const NEG_INFINITY: f128 = f128::neg_infinity();
    pub const MAX: f128 = f128::max_value();
    pub const MAX_10_EXP: u32 = 4932;
    pub const MAX_EXP: u32 = 16383;
    pub const MIN_POSITIVE: f128 = f128::from_arr([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]);
    pub const MIN_10_EXP: i32 = -4931;
    pub const MIN_EXP: i32 = -16382;
    pub const MIN: f128 = f128::from_arr([0x7f, 0xfe, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff]);
    pub const NAN: f128 = f128::nan();
    pub const SIGN_BIT: f128 = f128::from_arr([0x80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    pub const EXPONENT_BITS: f128 = f128::from_arr([0x7, 0xFF, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    pub const FRACTION_BITS: f128 = f128::from_arr([0x0, 0x0, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]);
    pub const ONE: f128 = f128::from_arr([0x3f, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);

    pub const PI: f128 = f128::from_arr([0x40, 0x00, 0x92, 0x1f, 0xb5, 0x44, 0x42, 0xd1, 0x84, 0x69, 0x89, 0x8c, 0xc5, 0x17, 0x01, 0xb8]);

    #[inline(always)]
    pub const fn from_arr(d: [u8; 16]) -> Self {
        f128(d)
    }

    #[inline(always)]
    pub fn from_u128(d: u128) -> Self {
        f128::from_arr(unsafe { mem::transmute::<u128, [u8; 16]>(d) })
    }

    #[inline(always)]
    pub fn from_bits<T: To16Bytes>(x: &To16Bytes) -> Self {
        f128( x.to_arr() )
    }

    #[inline(always)]
    pub fn to_u128(&self) -> u128 {
        unsafe { mem::transmute::<[u8; 16], u128>(self.0) }
    }

    #[inline(always)]
    pub const fn zero() -> Self { f128([0u8; 16]) }
    #[inline(always)]
    pub const fn neg_zero() -> Self { f128([80u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]) }
    #[inline(always)]
    pub const fn min_value() -> f128 { f128::from_arr([0x7f, 0xfe, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff]) }
    #[inline(always)]
    pub const fn max_value() -> f128 { f128::from_arr([0xff, 0xfe, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff]) }

    #[inline(always)]
    pub fn new<T: Into<f128>>(a: T) -> Self { a.into() }

    pub fn to_string(&self) -> String {
        self.to_string_fmt("%.36Qg").unwrap()
    }

    pub fn to_string_fmt<T: AsRef<str>>(&self, fmt: T) -> Option<String> {
        let mut buf: [u8; 128] = [0; 128];
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

        Ok(unsafe { f128::from_arr(strtoflt128_f(cstr.as_ptr()))})
    }

    #[inline(always)]
    pub const fn nan() -> Self {
        f128::from_arr([0x7Fu8, 0xFF, 0xFF, 0xFF, 0xFF,  0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFFu8])
    }

    #[inline(always)]
    pub const fn infinity() -> Self {
        f128::from_arr([0x7f, 0xff, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
    }

    #[inline(always)]
    pub const fn neg_infinity() -> Self {
        f128::from_arr([0xff, 0xff, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
    }

    pub fn is_finite(&self) -> bool {
        let bits = self.to_u128();
        (bits & f128::EXPONENT_BITS.to_u128()) != f128::EXPONENT_BITS.to_u128()
    }

    pub fn is_infinite(&self) -> bool {
        *self == f128::INFINITY || *self == f128::NEG_INFINITY
    }

    pub fn is_nan(&self) -> bool {
        let bits = self.to_u128();
        ((bits & f128::EXPONENT_BITS.to_u128()) == f128::EXPONENT_BITS.to_u128()) && ((bits & f128::FRACTION_BITS.to_u128()) != 0)
    }

    pub fn is_normal(&self) -> bool {
        // I'm pretty sure using >= here is acceptable.
        self.is_finite() && *self >= f128::ONE
    }

    pub fn classify(&self) -> FpCategory {
        match (self.is_normal(), self.is_finite(), self.is_nan()) {
            (true, true, false) => FpCategory::Normal,
            (false, true, false) => FpCategory::Subnormal,
            (_, _, true) => FpCategory::Nan,
            (_, false, _) => FpCategory::Infinite,
            _ => unreachable!()
        }
    }

    pub fn floor(self) -> Self {
        f128::from_arr(unsafe { floorq_f(self.0) })
    }

    pub fn ceil(self) -> Self {
        f128::from_arr(unsafe { ceilq_f(self.0) })
    }

    pub fn round(self) -> Self {
        f128::from_arr(unsafe { roundq_f(self.0) })
    }

    pub fn trunc(self) -> Self {
        f128::from_arr(unsafe { truncq_f(self.0) })
    }

    pub fn fract(self) -> Self {
        let mut x: c_int = 0;
        let _ = unsafe { frexpq_f(self.0, &mut x) };
        f128::from_u32(x as u32).unwrap()
    }

    pub fn abs(mut self) -> Self {
        self.0[0] &= 0x7F;
        self
    }

    pub fn signum(self) -> Self {
        match self.0[0] & 0x80 {
            0 => f128::INFINITY,
            1 => f128::NEG_INFINITY,
            _ => unreachable!()
        }
    }

    pub fn is_sign_negative(self) -> bool {
        match self.0[0] & 0x80 {
            0 => false,
            1 => true,
            _ => unreachable!()
        }
    }

    pub fn is_sign_positive(self) -> bool {
        match self.0[0] & 0x80 {
            1 => false,
            0 => true,
            _ => unreachable!()
        }
    }

    pub fn mul_add(self, a: f128, b: f128) -> f128 {
        f128::from_arr(unsafe { fmaq_f(self.0, a.0, b.0) })
    }

    pub fn recip(self) -> f128 {
        f128::ONE / self
    }

    pub fn powi(self, n: i32) -> f128 {
        let mut i = self.clone();
        for _ in 0..n {
            i *= self;
        }
        i
    }

    pub fn powf(self, n: f128) -> f128 {
        f128::from_arr(unsafe { powq_f(self.0, n.0) })
    }

    pub fn sqrt(self) -> f128 {
        f128::from_arr(unsafe { sqrtq_f(self.0) })
    }

    pub fn exp(self) -> f128 {
        f128::from_arr(unsafe { expq_f(self.0) })
    }

    pub fn exp2(self) -> f128 {
        (f128::ONE * f128::from_u8(2).unwrap()).powf(self)
    }


}

