use std::ops::*;
use ffi;
use ffi::*;
use std::convert::{ From, Into };
use std::iter::*;
use std::hash::{ Hash, Hasher };
use std::mem;
use std::slice;
use std::ffi::CString;
use num::*;
use f128_derive::*;

#[derive(Clone, Copy, PartialOrd, Ord, Eq, PartialEq)]
pub struct f128(pub [u8; 16]);

pub trait To16Bytes {
    fn to_arr(&self) -> [u8; 16];
    fn to_u128(&self) -> u128;
    fn to_i128(&self) -> i128;
}

impl f128 {
    #[inline(always)]
    pub fn from_arr(d: [u8; 16]) -> Self {
        f128(d)
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
    pub fn zero() -> Self { f128([0u8; 16]) }

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

    pub fn parse<T: AsRef<str>>(s: T) -> Option<Self> {
        let cstr;
        match CString::new(s.as_ref()) {
            Ok(e) => cstr = e,
            Err(_) => return None
        };
        Some(unsafe { f128::from_arr(strtoflt128_f(cstr.as_ptr()))})
    }
}

