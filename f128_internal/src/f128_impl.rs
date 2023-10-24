use std::convert::{From, Into};
use std::cmp::Ordering::*;
use std::iter::*;
use std::num::*;
use std::cmp::*;
use std::ops::*;
use std::mem;
use std::fmt;

use f128_t::*;
use ffi::*;
use ffi;

use libc::c_int;

use num_traits::*;

impl fmt::Debug for f128 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl fmt::Display for f128 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let format =
            if let Some(precision) = f.precision() {
                format!("%.{}Qg", precision)
            } else {
                "%Qg".to_string()
            };

        match self.to_string_fmt(format.as_str()) {
            Some(s) => write!(f, "{}", s),
            None => Err(fmt::Error),
        }

    }
}

impl Zero for f128 {
    fn is_zero(&self) -> bool {
        self.0 == f128::ZERO.0 || self.0 == f128::NEG_ZERO.0
    }
    fn zero() -> Self {
        f128::ZERO
    }
}

impl One for f128 {
    fn one() -> Self {
        f128::ONE
    }
}

impl ToPrimitive for f128 {
    fn to_i64(&self) -> Option<i64> {
        Some(unsafe { f128_to_i64(*self) })
    }
    fn to_u64(&self) -> Option<u64> {
        Some(unsafe { f128_to_u64(*self) })
    }
    fn to_isize(&self) -> Option<isize> {
        Some(unsafe { f128_to_i64(*self) as isize })
    }
    fn to_i8(&self) -> Option<i8> {
        Some(unsafe { f128_to_i8(*self) })
    }
    fn to_i16(&self) -> Option<i16> {
        Some(unsafe { f128_to_i16(*self) })
    }
    fn to_i32(&self) -> Option<i32> {
        Some(unsafe { f128_to_i32(*self) })
    }
    fn to_usize(&self) -> Option<usize> {
        Some(unsafe { f128_to_u64(*self) as usize })
    }
    fn to_u8(&self) -> Option<u8> {
        Some(unsafe { f128_to_u8(*self) })
    }
    fn to_u16(&self) -> Option<u16> {
        Some(unsafe { f128_to_u16(*self) })
    }
    fn to_u32(&self) -> Option<u32> {
        Some(unsafe { f128_to_u32(*self) })
    }
    fn to_f32(&self) -> Option<f32> {
        Some(unsafe { f128_to_f32(*self) })
    }
    fn to_f64(&self) -> Option<f64> {
        Some(unsafe { f128_to_f64(*self) })
    }
    fn to_i128(&self) -> Option<i128> {
        Some(unsafe { mem::transmute(f128_to_i128(*self)) })
    }
    fn to_u128(&self) -> Option<u128> {
        Some(unsafe { mem::transmute(f128_to_u128(*self)) })
    }
}

impl FromPrimitive for f128 {
    fn from_i64(n: i64) -> Option<Self> {
        Some(unsafe { i64_to_f128(n) })
    }
    fn from_u64(n: u64) -> Option<Self> {
        Some(unsafe { u64_to_f128(n) })
    }
    fn from_isize(n: isize) -> Option<Self> {
        Some(unsafe { isize_to_f128(n) })
    }
    fn from_i8(n: i8) -> Option<Self> {
        Some(unsafe { i8_to_f128(n) })
    }
    fn from_i16(n: i16) -> Option<Self> {
        Some(unsafe { i16_to_f128(n) })
    }
    fn from_i32(n: i32) -> Option<Self> {
        Some(unsafe { i32_to_f128(n) })
    }
    fn from_usize(n: usize) -> Option<Self> {
        Some(unsafe { usize_to_f128(n) })
    }
    fn from_u8(n: u8) -> Option<Self> {
        Some(unsafe { u8_to_f128(n) })
    }
    fn from_u16(n: u16) -> Option<Self> {
        Some(unsafe { u16_to_f128(n) })
    }
    fn from_u32(n: u32) -> Option<Self> {
        Some(unsafe { u32_to_f128(n) })
    }
    fn from_f32(n: f32) -> Option<Self> {
        Some(unsafe { f32_to_f128(n) })
    }
    fn from_f64(n: f64) -> Option<Self> {
        Some(unsafe { f64_to_f128(n) })
    }
    fn from_u128(n: u128) -> Option<Self> {
        Some(unsafe { u128_to_f128(mem::transmute(n)) })
    }
    fn from_i128(n: i128) -> Option<Self> {
        Some(unsafe { i128_to_f128(mem::transmute(n)) })
    }
}

