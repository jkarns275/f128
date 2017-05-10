use std::ops::*;
use ffi;
use ffi::*;
use std::num::{ One, Zero };
use std::convert::{ From, Into };
use std::iter::*;
use std::hash::{ Hash, Hasher };
use std::mem;
use std::slice;
use std::ffi::CString;

#[derive(Clone, Copy, PartialOrd, Ord, Eq, PartialEq)]
pub struct f128(i128);

impl f128 {
    #[inline(always)]
    fn from_i128(d: i128) -> Self {
        f128(d)
    }

    #[inline(always)]
    pub fn zero() -> Self { f128(0) }

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
    pub fn inner(&self) -> i128 { self.0}

    #[inline(always)]
    pub fn into_inner(self) -> i128 { self.0 }

    pub fn parse<T: AsRef<str>>(s: T) -> Option<Self> {
        let cstr;
        match CString::new(s.as_ref()) {
            Ok(e) => cstr = e,
            Err(_) => return None
        };
        Some(unsafe { f128::from_i128(strtoflt128_f(cstr.as_ptr()))})
    }
}

pub trait F128 {
    fn from_f128(x: f128) -> Self;
    fn f128(self) -> f128;
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
    }
}

macro_rules! into_f128_gen {
    ($($t:ident)*) => ($(
        impl F128 for $t {
            #[inline]
            fn from_f128(x: f128) -> Self {
                unsafe { concat_idents!(f128_to_, $t)(x.into_inner()) }
            }
            #[inline]
            fn f128(self) -> f128 {
                unsafe { f128::from_i128(concat_idents!($t, _to_f128)(self)) }
            }
        }
    )*)
}


into_f128_gen! { usize u8 u16 u32 u64 i128 isize i8 i16 i32 i64 u128 f32 f64 }

macro_rules! shl_impl {
    ($t:ty, $f:ty) => (
    impl Shr<$f> for $t {
        type Output = $t;

        #[inline]
        fn shr(self, other: $f) -> $t {
            f128::from_i128(self.into_inner() >> other)
        }
    }
        forward_ref_binop! { impl Shr, shr for $t, $f }
    )
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
    ($t:ty, $f:ty) => (
        impl ShrAssign<$f> for $t {
            #[inline]
            fn shr_assign(&mut self, other: $f) {
                *self = *self >> other
            }
        }
    )
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
    ($t:ty, $f:ty) => (
    impl Shl<$f> for $t {
        type Output = $t;

        #[inline]
        fn shl(self, other: $f) -> $t {
            f128::from_i128(self.into_inner() << other)
        }
    }
        forward_ref_binop! { impl Shl, shl for $t, $f }
    )
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
    ($t:ty, $f:ty) => (
        impl ShlAssign<$f> for $t {
            #[inline]
            fn shl_assign(&mut self, other: $f) {
                *self = *self << other
            }
        }
    )
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

macro_rules! impl_from {
    ($($sm: ident)*) => ($(
        impl From<$sm> for f128 {
            #[inline]
            fn from(small: $sm) -> f128 {
                unsafe { f128::from_i128(concat_idents!($sm, _to_f128)(small)) }
            }
        }
        impl Into<$sm> for f128 {
            #[inline]
            fn into(self) -> $sm {
               unsafe { concat_idents!(f128_to_, $sm)(self.0) }
            }
        }
    )*)
}

impl_from! { usize u8 u16 u32 u64 i128 isize i8 i16 i32 i64 u128 f32 f64 }

impl Add for f128 {
    type Output = f128;

    #[inline]
    fn add(self, other: f128) -> f128 {
        f128::from_i128(unsafe { ffi::f128_add(self.into_inner(), other.into_inner()) })
    }

}

impl AddAssign for f128 {
    #[inline]
    fn add_assign(&mut self, other: f128) {
        *self = *self + other;
    }
}

forward_ref_binop! { impl Add, add for f128, f128 }

impl Sub for f128 {
    type Output = f128;

    #[inline]
    fn sub(self, other: f128) -> f128 {
        f128::from_i128(unsafe { ffi::f128_sub(self.into_inner(), other.into_inner()) })
    }
}

impl SubAssign for f128 {
    #[inline]
    fn sub_assign(&mut self, other: f128) {
        *self = *self - other;
    }
}

forward_ref_binop! { impl Sub, sub for f128, f128 }

impl Mul for f128 {
    type Output = f128;

    #[inline]
    fn mul(self, other: f128) -> f128 {
        f128::from_i128(unsafe { ffi::f128_mul(self.into_inner(), other.into_inner()) })
    }
}

impl MulAssign for f128 {
    #[inline]
    fn mul_assign(&mut self, other: f128) {
        *self = *self * other;
    }
}

forward_ref_binop! { impl Mul, mul for f128, f128 }

impl Div for f128 {
    type Output = f128;

    #[inline]
    fn div(self, other: f128) -> f128 {
        f128::from_i128(unsafe { ffi::f128_div(self.into_inner(), other.into_inner()) })
    }
}

impl DivAssign for f128 {
    #[inline]
    fn div_assign(&mut self, other: f128) {
        *self = *self / other
    }
}

forward_ref_binop! { impl Div, div for f128, f128 }

impl Rem<f128> for f128 {
    type Output = f128;

    fn rem(self, other: f128) -> f128 {
        unsafe { f128::from_i128(ffi::fmodq_f(self.0, other.0)) }
    }
}

impl<'a> Rem<&'a f128> for f128 {
    type Output = f128;

    fn rem(self, other: &'a f128) -> f128 {
        unsafe { f128::from_i128(ffi::fmodq_f(self.0, other.0)) }
    }
}

impl<'a, 'b> Rem<&'a f128> for &'b f128 {
    type Output = f128;

    fn rem(self, other: &'a f128) -> f128 {
        unsafe { f128::from_i128(ffi::fmodq_f(self.0, other.0)) }
    }
}

impl<'a> Rem<f128> for &'a f128 {
    type Output = f128;

    fn rem(self, other: f128) -> f128 {
        unsafe { f128::from_i128(ffi::fmodq_f(self.0, other.0)) }
    }
}

impl Sum for f128 {
    fn sum<I: Iterator<Item=f128>>(iter: I) -> f128 {
        iter.fold(0.0f64.f128(), |a, b| a + b)
    }
}

impl Product for f128 {
    fn product<I: Iterator<Item=f128>>(iter: I) -> f128 {
        iter.fold(1.f128(), |a, b| a * b)
    }
}

impl<'a> Sum<&'a f128> for f128 {
    fn sum<I: Iterator<Item=&'a f128>>(iter: I) -> f128 {
        iter.fold(0.f128(), |a, b| a + *b)
    }
}

impl<'a> Product<&'a f128> for f128 {
    fn product<I: Iterator<Item=&'a f128>>(iter: I) -> f128 {
        iter.fold(1.f128(), |a, b| a * *b)
    }
}

impl Hash for f128 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_i128(self.0)
    }
    fn hash_slice<H: Hasher>(data: &[f128], state: &mut H) {
        let newlen = data.len() * mem::size_of::<f128>();
        let ptr = data.as_ptr() as *const u8;
        state.write(unsafe { slice::from_raw_parts(ptr, newlen) })
    }
}
