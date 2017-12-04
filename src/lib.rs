#![feature(link_args)]
#![feature(i128_type)]
#![feature(i128)]
#![feature(zero_one)]
#![feature(libc)]
#![feature(concat_idents)]
#![feature(use_extern_macros)]
extern crate libc;
extern crate num;

use std::f64;
mod ffi;
mod f128_t;

pub use f128_t::f128;

fn main() {
    let x = 0xfffeffffffffffffffffffffffffffff;
    let z = 0x7ffeffffffffffffffffffffffffffff_u128;
    let y: f128 = f128::from_bits(x);
    let w: f128 = f128::from_bits(z);
    println!("{}, {}", y.to_string(), w.to_string());

}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        //let x = unsafe { add(0, 0) };
        //println!("{:x}", x);
    }
}