impl Num for f128 {
    type FromStrRadixErr = ();
    fn from_str_radix(_s: &str, _radix: u32) -> Result<Self, ()> {
        unimplemented!()
    }
}

impl NumCast for f128 {
    fn from<T: ToPrimitive>(n: T) -> Option<Self> {
        f128::from_f64(n.to_f64().unwrap())
    }
}

impl FloatConst for f128 {
    #[cfg(target_endian = "little")]
    fn E() -> f128 { f128([0x7A, 0x4E, 0x40, 0xAC, 0xB8, 0x5F, 0x35, 0x95, 0x76, 0x45, 0xB1, 0xA8, 0xF0, 0x5B, 0x00, 0x40]) }
    #[cfg(target_endian = "big")]
    fn E() -> f128 { f128([0x40, 0x00, 0x5B, 0xF0, 0xA8, 0xB1, 0x45, 0x76, 0x95, 0x35, 0x5F, 0xB8, 0xAC, 0x40, 0x4E, 0x7A]) }

    #[cfg(target_endian = "little")]
    fn FRAC_1_PI() -> f128 { f128([0x6A, 0xEA, 0xA3, 0xAF, 0x4E, 0xF8, 0x53, 0x2A, 0x88, 0x9C, 0xDC, 0x06, 0xF3, 0x45, 0xFD, 0x3F]) }
    #[cfg(target_endian = "big")]
    fn FRAC_1_PI() -> f128 { f128([0x3F, 0xFD, 0x45, 0xF3, 0x06, 0xDC, 0x9C, 0x88, 0x2A, 0x53, 0xF8, 0x4E, 0xAF, 0xA3, 0xEA, 0x6A]) }

    #[cfg(target_endian = "little")]
    fn FRAC_1_SQRT_2() -> f128 { f128([0x95, 0xEA, 0x66, 0x13, 0xFB, 0xB2, 0x08, 0xC9, 0xBC, 0xF3, 0x67, 0xE6, 0x09, 0x6A, 0xFE, 0x3F]) }
    #[cfg(target_endian = "big")]
    fn FRAC_1_SQRT_2() -> f128 { f128([0x3F, 0xFE, 0x6A, 0x09, 0xE6, 0x67, 0xF3, 0xBC, 0xC9, 0x08, 0xB2, 0xFB, 0x13, 0x66, 0xEA, 0x95]) }

    #[cfg(target_endian = "little")]
    fn FRAC_2_PI() -> f128 { f128([0x6A, 0xEA, 0xA3, 0xAF, 0x4E, 0xF8, 0x53, 0x2A, 0x88, 0x9C, 0xDC, 0x06, 0xF3, 0x45, 0xFE, 0x3F]) }
    #[cfg(target_endian = "big")]
    fn FRAC_2_PI() -> f128 { f128([0x3F, 0xFE, 0x45, 0xF3, 0x06, 0xDC, 0x9C, 0x88, 0x2A, 0x53, 0xF8, 0x4E, 0xAF, 0xA3, 0xEA, 0x6A]) }

    #[cfg(target_endian = "little")]
    fn FRAC_2_SQRT_PI() -> f128 { f128([0xFE, 0xD7, 0xFE, 0x14, 0xA9, 0xE3, 0x1A, 0xD1, 0xB6, 0x29, 0x04, 0x75, 0xDD, 0x20, 0xFF, 0x3F]) }
    #[cfg(target_endian = "big")]
    fn FRAC_2_SQRT_PI() -> f128 { f128([0x3F, 0xFF, 0x20, 0xDD, 0x75, 0x04, 0x29, 0xB6, 0xD1, 0x1A, 0xE3, 0xA9, 0x14, 0xFE, 0xD7, 0xFE]) }

