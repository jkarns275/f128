extern crate libc;
extern crate num_traits;

mod f128_impl;
mod f128_t;
pub mod ffi;

pub use f128_impl::*;
pub use f128_t::*;

#[cfg(test)]
mod tests {

    use super::*;
    use num_traits::*;
    use std::{num::FpCategory, cmp::Ordering};

    #[test]
    fn test_minus() {
        let a = f128::from_f64(-4.).unwrap();
        assert_eq!(a.is_finite(), true);
        assert_eq!(a.is_infinite(), false);
        assert_eq!(a.is_sign_negative(), true);
        assert_eq!(a.is_sign_positive(), false);
        assert_eq!(a.signum(), -f128::ONE);

        let a = f128::from_f64(4.).unwrap();
        assert_eq!(a.is_finite(), true);
        assert_eq!(a.is_infinite(), false);
        assert_eq!(a.is_sign_negative(), false);
        assert_eq!(a.is_sign_positive(), true);
        assert_eq!(a.signum(), f128::ONE);
    }

    #[test]
    fn test_constants() {
        let pi = f128::parse("3.1415926535897932384626433832795028841971693993751058").unwrap();
        let e = f128::parse("2.7182818284590452353602874713526624977572").unwrap();
        let one = f128::parse("1.0").unwrap();
        let two = f128::parse("2.0").unwrap();

        // .0 because using actual float comparison won't work, and we're concerned about the bits
        assert!(pi.bitwise_eq(f128::PI));
        assert!(e.bitwise_eq(f128::E));
        assert!(one.bitwise_eq(f128::ONE));
        assert!(two.bitwise_eq(f128::TWO));

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
        assert!(f128::from_u128(123456789)
            .unwrap()
            .bitwise_eq(f128::parse("123456789.0").unwrap()));
        assert!(f128::from_i128(5i128)
            .unwrap()
            .bitwise_eq(f128::parse("5.0").unwrap()));
        assert!(f128::from_i64(-64)
            .unwrap()
            .bitwise_eq(f128::parse("-64.0").unwrap()));
        assert!(f128::from_u64(10_000_000)
            .unwrap()
            .bitwise_eq(f128::parse("10000000.0").unwrap()));
        assert!(f128::from_i32(5i32)
            .unwrap()
            .bitwise_eq(f128::parse("5.0").unwrap()));
        assert!(f128::from_u32(0)
            .unwrap()
            .bitwise_eq(f128::parse("0.0").unwrap()));
        assert!(f128::from_u16(32000)
            .unwrap()
            .bitwise_eq(f128::parse("32000.0").unwrap()));
        assert!(f128::from_i16(-30000)
            .unwrap()
            .bitwise_eq(f128::parse("-30000.0").unwrap()));
        assert!(f128::from_i8(-100)
            .unwrap()
            .bitwise_eq(f128::parse("-100.0").unwrap()));
        assert!(f128::from_u8(255)
            .unwrap()
            .bitwise_eq(f128::parse("255.0").unwrap()));
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
        ($a:expr, $b:expr, $epsilon:expr) => {
            assert!(($a - $b).abs() < $epsilon)
        };
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
        use std::{f32, f64};
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

    #[test]
    fn test_cmp() {
        let a = f128::parse("1.5").unwrap();
        let c = f128::parse("1.5").unwrap();
        let b = f128::parse("3.0").unwrap();
        assert!(a == c);
        assert!(a < b);
        assert!(a <= b);
        assert!(b > a);
        assert!(b >= a);
        assert!(a != b);
    }

    #[test]
    fn test_total_cmp() {
        let a = f128::parse("-1.5").unwrap();
        let b = f128::parse("1.5").unwrap();
        let c = f128::parse("3.0").unwrap();

        assert_eq!(a.total_cmp(&a), Ordering::Equal);
        assert_eq!(a.total_cmp(&b), Ordering::Less);
        assert_eq!(a.total_cmp(&c), Ordering::Less);
        assert_eq!(b.total_cmp(&a), Ordering::Greater);
        assert_eq!(b.total_cmp(&b), Ordering::Equal);
        assert_eq!(b.total_cmp(&c), Ordering::Less);
        assert_eq!(c.total_cmp(&a), Ordering::Greater);
        assert_eq!(c.total_cmp(&b), Ordering::Greater);
        assert_eq!(c.total_cmp(&c), Ordering::Equal);

        let nan = f128::NAN;
        let minnan = -f128::NAN;
        let inf = f128::INFINITY;
        let mininf = f128::NEG_INFINITY;

        assert_eq!(nan.total_cmp(&nan), Ordering::Equal);
        assert_eq!(nan.total_cmp(&minnan), Ordering::Greater);
        assert_eq!(minnan.total_cmp(&nan), Ordering::Less);
        assert_eq!(minnan.total_cmp(&minnan), Ordering::Equal);
        assert_eq!(nan.total_cmp(&inf), Ordering::Greater);
        assert_eq!(nan.total_cmp(&mininf), Ordering::Greater);
        assert_eq!(minnan.total_cmp(&inf), Ordering::Less);

        assert_eq!(nan.total_cmp(&a), Ordering::Greater);
        assert_eq!(minnan.total_cmp(&a), Ordering::Less);
        assert_eq!(inf.total_cmp(&a), Ordering::Greater);
        assert_eq!(mininf.total_cmp(&a), Ordering::Less);
    }
}
