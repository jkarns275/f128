extern crate f128_internal;
extern crate f128_input;
extern crate num_traits;

pub use f128_input::*;
pub use f128_internal::*;

#[macro_export]
macro_rules! f128 {
    ($e:expr) => (f128_inner!($e));
    ($f:expr, $($e:expr),+) => ((f128_inner!($f), $(f128_inner!($e)),+));
    [$f:expr, $($e:expr),+] => ([f128_inner!($f), $(f128_inner!($e)),+]);
    [$f:expr; $l:expr] => ([f128_inner!($f); $l]);
}


#[cfg(test)]
mod tests {

    use super::*;
    use num_traits::*;
    use std::num::FpCategory;

    #[test]
    fn test_align() {
        assert_eq!(std::mem::align_of::<f128>(), 16);
    }

    // Utility functions used to create constants in the form of byte arrays. 
    // Only made to run on little endian systems.
    #[cfg(target_endian = "little")]
    fn le_bytes(f: [u8; 16]) -> String {
        let mut s = String::new();
        s.push('[');
        for i in 0..15 {
            s += format!("0x{:02X}, ", f[i]).as_ref();
        }
        s += format!("0x{:02X}]", f[15]).as_ref();
        s
    }

    #[cfg(target_endian = "little")]
    fn be_bytes(f: [u8; 16]) -> String {
        let mut s = String::new();
        s.push('[');
        for i in 0..15 {
            s += format!("0x{:02X}, ", f[15 - i]).as_ref();
        }
        s += format!("0x{:02X}]", f[0]).as_ref();
        s
    }

    fn print_constant(name: &str, value: f128) {
        let le_bytes_string = le_bytes(value.into_inner());
        let be_bytes_string = be_bytes(value.into_inner());
        println!("    #[cfg(target_endian = \"little\")]");
        println!("    fn {}() -> f128 {{ f128({}) }}", name, le_bytes_string);
        println!("    #[cfg(target_endian = \"big\")]");
        println!("    fn {}() -> f128 {{ f128({}) }}\n", name, be_bytes_string);
    }

    #[test]
    fn print_constants() {
        print_constant("E", f128::E);
        print_constant("FRAC_1_PI", f128::ONE / f128::PI);
        print_constant("FRAC_1_SQRT_2", f128::ONE / f128::TWO.sqrt());
        print_constant("FRAC_2_PI", f128::TWO / f128::PI);
        print_constant("FRAC_2_SQRT_PI", f128::TWO / f128::PI.sqrt());
        print_constant("FRAC_PI_2", f128::PI / f128::TWO);
        print_constant("FRAC_PI_3", f128::PI / f128!(3.0));
        print_constant("FRAC_PI_4", f128::PI / f128!(4.0));
        print_constant("FRAC_PI_6", f128::PI / f128!(6.0));
        print_constant("FRAC_PI_8", f128::PI / f128!(8.0));
        print_constant("LN_10", f128!(10.0).ln());
        print_constant("LN_2", f128!(2.0).ln());
        print_constant("LOG10_E", f128::E.log10());
        print_constant("LOG2_E", f128::E.log2());
        print_constant("PI", f128::PI);
        print_constant("SQRT_2", f128::TWO.sqrt());
    }

    #[test]
    fn test_minus() {
        let a = f128!(-4.0);
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
        let (pi, e, one, two): (f128, f128, f128, f128) =  f128!(
                                       3.1415926535897932384626433832795028841971693993751058,
                                       2.7182818284590452353602874713526624977572,
                                       1.0,
                                       2.0);

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
        assert!(!f128::INFINITY.is_sign_negative());

        assert!(f128::NEG_INFINITY.is_infinite());
        assert!(!f128::NEG_INFINITY.is_finite());
        assert!(f128::NEG_INFINITY.is_sign_negative());
        
        assert!(!f128::MAX.is_nan());
        assert!(f128::MAX.is_finite());
        
        assert!(!f128::MIN.is_nan());
        assert!(f128::MIN.is_finite());

        assert!(f128::MIN_POSITIVE.is_finite());
        assert!(f128::MIN_POSITIVE.is_normal());
        assert!(!f128::MIN_POSITIVE.is_nan());

        assert!(f128::MIN_POSITIVE_SUBNORMAL.is_finite());
        assert!(!f128::MIN_POSITIVE_SUBNORMAL.is_normal());
        assert!(!f128::MIN_POSITIVE_SUBNORMAL.is_nan());

        assert!(!f128::ZERO.is_normal());
        assert!(!f128::ZERO.is_infinite());
        assert!(!f128::ZERO.is_nan());
        assert!(f128::ZERO.is_finite());
        assert!(f128::ZERO.is_zero());
    }