    #[cfg(target_endian = "little")]
    fn FRAC_PI_2() -> f128 { f128([0xB8, 0x01, 0x17, 0xC5, 0x8C, 0x89, 0x69, 0x84, 0xD1, 0x42, 0x44, 0xB5, 0x1F, 0x92, 0xFF, 0x3F]) }
    #[cfg(target_endian = "big")]
    fn FRAC_PI_2() -> f128 { f128([0x3F, 0xFF, 0x92, 0x1F, 0xB5, 0x44, 0x42, 0xD1, 0x84, 0x69, 0x89, 0x8C, 0xC5, 0x17, 0x01, 0xB8]) }

    #[cfg(target_endian = "little")]
    fn FRAC_PI_3() -> f128 { f128([0x7B, 0x56, 0x0F, 0x2E, 0xB3, 0x5B, 0x46, 0x58, 0x36, 0xD7, 0x82, 0x23, 0x15, 0x0C, 0xFF, 0x3F]) }
    #[cfg(target_endian = "big")]
    fn FRAC_PI_3() -> f128 { f128([0x3F, 0xFF, 0x0C, 0x15, 0x23, 0x82, 0xD7, 0x36, 0x58, 0x46, 0x5B, 0xB3, 0x2E, 0x0F, 0x56, 0x7B]) }

    #[cfg(target_endian = "little")]
    fn FRAC_PI_4() -> f128 { f128([0xB8, 0x01, 0x17, 0xC5, 0x8C, 0x89, 0x69, 0x84, 0xD1, 0x42, 0x44, 0xB5, 0x1F, 0x92, 0xFE, 0x3F]) }
    #[cfg(target_endian = "big")]
    fn FRAC_PI_4() -> f128 { f128([0x3F, 0xFE, 0x92, 0x1F, 0xB5, 0x44, 0x42, 0xD1, 0x84, 0x69, 0x89, 0x8C, 0xC5, 0x17, 0x01, 0xB8]) }

    #[cfg(target_endian = "little")]
    fn FRAC_PI_6() -> f128 { f128([0x7B, 0x56, 0x0F, 0x2E, 0xB3, 0x5B, 0x46, 0x58, 0x36, 0xD7, 0x82, 0x23, 0x15, 0x0C, 0xFE, 0x3F]) }
    #[cfg(target_endian = "big")]
    fn FRAC_PI_6() -> f128 { f128([0x3F, 0xFE, 0x0C, 0x15, 0x23, 0x82, 0xD7, 0x36, 0x58, 0x46, 0x5B, 0xB3, 0x2E, 0x0F, 0x56, 0x7B]) }

    #[cfg(target_endian = "little")]
    fn FRAC_PI_8() -> f128 { f128([0xB8, 0x01, 0x17, 0xC5, 0x8C, 0x89, 0x69, 0x84, 0xD1, 0x42, 0x44, 0xB5, 0x1F, 0x92, 0xFD, 0x3F]) }
    #[cfg(target_endian = "big")]
    fn FRAC_PI_8() -> f128 { f128([0x3F, 0xFD, 0x92, 0x1F, 0xB5, 0x44, 0x42, 0xD1, 0x84, 0x69, 0x89, 0x8C, 0xC5, 0x17, 0x01, 0xB8]) }

    #[cfg(target_endian = "little")]
    fn LN_10() -> f128 { f128([0xA6, 0x05, 0x57, 0xAC, 0xAD, 0xD4, 0x2D, 0x58, 0x51, 0x55, 0xBB, 0x1B, 0xBB, 0x26, 0x00, 0x40]) }
    #[cfg(target_endian = "big")]
    fn LN_10() -> f128 { f128([0x40, 0x00, 0x26, 0xBB, 0x1B, 0xBB, 0x55, 0x51, 0x58, 0x2D, 0xD4, 0xAD, 0xAC, 0x57, 0x05, 0xA6]) }

