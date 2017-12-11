#![feature(link_args)]
#![feature(i128_type)]
#![feature(i128)]
#![feature(zero_one)]
#![feature(libc)]
#![feature(concat_idents)]
#![feature(use_extern_macros)]
#![feature(const_fn)]
extern crate libc;
extern crate num;

use std::f64;
mod ffi;
mod f128_t;
mod f128_derive;

pub use f128_t::f128;
pub use f128_derive::*;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        //let x = unsafe { add(0, 0) };
        //println!("{:x}", x);
        let x = f128::parse("2.71828182845904523536028747135266249775724709369995").unwrap();
        eprint!("{:?}", x.0);

    }
}

fn main() {
    use num::FromPrimitive;

    let a = f128::PI;

    let b = a*f128::from_i8(5).unwrap();

    let c = b / f128::from_i8(5).unwrap();

    println!("{} == {}; {}", a.to_string(), c.to_string(), b.to_string());
}
