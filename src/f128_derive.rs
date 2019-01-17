use f128_t::*;
use std::ops::*;
use ffi;
use ffi::*;
use std::convert::{ From, Into };
use std::iter::*;
use std::hash::{ Hash, Hasher };
use std::mem;
use std::slice;
use std::ffi::CString;
use std::cmp::*;
use std::cmp::Ordering::*;
use std::fmt::{Debug, Formatter, Error};

impl Debug for f128 {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.write_str(&self.to_string())
    }
}

impl Neg for f128 {
    type Output = Self;
    #[cfg(target_endian = "little")]
    fn neg(mut self) -> Self {
        self.0[15] ^= 0x8;
        self
    }

    #[cfg(target_endian = "big")]
    fn neg(mut self) -> Self {
        self.0[0] ^= 0x8;
        self
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
    }
}

macro_rules! shl_impl {
    ($t:ty, $f:ty) => (
    impl Shr<$f> for $t {
        type Output = $t;

        #[inline]
        fn shr(self, other: $f) -> f128 {
            f128::from_bits::<u128>(&(unsafe { mem::transmute::<[u8; 16], u128>(self.inner()) } >> other))
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
            f128::from_bits::<u128>( &(unsafe { mem::transmute::<[u8; 16], u128>(self.into_inner()) } << other))
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

impl Add for f128 {
    type Output = f128;

    #[inline]
    fn add(self, other: f128) -> f128 {
        f128::from_arr(unsafe { ffi::f128_add(self.into_inner(), other.into_inner()) })
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
        f128::from_arr(unsafe { ffi::f128_sub(self.into_inner(), other.into_inner()) })
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
        f128::from_arr(unsafe { ffi::f128_mul(self.into_inner(), other.into_inner()) })
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
        f128::from_arr(unsafe { ffi::f128_div(self.into_inner(), other.into_inner()) })
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
        unsafe { f128::from_arr(ffi::fmodq_f(self.inner(), other.inner())) }
    }
}

impl<'a> Rem<&'a f128> for f128 {
    type Output = f128;

    fn rem(self, other: &'a f128) -> f128 {
        unsafe { f128::from_arr(ffi::fmodq_f(self.inner(), other.inner())) }
    }
}

impl<'a, 'b> Rem<&'a f128> for &'b f128 {
    type Output = f128;

    fn rem(self, other: &'a f128) -> f128 {
        unsafe { f128::from_arr(ffi::fmodq_f(self.inner(), other.inner())) }
    }
}

impl<'a> Rem<f128> for &'a f128 {
    type Output = f128;

    fn rem(self, other: f128) -> f128 {
        unsafe { f128::from_arr(ffi::fmodq_f(self.inner(), other.inner())) }
    }
}

impl Sum for f128 {
    fn sum<I: Iterator<Item=f128>>(iter: I) -> f128 {
        iter.fold(0.into(), |a, b| a + b)
    }
}

impl Product for f128 {
    fn product<I: Iterator<Item=f128>>(iter: I) -> f128 {
        iter.fold(1.into(), |a, b| a * b)
    }
}

impl<'a> Sum<&'a f128> for f128 {
    fn sum<I: Iterator<Item=&'a f128>>(iter: I) -> f128 {
        iter.fold(0.into(), |a, b| a + *b)
    }
}

impl<'a> Product<&'a f128> for f128 {
    fn product<I: Iterator<Item=&'a f128>>(iter: I) -> f128 {
        iter.fold(1.into(), |a, b| a * *b)
    }
}

impl Hash for f128 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_i128( unsafe { mem::transmute::<[u8; 16], i128>(self.inner()) })
    }
    fn hash_slice<H: Hasher>(data: &[f128], state: &mut H) {
        let newlen = data.len() * mem::size_of::<f128>();
        let ptr = data.as_ptr() as *const u8;
        state.write(unsafe { slice::from_raw_parts(ptr, newlen) })
    }
}

macro_rules! impl_from {
    ($($sm: ident)*) => ($(
        impl From<$sm> for f128 {
            #[inline]
            fn from(small: $sm) -> f128 {
                unsafe { f128::from_arr(concat_idents!($sm, _to_f128)(small)) }
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


macro_rules! impl_from_arr {
    ($($sm: ident)*) => ($(
        impl From<$sm> for f128 {
            #[inline]
            fn from(small: $sm) -> f128 {
                let x = unsafe { mem::transmute::<$sm, [u8; 16]>(small) };
                unsafe { f128::from_arr(concat_idents!($sm, _to_f128)(x)) }
            }
        }
        impl Into<$sm> for f128 {
            #[inline]
            fn into(self) -> $sm {
               unsafe { mem::transmute::<[u8; 16], $sm>(concat_idents!(f128_to_, $sm)(self.0)) }
            }
        }
    )*)
}

impl_from! { u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 }
impl_from_arr! { i128 u128 }

pub trait F128 {
    fn from_f128(x: f128) -> Self;
    fn f128(self) -> f128;
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
                unsafe { f128::from_arr(concat_idents!($t, _to_f128)(self)) }
            }
        }
    )*)
}


macro_rules! into_f128_arr_gen {
    ($($t:ident)*) => ($(
        impl F128 for $t {
            #[inline]
            fn from_f128(x: f128) -> Self {
                unsafe { mem::transmute::<[u8; 16], $t>(concat_idents!(f128_to_, $t)(x.into_inner())) }
            }
            #[inline]
            fn f128(self) -> f128 {
                unsafe { f128::from_arr(concat_idents!($t, _to_f128)( mem::transmute::<$t, [u8; 16]>(self) )) }
            }
        }
    )*)
}


into_f128_gen! { u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 }
into_f128_arr_gen! { i128 u128 }

impl PartialOrd for f128 {
    fn partial_cmp(&self, other: &f128) -> Option<Ordering> {
        let lte = unsafe { lteq(self.into_inner(), other.into_inner()) };
        let gte = unsafe { gteq(self.into_inner(), other.into_inner()) };

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
        unsafe { eqq(self.0, other.0) != 0 }
    }

    fn ne(&self, other: &Self) -> bool {
        unsafe { neqq(self.0, other.0) != 0 }
    }
}