    #[cfg(target_endian = "little")]
    fn LN_2() -> f128 { f128([0xE6, 0x07, 0x30, 0x67, 0xC7, 0x93, 0x57, 0xF3, 0x9E, 0xA3, 0xEF, 0x2F, 0xE4, 0x62, 0xFE, 0x3F]) }
    #[cfg(target_endian = "big")]
    fn LN_2() -> f128 { f128([0x3F, 0xFE, 0x62, 0xE4, 0x2F, 0xEF, 0xA3, 0x9E, 0xF3, 0x57, 0x93, 0xC7, 0x67, 0x30, 0x07, 0xE6]) }

    #[cfg(target_endian = "little")]
    fn LOG10_E() -> f128 { f128([0x67, 0x5A, 0x5F, 0x55, 0xB7, 0x6A, 0x2A, 0xE3, 0x50, 0x6E, 0x52, 0xB1, 0xB7, 0xBC, 0xFD, 0x3F]) }
    #[cfg(target_endian = "big")]
    fn LOG10_E() -> f128 { f128([0x3F, 0xFD, 0xBC, 0xB7, 0xB1, 0x52, 0x6E, 0x50, 0xE3, 0x2A, 0x6A, 0xB7, 0x55, 0x5F, 0x5A, 0x67]) }

    #[cfg(target_endian = "little")]
    fn LOG2_E() -> f128 { f128([0x3A, 0xD2, 0xA0, 0xFD, 0x0F, 0x7D, 0x77, 0xE1, 0x2F, 0xB8, 0x52, 0x76, 0x54, 0x71, 0xFF, 0x3F]) }
    #[cfg(target_endian = "big")]
    fn LOG2_E() -> f128 { f128([0x3F, 0xFF, 0x71, 0x54, 0x76, 0x52, 0xB8, 0x2F, 0xE1, 0x77, 0x7D, 0x0F, 0xFD, 0xA0, 0xD2, 0x3A]) }

    #[cfg(target_endian = "little")]
    fn PI() -> f128 { f128([0xB8, 0x01, 0x17, 0xC5, 0x8C, 0x89, 0x69, 0x84, 0xD1, 0x42, 0x44, 0xB5, 0x1F, 0x92, 0x00, 0x40]) }
    #[cfg(target_endian = "big")]
    fn PI() -> f128 { f128([0x40, 0x00, 0x92, 0x1F, 0xB5, 0x44, 0x42, 0xD1, 0x84, 0x69, 0x89, 0x8C, 0xC5, 0x17, 0x01, 0xB8]) }

    #[cfg(target_endian = "little")]
    fn SQRT_2() -> f128 { f128([0x96, 0xEA, 0x66, 0x13, 0xFB, 0xB2, 0x08, 0xC9, 0xBC, 0xF3, 0x67, 0xE6, 0x09, 0x6A, 0xFF, 0x3F]) }
    #[cfg(target_endian = "big")]
    fn SQRT_2() -> f128 { f128([0x3F, 0xFF, 0x6A, 0x09, 0xE6, 0x67, 0xF3, 0xBC, 0xC9, 0x08, 0xB2, 0xFB, 0x13, 0x66, 0xEA, 0x96]) }

}

impl Float for f128 {
    fn nan() -> Self {
        f128::NAN
    }

    fn infinity() -> Self {
        f128::INFINITY
    }

    fn neg_infinity() -> Self {
        f128::NEG_INFINITY
    }

    fn neg_zero() -> Self {
        f128::NEG_ZERO
    }

    fn min_value() -> f128 {
        f128::MIN
    }

    fn max_value() -> f128 {
        f128::MAX
    }

    fn min_positive_value() -> f128 {
        f128::MIN_POSITIVE
    }

    fn is_finite(self) -> bool {
        !self.is_infinite() && !self.is_nan()
    }

    fn is_infinite(self) -> bool {
        // It's fine to compare the bits here since there is only 1 bit pattern that is inf, and one
        // that is -inf.
        let res = self.to_bits() & 0x7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFu128;
        res == f128::EXPONENT_BITS.to_bits()
    }

    fn is_nan(self) -> bool {
        (self.to_bits() & 0x7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFu128)
            > f128::EXPONENT_BITS.to_bits()
    }

    #[inline]
    fn epsilon() -> Self {
        f128::EPSILON
    }

