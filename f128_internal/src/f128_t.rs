use std::mem;
use std::str;
use std::fmt;
use std::ffi::CString;
use std::ffi::NulError;
use std::num::FpCategory;

use ffi::*;

use libc::c_int;

use num_traits::*;


#[repr(C)]
#[derive(Clone, Copy)]
pub struct f128(pub(crate) [u8; 16]);

impl f128 {
    pub const RADIX: u32 = 128;
    pub const MANTISSA_DIGITS: u32 = 112;

    pub const MAX_10_EXP: u32 = 4932;
    pub const MAX_EXP: u32 = 16383;
    pub const MIN_10_EXP: i32 = -4931;
    pub const MIN_EXP: i32 = -16382;
    pub const ZERO: f128 = f128([0; 16]);

    #[cfg(target_endian = "big")]
    pub const SIGN_BIT: f128 = f128([
        0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00,
    ]);
    #[cfg(target_endian = "little")]
    pub const SIGN_BIT: f128 = f128([
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x80,
    ]);

    #[cfg(target_endian = "big")]
    pub const EXPONENT_BITS: f128 = f128([
        0x7f, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00,
    ]);
    #[cfg(target_endian = "little")]
    pub const EXPONENT_BITS: f128 = f128([
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff,
        0x7f,
    ]);

    #[cfg(target_endian = "big")]
    pub const FRACTION_BITS: f128 = f128([
        0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF,
    ]);
    #[cfg(target_endian = "little")]
    pub const FRACTION_BITS: f128 = f128([
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00,
        0x00,
    ]);

    #[cfg(target_endian = "big")]
    pub const MIN: f128 = f128([
        0xff, 0xfe, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff,
    ]);
    #[cfg(target_endian = "little")]
    pub const MIN: f128 = f128([
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xfe,
        0xff,
    ]);

    #[cfg(target_endian = "big")]
    pub const MIN_POSITIVE: f128 = f128([
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x01,
    ]);
    #[cfg(target_endian = "little")]
    pub const MIN_POSITIVE: f128 = f128([
        0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00,
    ]);

    #[cfg(target_endian = "big")]
    pub const ONE: f128 = f128([
        0x3f, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00,
    ]);
    #[cfg(target_endian = "little")]
    pub const ONE: f128 = f128([
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff,
        0x3f,
    ]);

    #[cfg(target_endian = "big")]
    pub const TWO: f128 = f128([
        0x40, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00,
    ]);
    #[cfg(target_endian = "little")]
    pub const TWO: f128 = f128([
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x40,
    ]);

    #[cfg(target_endian = "big")]
    pub const E: f128 = f128([
        0x40, 0x00, 0x5b, 0xf0, 0xa8, 0xb1, 0x45, 0x76, 0x95, 0x35, 0x5f, 0xb8, 0xac, 0x40, 0x4e,
        0x7a,
    ]);
    #[cfg(target_endian = "little")]
    pub const E: f128 = f128([
        0x7a, 0x4e, 0x40, 0xac, 0xb8, 0x5f, 0x35, 0x95, 0x76, 0x45, 0xb1, 0xa8, 0xf0, 0x5b, 0x00,
        0x40,
    ]);

    #[cfg(target_endian = "big")]
    pub const PI: f128 = f128([
        0x40, 0x00, 0x92, 0x1f, 0xb5, 0x44, 0x42, 0xd1, 0x84, 0x69, 0x89, 0x8c, 0xc5, 0x17, 0x01,
        0xb8,
    ]);
    #[cfg(target_endian = "little")]
    pub const PI: f128 = f128([
        0xb8, 0x01, 0x17, 0xc5, 0x8c, 0x89, 0x69, 0x84, 0xd1, 0x42, 0x44, 0xb5, 0x1f, 0x92, 0x00,
        0x40,
    ]);