    #[test]
    fn test_classify() {
        let pi = f128::PI;
        let one = f128::ONE;
        let half = f128!(0.5);
        let min = f128::MIN_POSITIVE_SUBNORMAL;
        let zero = f128::ZERO;
        let minzero = f128::NEG_ZERO;

        assert_eq!(half.classify(), FpCategory::Normal);
        assert_eq!(one.classify(), FpCategory::Normal);
        assert_eq!(pi.classify(), FpCategory::Normal);
        assert_eq!(min.classify(), FpCategory::Subnormal);
        assert_eq!(zero.classify(), FpCategory::Zero);
        assert_eq!(minzero.classify(), FpCategory::Zero);
        assert_eq!(f128::INFINITY.classify(), FpCategory::Infinite);
        assert_eq!(f128::NEG_INFINITY.classify(), FpCategory::Infinite);
        assert_eq!(f128::NAN.classify(), FpCategory::Nan);
    }

    #[test]
    fn test_signum() {
        let inf = f128::INFINITY;
        let mininf = f128::NEG_INFINITY;
        let nan = f128::NAN;
        let minnan = -f128::NAN;
        let zero = f128::ZERO;
        let minzero = f128::NEG_ZERO;
        let one = f128::ONE;
        let minone = -f128::ONE;

        assert_eq!(inf.signum(), one);
        assert_eq!(mininf.signum(), minone);
        assert!(nan.signum().is_nan());
        assert!(minnan.signum().is_nan());
        assert_eq!(zero.signum(), one);
        assert_eq!(minzero.signum(), minone);
        assert_eq!(one.signum(), one);
        assert_eq!(minone.signum(), minone);
    }

    #[test]
    fn test_is_normal() {
        let min = f128::MIN_POSITIVE;
        let max = f128::MAX;
        let zero = f128::ZERO;
        let minzero = f128::NEG_ZERO;
        let one = f128::ONE;
        let minone = -f128::ONE;

        assert!(one.is_normal());
        assert!(minone.is_normal());
        assert!(min.is_normal());
        assert!(max.is_normal());

        assert!(!zero.is_normal());
        assert!(!minzero.is_normal());
        assert!(!f128::NAN.is_normal());
        assert!(!f128::INFINITY.is_normal());
    }


    #[test]
    fn test_f128_to_primitive() {
        let a = f128!(1003.0);

        assert_eq!(1003i64, a.to_i64().unwrap());
        assert_eq!(1003u64, a.to_u64().unwrap());
        assert_eq!(1003i128, a.to_i128().unwrap());
        assert_eq!(1003u128, a.to_u128().unwrap());
    }

    #[test]
    fn test_conversions() {
        assert!(f128::from_u128(123456789).unwrap().bitwise_eq(f128!(123456789.0)));
        assert!(f128::from_i128(5i128).unwrap().bitwise_eq(f128!(5.0)));
        assert!(f128::from_i64(-64).unwrap().bitwise_eq(f128!(-64.0)));
        assert!(f128::from_u64(10_000_000).unwrap().bitwise_eq(f128!(10000000.0)));
        assert!(f128::from_i32(5i32).unwrap().bitwise_eq(f128!(5.0)));
        assert!(f128::from_u32(0).unwrap().bitwise_eq(f128!(0.0)));
        assert!(f128::from_u16(32000).unwrap().bitwise_eq(f128!(32000.0)));
        assert!(f128::from_i16(-30000).unwrap().bitwise_eq(f128!(-30000.0)));
        assert!(f128::from_i8(-100).unwrap().bitwise_eq(f128!(-100.0)));
        assert!(f128::from_u8(255).unwrap().bitwise_eq(f128!(255.0)));
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
        let thirty = f128!(30);
        let nthirty = f128!(-30);
        let oneandhalf = f128!(1.5);
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
        let oneandhalf = f128!(1.6);
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
        let a = f128!(1.5);
        let c = f128!(1.5);
        let b = f128!(3.0);
        assert!(a == c);
        assert!(a < b);
        assert!(a <= b);
        assert!(b > a);
        assert!(b >= a);
        assert!(a != b);
    }
}