    fn is_normal(self) -> bool {
        // Normal is defined as having an exponent not equal to 0 and being finite
        let exp_bits = self.exp_bits();
        exp_bits != 0 && exp_bits != 0x7FFF
    }

    fn classify(self) -> FpCategory {
        if self.is_infinite() {
            FpCategory::Infinite
        } else if self.is_nan() {
            FpCategory::Nan
        } else {
            let exp_bits = self.exp_bits();
            let mant_bits = self.fract_bits();
            match (exp_bits, mant_bits) {
                (0, 0) => FpCategory::Zero,
                (0, _) => FpCategory::Subnormal,
                (_, _) => FpCategory::Normal,
            }
        }
    }

    fn floor(self) -> Self {
        unsafe { floorq_f(self) }
    }

    fn ceil(self) -> Self {
        unsafe { ceilq_f(self) }
    }

    fn round(self) -> Self {
        unsafe { roundq_f(self) }
    }

    fn trunc(self) -> Self {
        unsafe { truncq_f(self) }
    }

    fn fract(self) -> Self {
        let mut x: c_int = 0;
        unsafe { frexpq_f(self, &mut x) }
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
        if self.is_nan() {
            Self::NAN
        } else {
            if self.is_sign_positive() {
                Self::ONE
            } else {
                -Self::ONE
            }
        }
    }

    #[cfg(target_endian = "big")]
    #[inline]
    fn is_sign_negative(self) -> bool {
        match self.0[0] & 0x80 {
            0 => true,
            0x80 => false,
            _ => unreachable!(),
        }
    }

    #[cfg(target_endian = "little")]
    #[inline]
    fn is_sign_negative(self) -> bool {
        match self.0[15] & 0x80 {
            0 => false,
            0x80 => true,
            _ => unreachable!(),
        }
    }

    #[cfg(target_endian = "big")]
    #[inline]
    fn is_sign_positive(self) -> bool {
        match self.0[0] & 0x80 {
            0 => true,
            0x80 => false,
            _ => unreachable!(),
        }
    }

    #[cfg(target_endian = "little")]
    #[inline]
    fn is_sign_positive(self) -> bool {
        match self.0[15] & 0x80 {
            0 => true,
            0x80 => false,
            _ => unreachable!(),
        }
    }

    fn mul_add(self, a: f128, b: f128) -> f128 {
        unsafe { fmaq_f(self, a, b) }
    }

    fn recip(self) -> f128 {
        f128::ONE / self
    }

    fn powi(self, n: i32) -> f128 {
        let mut i = self.clone();
        if n == 0 {
            return f128::ONE;
        };
        if n < 0 {
            for _ in n as i64 - 1..0 {
                i /= self;
            }
        } else {
            for _ in 1..n {
                i *= self;
            }
        }
        i
    }

    fn powf(self, n: f128) -> f128 {
        unsafe { powq_f(self, n) }
    }

    fn sqrt(self) -> f128 {
        unsafe { sqrtq_f(self) }
    }

    fn exp(self) -> f128 {
        unsafe { expq_f(self) }
    }

    fn exp2(self) -> f128 {
        // TODO: Change this to a constant two
        (f128::ONE * f128::from_u8(2).unwrap()).powf(self)
    }

    fn ln(self) -> f128 {
        unsafe { logq_f(self) }
    }

    fn log(self, base: f128) -> f128 {
        // Change of base formula
        let numr = self.ln();
        let denm = base.ln();
        numr / denm
    }

    fn log2(self) -> f128 {
        unsafe { log2q_f(self) }
    }

    fn log10(self) -> f128 {
        unsafe { log10q_f(self) }
    }

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

    fn abs_sub(self, other: f128) -> f128 {
        (self - other).abs()
    }

    fn cbrt(self) -> f128 {
        unsafe { cbrtq_f(self) }
    }

    fn hypot(self, other: f128) -> f128 {
        unsafe { hypotq_f(self, other) }
    }

    fn sin(self) -> f128 {
        unsafe { sinq_f(self) }
    }

    fn cos(self) -> f128 {
        unsafe { cosq_f(self) }
    }