    #[cfg(target_endian = "little")]
    pub const INFINITY: f128 = f128([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xff, 0x7f]);
    #[cfg(target_endian = "big")]
    pub const INFINITY: f128 = f128([0x7F, 0xFF, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);

    #[cfg(target_endian = "big")]
    pub const NAN: f128 = f128([
        0x7F, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFFu8,
        0xFFu8, 0xFF,
    ]);
    #[cfg(target_endian = "little")]
    pub const NAN: f128 = f128([
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFFu8,
        0xFFu8, 0x7F,
    ]);

    #[cfg(target_endian = "little")]
    pub const NEG_INFINITY: f128 = f128([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xff, 0xff]);
    #[cfg(target_endian = "big")]
    pub const NEG_INFINITY: f128 = f128([0xff, 0xff, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);

    #[cfg(target_endian = "little")]
    pub const EPSILON: f128 = f128([
        0x65, 0x64, 0xD2, 0x5B, 0x93, 0xF2, 0x61, 0x43, 0, 0x51, 0x2F, 0x7F, 0x8A, 0, 0x8F, 0x3F,
    ]);
    #[cfg(target_endian = "big")]
    pub const EPSILON: f128 = f128([
        0x3F, 0x8F, 0x0, 0x8A, 0x7F, 0x2F, 0x51, 0x0, 0x43, 0x61, 0xF2, 0x93, 0x5B, 0xD2, 0x64,
        0x65,
    ]);

    #[cfg(target_endian = "little")]
    pub const NEG_ZERO: f128 = f128([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x80]);
    #[cfg(target_endian = "big")]
    pub const NEG_ZERO: f128 = f128([0x80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);

    #[cfg(target_endian = "big")]
    pub const MAX: f128 = f128([
        0x7f, 0xfe, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff,
    ]);
    #[cfg(target_endian = "little")]
    pub const MAX: f128 = f128([
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xfe,
        0x7f,
    ]);

    #[inline(always)]
    pub(crate) fn from_arr(d: [u8; 16]) -> Self {
        f128(d)
    }

    #[inline(always)]
    pub(crate) fn from_raw_u128(d: u128) -> Self {
        f128::from_arr(unsafe { mem::transmute::<u128, [u8; 16]>(d) })
    }

    #[inline(always)]
    pub(crate) fn inner_as_u128(&self) -> u128 {
        unsafe { mem::transmute::<[u8; 16], u128>(self.0) }
    }

    #[inline(always)]
    pub fn new<T: Into<f128>>(a: T) -> Self {
        a.into()
    }

    // pub fn to_string(&self) -> String {
    //     self.to_string_fmt("%.36Qg").unwrap()
    // }

    pub fn to_string_fmt<T: AsRef<str>>(&self, fmt: T) -> Option<String> {
        let mut buf = [0u8; 128];
        let cstr;
        match CString::new(fmt.as_ref()) {
            Ok(e) => cstr = e,
            Err(_) => return None,
        };
        let n = unsafe { qtostr((&mut buf).as_mut_ptr(), 128, cstr.as_ptr(), self.clone()) };
        let mut v = Vec::with_capacity(n as usize);
        for i in 0..n {
            v.push(buf[i as usize]);
        }
        Some(String::from_utf8(v).unwrap())
    }

    #[inline(always)]
    pub fn inner(&self) -> [u8; 16] {
        self.0.clone()
    }

    #[inline(always)]
    pub fn into_inner(self) -> [u8; 16] {
        self.0
    }

    pub fn parse<T: AsRef<str>>(s: T) -> Result<Self, NulError> {
        let cstr = CString::new(s.as_ref())?;

        Ok(unsafe { strtoflt128_f(cstr.as_ptr()) })
    }

    pub fn exp_bits(&self) -> u32 {
        let exp_bits = f128::EXPONENT_BITS.inner_as_u128();
        ((self.inner_as_u128() & exp_bits) >> 112) as u32
    }

    pub fn fract_bits(&self) -> u128 {
        self.inner_as_u128() & f128::FRACTION_BITS.inner_as_u128()
    }

    pub fn bitwise_eq(self, other: Self) -> bool {
        self.0 == other.0
    }
}

impl fmt::Display for f128 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let format = 
            if let Some(precision) = f.precision() {
                format!("%.{}g", precision)
            } else {
                "%g".to_string()
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
        let res = self.inner_as_u128() & 0x7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFu128;
        res == f128::EXPONENT_BITS.inner_as_u128()
    }

    fn is_nan(self) -> bool {
        (self.inner_as_u128() & 0x7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFu128)
            > f128::EXPONENT_BITS.inner_as_u128()
    }

    #[inline]
    fn epsilon() -> Self {
        f128::EPSILON
    }

    fn is_normal(self) -> bool {
        // Normal is defined as having an exponent not equal to 0
        self.exp_bits() != 0
    }

    fn classify(self) -> FpCategory {
        let x = (self.is_normal(), self.is_finite(), self.is_nan());
        match x {
            (true, true, false) => FpCategory::Normal,
            (false, true, false) => FpCategory::Subnormal,
            (_, _, true) => FpCategory::Nan,
            (_, false, _) => FpCategory::Infinite,
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
        if self == Self::NAN {
            return self;
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
