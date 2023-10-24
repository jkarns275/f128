use std::ffi::CString;
use std::mem;
use std::str;

use ffi::*;


#[repr(C, align(16))]
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
    pub const MIN_POSITIVE_SUBNORMAL: f128 = f128([
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x01,
    ]);
    #[cfg(target_endian = "little")]
    pub const MIN_POSITIVE_SUBNORMAL: f128 = f128([
        0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00,
    ]);

    #[cfg(target_endian = "big")]
    pub const MIN_POSITIVE_NORMAL: f128 = f128([
        0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ]);
    #[cfg(target_endian = "little")]
    pub const MIN_POSITIVE_NORMAL: f128 = f128([
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01,
    ]);

    /// The smallest positive normal number. This is larger than MIN_POSITIVE_SUBNORMAL, and equal
    /// to MIN_POSITIVE_NORMAL
    pub const MIN_POSITIVE: f128 = f128::MIN_POSITIVE_NORMAL;

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
    pub fn from_bits(d: u128) -> Self {
        f128::from_arr(unsafe { mem::transmute::<u128, [u8; 16]>(d) })
    }

    #[inline(always)]
    pub fn to_bits(self) -> u128 {
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
        assert!(n > 0);
        let n = n as usize;
        let s = str::from_utf8(&buf[..n]).unwrap();
        Some(s.to_string())
    }

    #[inline(always)]
    pub fn inner(&self) -> [u8; 16] {
        self.0.clone()
    }

    #[inline(always)]
    pub fn into_inner(self) -> [u8; 16] {
        self.0
    }

    pub fn parse<T: AsRef<str>>(s: T) -> Result<Self, ParseF128Error> {
        let s = s.as_ref();
        let len = s.len();
        if len == 0 {
            return Err(pf128_empty());
        }
        let cstr = CString::new(s)
            .or(Err(pf128_invalid()))?;
        let mut end: *const i8 = std::ptr::null();

        let f;
        unsafe {
            f = strtoflt128_f(cstr.as_ptr(), &mut end);
            if end != cstr.as_ptr().offset(len as isize) {
                return Err(pf128_invalid())
            }
        }
        Ok(f)
    }

    pub fn exp_bits(&self) -> u32 {
        let exp_bits = f128::EXPONENT_BITS.to_bits();
        ((self.to_bits() & exp_bits) >> 112) as u32
    }

    pub fn fract_bits(&self) -> u128 {
        self.to_bits() & f128::FRACTION_BITS.to_bits()
    }

    pub fn bitwise_eq(self, other: Self) -> bool {
        self.0 == other.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseF128Error {
    kind: F128ErrorKind,
}

const fn pf128_empty() -> ParseF128Error {
    ParseF128Error { kind: F128ErrorKind::Empty }
}
const fn pf128_invalid() -> ParseF128Error {
    ParseF128Error { kind: F128ErrorKind::Invalid }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum F128ErrorKind {
    Empty,
    Invalid,
}
impl std::fmt::Display for ParseF128Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.kind {
            F128ErrorKind::Empty => f.write_str("cannot parse f128 from empty string"),
            F128ErrorKind::Invalid => f.write_str("invalid f128 literal"),
        }
    }
}
impl std::error::Error for ParseF128Error {}

impl std::str::FromStr for f128 {
    type Err = ParseF128Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        f128::parse(s)
    }
}
