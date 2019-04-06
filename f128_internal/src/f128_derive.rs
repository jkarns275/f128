use f128_t::*;
use ffi;
use ffi::*;
use std::cmp::Ordering::*;
use std::cmp::*;
use std::convert::{From, Into};
use std::fmt::{Debug, Error, Formatter};
use std::iter::*;
use std::mem;
use std::ops::*;

impl Debug for f128 {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.write_str(&self.to_string())
    }
}

impl Neg for f128 {
    type Output = Self;

    fn neg(self) -> Self {
        let mut bits = self.inner_as_u128();
        bits ^= f128::SIGN_BIT.inner_as_u128();
        f128::from_raw_u128(bits)
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

macro_rules! shl_impl {
    ($t:ty, $f:ty) => {
        impl Shr<$f> for $t {
            type Output = $t;

            #[inline]
            fn shr(self, other: $f) -> f128 {
                unsafe {
                    mem::transmute::<u128, f128>(
                        mem::transmute::<[u8; 16], u128>(self.inner()) >> other
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
                        mem::transmute::<[u8; 16], u128>(self.inner()) << other
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

impl Rem<f128> for f128 {
    type Output = f128;

    fn rem(self, other: f128) -> f128 {
        unsafe { ffi::fmodq_f(self, other) }
    }
}

impl<'a> Rem<&'a f128> for f128 {
    type Output = f128;

    fn rem(self, other: &'a f128) -> f128 {
        unsafe { ffi::fmodq_f(self, *other) }
    }
}

impl<'a, 'b> Rem<&'a f128> for &'b f128 {
    type Output = f128;

    fn rem(self, other: &'a f128) -> f128 {
        unsafe { ffi::fmodq_f(*self, *other) }
    }
}

impl<'a> Rem<f128> for &'a f128 {
    type Output = f128;

    fn rem(self, other: f128) -> f128 {
        unsafe { ffi::fmodq_f(*self, other) }
    }
}

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

        impl F128 for $ty {
            #[inline]
            fn from_f128(x: f128) -> Self {
                unsafe { ::std::mem::transmute($to(::std::mem::transmute(x))) }
            }
            #[inline]
            fn f128(self) -> f128 {
                unsafe { ::std::mem::transmute($from(::std::mem::transmute(self))) }
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

pub trait F128 {
    fn from_f128(x: f128) -> Self;
    fn f128(self) -> f128;
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
