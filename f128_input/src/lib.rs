extern crate f128_internal;
extern crate proc_macro;

use f128_internal::f128;
use proc_macro::TokenStream;
use std::str;

fn parse(s: &str) -> String {
    let s = s.replace(' ', "");
    let s = &s[..];
    match f128::parse(s) {
        Ok(it) =>
            format!(
                "unsafe {{ ::std::mem::transmute::<[u8; 16], f128>({:?}) }}",
                it.into_inner()
            ),
        Err(_) => panic!("'{:?}' is not a valid f128", s),
    }
}

#[proc_macro]
pub fn f128_inner(item: TokenStream) -> TokenStream {
    let s = item.to_string();
    parse(&s[..]).parse().unwrap()
}