    fn tan(self) -> f128 {
        unsafe { tanq_f(self) }
    }

    fn asin(self) -> f128 {
        unsafe { asinq_f(self) }
    }

    fn acos(self) -> f128 {
        unsafe { acosq_f(self) }
    }

    fn atan(self) -> f128 {
        unsafe { atanq_f(self) }
    }

    fn atan2(self, other: f128) -> f128 {
        unsafe { atan2q_f(self, other) }
    }

    fn sin_cos(self) -> (f128, f128) {
        (self.sin(), self.cos())
    }

    fn exp_m1(self) -> f128 {
        unsafe { expm1q_f(self) }
    }

    fn ln_1p(self) -> f128 {
        unsafe { log1pq_f(self) }
    }

    fn sinh(self) -> f128 {
        unsafe { sinhq_f(self) }
    }

    fn cosh(self) -> f128 {
        unsafe { coshq_f(self) }
    }

    fn tanh(self) -> f128 {
        unsafe { tanhq_f(self) }
    }

    fn asinh(self) -> f128 {
        unsafe { asinhq_f(self) }
    }

    fn acosh(self) -> f128 {
        unsafe { acoshq_f(self) }
    }

    fn atanh(self) -> f128 {
        unsafe { atanhq_f(self) }
    }

    fn integer_decode(self) -> (u64, i16, i8) {
        unimplemented!("This function cannot be accurately implemented with num v0.2.6 - the mantissa type needs to be upped to u128.")
    }
}

impl f128 {
    pub fn is_subnormal(&self) -> bool {
        self.classify() == FpCategory::Subnormal
    }

    /// Return the ordering between `self` and `other` in accordance to
    /// the `totalOrder` predicate as defined in the IEEE 754 (2008 revision),
    /// as implemented in `core` for `f32` and `f64`.
    pub fn total_cmp(&self, rhs: &Self) -> Ordering {
        // See f32::total_cmp or f64::total_cmp in `core` for explanation.
        let mut left = self.to_bits() as i128;
        let mut right = rhs.to_bits() as i128;

        left ^= (((left >> 127) as u128) >> 1) as i128;
        right ^= (((right >> 127) as u128) >> 1) as i128;

        left.cmp(&right)
    }
}

impl Neg for f128 {
    type Output = Self;

    fn neg(self) -> Self {
        let mut bits = self.to_bits();
        bits ^= f128::SIGN_BIT.to_bits();
        f128::from_bits(bits)
    }
}

macro_rules! forward_ref_binop {
    (impl $imp:ident, $method:ident for $t:ty, $u:ty) => {
        impl<'a> $imp<$u> for &'a $t {
            type Output = <$t as $imp<$u>>::Output;

            #[inline]
            fn $method(self, other: $u) -> <$t as $imp<$u>>::Output {
                $imp::$method(*self, other)
            }
        }

        impl<'a> $imp<&'a $u> for $t {
            type Output = <$t as $imp<$u>>::Output;

            #[inline]
            fn $method(self, other: &'a $u) -> <$t as $imp<$u>>::Output {
                $imp::$method(self, *other)
            }
        }

        impl<'a, 'b> $imp<&'a $u> for &'b $t {
            type Output = <$t as $imp<$u>>::Output;

            fn $method(self, other: &'a $u) -> <$t as $imp<$u>>::Output {
                $imp::$method(*self, *other)
            }
        }
    };
}

macro_rules! forward_ref_assignop {
    (impl $imp:ident, $method:ident for $t:ty, $u:ty) => {
        impl<'a> $imp<&'a $u> for $t {

            #[inline]
            fn $method(&mut self, other: &'a $u) {
                $imp::$method(self, *other)
            }
        }
    };
}

macro_rules! shl_impl {
    ($t:ty, $f:ty) => {
        impl Shr<$f> for $t {
            type Output = $t;

            #[inline]
            fn shr(self, other: $f) -> f128 {
                unsafe {
                    mem::transmute::<u128, f128>(
                        mem::transmute::<[u8; 16], u128>(self.inner()) >> other,
                    )
                }
            }
        }
        forward_ref_binop! { impl Shr, shr for $t, $f }
    };
}

