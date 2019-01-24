#![feature(link_args)]
#![feature(i128_type)]
#![feature(i128)]
#![feature(libc)]
#![feature(concat_idents)]
#![feature(use_extern_macros)]
#![feature(const_fn)]
#![allow(warnings)]
extern crate libc;
extern crate num_traits;

use std::f64;
mod ffi;
mod f128_t;
mod f128_derive;

pub use f128_t::f128;
pub use f128_derive::*;

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;
    use std::num::FpCategory;
    use num_traits::*;
    use ffi::*;

    #[test]
    fn test_constants() {
        let pi = f128::parse("3.1415926535897932384626433832795028841971693993751058").unwrap();
        let e = f128::parse("2.7182818284590452353602874713526624977572").unwrap();
        let one = f128::parse("1.0").unwrap();
        let two = f128::parse("2.0").unwrap();

        // .0 because using actual float comparison won't work, and we're concerned about the bits
        assert_eq!(pi.0, f128::PI.0);
        assert_eq!(e.0, f128::E.0);
        assert_eq!(one.0, f128::ONE.0);
        assert_eq!(two.0, f128::TWO.0);

        assert!(f128::NAN.is_nan());
        assert!(!f128::NAN.is_finite());
        assert!(!f128::NAN.is_infinite());

        assert!(f128::INFINITY.is_infinite());
        assert!(!f128::INFINITY.is_finite());

        assert!(f128::NEG_INFINITY.is_infinite());
        assert!(!f128::NEG_INFINITY.is_finite());

    }

    #[test]
    fn test_classify() {
        let pi = f128::PI;
        let one = f128::ONE;
        let half = f128::parse("0.5").unwrap();
        let zero = f128::from_u8(0).unwrap();
        let other = -zero;
        let min = f128::MIN_POSITIVE;

        assert_eq!(half.classify(), FpCategory::Normal);
        assert_eq!(one.classify(), FpCategory::Normal);
        assert_eq!(pi.classify(), FpCategory::Normal);
        assert_eq!(min.classify(), FpCategory::Subnormal);
        assert_eq!(f128::INFINITY.classify(), FpCategory::Infinite);
        assert_eq!(f128::NEG_INFINITY.classify(), FpCategory::Infinite);
        assert_eq!(f128::NAN.classify(), FpCategory::Nan);
    }

    #[test]
    fn test_f128_to_primitive() {
        let a = f128::parse("1003.0").unwrap();

        assert_eq!(1003i64, a.to_i64().unwrap());
        assert_eq!(1003u64, a.to_u64().unwrap());
        assert_eq!(1003i128, a.to_i128().unwrap());
        assert_eq!(1003u128, a.to_u128().unwrap());
    }

    #[test]
    fn test_conversions() {
        assert!(f128::from_u128(123456789).unwrap().0 == f128::parse("123456789.0").unwrap().0);
        assert!(f128::from_i128(5i128).unwrap().0 == f128::parse("5.0").unwrap().0);
        assert!(f128::from_i64(-64).unwrap().0 == f128::parse("-64.0").unwrap().0);
        assert!(f128::from_u64(10_000_000).unwrap().0 == f128::parse("10000000.0").unwrap().0);
        assert!(f128::from_i32(5i32).unwrap().0 == f128::parse("5.0").unwrap().0);
        assert!(f128::from_u32(0).unwrap().0 == f128::parse("0.0").unwrap().0);
        assert!(f128::from_u16(32000).unwrap().0 == f128::parse("32000.0").unwrap().0);
        assert!(f128::from_i16(-30000).unwrap().0 == f128::parse("-30000.0").unwrap().0);
        assert!(f128::from_i8(-100).unwrap().0 == f128::parse("-100.0").unwrap().0);
        assert!(f128::from_u8(255).unwrap().0 == f128::parse("255.0").unwrap().0);
    }

    #[test]
    fn test_to_string() {
        assert_eq!(f128::infinity().to_string().as_str(), "inf");
        assert_eq!(f128::neg_infinity().to_string().as_str(), "-inf");
        assert_eq!(f128::nan().to_string().as_str(), "nan");
        assert_eq!(f128::neg_zero().to_string().as_str(), "-0");
        assert_eq!(f128::zero().to_string().as_str(), "0");
    }

    macro_rules! assert_approx_eq {
        ($a:expr, $b:expr, $epsilon:expr) => ( assert!(($a - $b).abs() < $epsilon) )
    }

    const EPSILON: f128 = f128::EPSILON;

    #[test]
    fn test_casts_to_f128() {
        let thirty = f128::parse("30").unwrap();
        let nthirty = f128::parse("-30").unwrap();
        let oneandhalf = f128::parse("1.5").unwrap();
        assert_approx_eq!(oneandhalf, f128::from_f64(1.5).unwrap(), EPSILON);
        assert_approx_eq!(oneandhalf, f128::from_f32(1.5).unwrap(), EPSILON);
        assert_approx_eq!(thirty, f128::from_u64(30).unwrap(), EPSILON);
        assert_approx_eq!(nthirty, f128::from_i64(-30).unwrap(), EPSILON);
        assert_approx_eq!(thirty, f128::from_u32(30).unwrap(), EPSILON);
        assert_approx_eq!(nthirty, f128::from_i32(-30).unwrap(), EPSILON);
        assert_approx_eq!(thirty, f128::from_u16(30).unwrap(), EPSILON);
        assert_approx_eq!(nthirty, f128::from_i16(-30).unwrap(), EPSILON);
        assert_approx_eq!(thirty, f128::from_u8(30).unwrap(), EPSILON);
        assert_approx_eq!(nthirty, f128::from_i8(-30).unwrap(), EPSILON);
    }

    #[test]
    fn test_casts_from_f128() {
        use std::{f64, f32};
        let oneandhalf = f128::parse("1.6").unwrap();
        assert_approx_eq!(1.6f64, oneandhalf.to_f64().unwrap(), f64::EPSILON);
        assert_approx_eq!(1.6f32, oneandhalf.to_f32().unwrap(), f32::EPSILON);
        assert_eq!(1i32, oneandhalf.to_i32().unwrap());
        assert_eq!(1u32, oneandhalf.to_u32().unwrap());
        assert_eq!(1i64, oneandhalf.to_i64().unwrap());
        assert_eq!(1u64, oneandhalf.to_u64().unwrap());
        assert_eq!(1i16, oneandhalf.to_i16().unwrap());
        assert_eq!(1u16, oneandhalf.to_u16().unwrap());
        assert_eq!(1i8, oneandhalf.to_i8().unwrap());
        assert_eq!(1u8, oneandhalf.to_u8().unwrap());

    }
}