shl_impl! { f128, u8 }
shl_impl! { f128, u16 }
shl_impl! { f128, u32 }
shl_impl! { f128, u64 }
shl_impl! { f128, u128 }
shl_impl! { f128, usize }

shl_impl! { f128, i8 }
shl_impl! { f128, i16 }
shl_impl! { f128, i32 }
shl_impl! { f128, i64 }
shl_impl! { f128, i128 }
shl_impl! { f128, isize }

macro_rules! shr_assign_impl {
    ($t:ty, $f:ty) => {
        impl ShrAssign<$f> for $t {
            #[inline]
            fn shr_assign(&mut self, other: $f) {
                *self = *self >> other
            }
        }
    };
}

shr_assign_impl! {f128, u8 }
shr_assign_impl! {f128, u16 }
shr_assign_impl! {f128, u32 }
shr_assign_impl! {f128, u64 }
shr_assign_impl! {f128, u128 }
shr_assign_impl! {f128, usize }

shr_assign_impl! {f128, i8 }
shr_assign_impl! {f128, i16 }
shr_assign_impl! {f128, i32 }
shr_assign_impl! {f128, i64 }
shr_assign_impl! {f128, i128 }
shr_assign_impl! {f128, isize }

macro_rules! shl_impl {
    ($t:ty, $f:ty) => {
        impl Shl<$f> for $t {
            type Output = $t;

            #[inline]
            fn shl(self, other: $f) -> $t {
                unsafe {
                    mem::transmute::<u128, f128>(
                        mem::transmute::<[u8; 16], u128>(self.inner()) << other,
                    )
                }
            }
        }
        forward_ref_binop! { impl Shl, shl for $t, $f }
    };
}

shl_impl! { f128, u8 }
shl_impl! { f128, u16 }
shl_impl! { f128, u32 }
shl_impl! { f128, u64 }
shl_impl! { f128, u128 }
shl_impl! { f128, usize }

shl_impl! { f128, i8 }
shl_impl! { f128, i16 }
shl_impl! { f128, i32 }
shl_impl! { f128, i64 }
shl_impl! { f128, i128 }
shl_impl! { f128, isize }

macro_rules! shl_assign_impl {
    ($t:ty, $f:ty) => {
        impl ShlAssign<$f> for $t {
            #[inline]
            fn shl_assign(&mut self, other: $f) {
                *self = *self << other
            }
        }
    };
}

shl_assign_impl! {f128, u8 }
shl_assign_impl! {f128, u16 }
shl_assign_impl! {f128, u32 }
shl_assign_impl! {f128, u64 }
shl_assign_impl! {f128, u128 }
shl_assign_impl! {f128, usize }

shl_assign_impl! {f128, i8 }
shl_assign_impl! {f128, i16 }
shl_assign_impl! {f128, i32 }
shl_assign_impl! {f128, i64 }
shl_assign_impl! {f128, i128 }
shl_assign_impl! {f128, isize }

impl Add for f128 {
    type Output = f128;

    #[inline]
    fn add(self, other: f128) -> f128 {
        unsafe { ffi::f128_add(self, other) }
    }
}

impl AddAssign for f128 {
    #[inline]
    fn add_assign(&mut self, other: f128) {
        *self = *self + other;
    }
}

forward_ref_binop! { impl Add, add for f128, f128 }
forward_ref_assignop! { impl AddAssign, add_assign for f128, f128 }

impl Sub for f128 {
    type Output = f128;

    #[inline]
    fn sub(self, other: f128) -> f128 {
        unsafe { ffi::f128_sub(self, other) }
    }
}

impl SubAssign for f128 {
    #[inline]
    fn sub_assign(&mut self, other: f128) {
        *self = *self - other;
    }
}

forward_ref_binop! { impl Sub, sub for f128, f128 }
forward_ref_assignop! { impl SubAssign, sub_assign for f128, f128 }

impl Mul for f128 {
    type Output = f128;

    #[inline]
    fn mul(self, other: f128) -> f128 {
        unsafe { ffi::f128_mul(self, other) }
    }
}

impl MulAssign for f128 {
    #[inline]
    fn mul_assign(&mut self, other: f128) {
        *self = *self * other;
    }
}

forward_ref_binop! { impl Mul, mul for f128, f128 }
forward_ref_assignop! { impl MulAssign, mul_assign for f128, f128 }

impl Div for f128 {
    type Output = f128;

    #[inline]
    fn div(self, other: f128) -> f128 {
        unsafe { ffi::f128_div(self, other) }
    }
}

impl DivAssign for f128 {
    #[inline]
    fn div_assign(&mut self, other: f128) {
        *self = *self / other
    }
}

forward_ref_binop! { impl Div, div for f128, f128 }
forward_ref_assignop! { impl DivAssign, div_assign for f128, f128 }

impl Rem<f128> for f128 {
    type Output = f128;

    #[inline]
    fn rem(self, other: f128) -> f128 {
        unsafe { ffi::fmodq_f(self, other) }
    }
}

impl RemAssign for f128 {
    #[inline]
    fn rem_assign(&mut self, other: f128) {
        *self = *self % other
    }
}

forward_ref_binop! { impl Rem, rem for f128, f128 }
forward_ref_assignop! { impl RemAssign, rem_assign for f128, f128 }

impl Sum for f128 {
    fn sum<I: Iterator<Item = f128>>(iter: I) -> f128 {
        iter.fold(0.into(), |a, b| a + b)
    }
}

impl Product for f128 {
    fn product<I: Iterator<Item = f128>>(iter: I) -> f128 {
        iter.fold(1.into(), |a, b| a * b)
    }
}

impl<'a> Sum<&'a f128> for f128 {
    fn sum<I: Iterator<Item = &'a f128>>(iter: I) -> f128 {
        iter.fold(0.into(), |a, b| a + *b)
    }
}

impl<'a> Product<&'a f128> for f128 {
    fn product<I: Iterator<Item = &'a f128>>(iter: I) -> f128 {
        iter.fold(1.into(), |a, b| a * *b)
    }
}

macro_rules! impl_from_to {
    ($($to:ident, $from:ident - $ty:ty),*) => ($(
        impl From<$ty> for f128 {
            #[inline]
            fn from(small: $ty) -> f128 {
                unsafe { ::std::mem::transmute($from(::std::mem::transmute(small))) }
            }
        }
        impl Into<$ty> for f128 {
            #[inline]
            fn into(self) -> $ty {
               unsafe { ::std::mem::transmute($to(::std::mem::transmute(self))) }
            }
        }

    )*)
}

impl_from_to! {
    f128_to_isize,   isize_to_f128  - isize,
    f128_to_i8,      i8_to_f128     - i8,
    f128_to_i16,     i16_to_f128    - i16,
    f128_to_i32,     i32_to_f128    - i32,
    f128_to_i64,     i64_to_f128    - i64,
    f128_to_i128,    i128_to_f128   - i128,
    f128_to_usize,   usize_to_f128  - usize,
    f128_to_u8,      u8_to_f128     - u8,
    f128_to_u16,     u16_to_f128    - u16,
    f128_to_u32,     u32_to_f128    - u32,
    f128_to_u64,     u64_to_f128    - u64,
    f128_to_u128,    u128_to_f128   - u128,
    f128_to_f32,     f32_to_f128    - f32,
    f128_to_f64,     f64_to_f128    - f64
}

impl PartialOrd for f128 {
    fn partial_cmp(&self, other: &f128) -> Option<Ordering> {
        let lte = unsafe { lteq(*self, *other) };
        let gte = unsafe { gteq(*self, *other) };

        match (lte != 0, gte != 0) {
            (false, false) => None,
            (false, true) => Some(Greater),
            (true, false) => Some(Less),
            (true, true) => Some(Equal),
        }
    }
}

impl PartialEq for f128 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { eqq(*self, *other) != 0 }
    }

    fn ne(&self, other: &Self) -> bool {
        unsafe { neqq(*self, *other) != 0 }
    }
}

#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Zeroable for f128 {}
#[cfg(feature = "bytemuck")]
unsafe impl bytemuck::Pod for f128 {}
