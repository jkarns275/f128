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
        Err(_) => panic!(format!("'{:?}' is not a valid f128", s)),
    }
}

#[proc_macro]
pub fn f128(item: TokenStream) -> TokenStream {
    let s = item.to_string();
    let r = s.split(',')
        .map(parse)
        .collect::<Vec<String>>();
    match r.len() {
        0 => "".parse().unwrap(),
        1 => r[0].parse().unwrap(),
        _ => {
            let len = r.iter().map(|s| s.len() + 2).sum();
            let mut string = String::with_capacity(len);
            for s in r.iter() {
                string.push_str(&s[..]);
                string.push_str(", ");
            }
            string.parse().unwrap()
        }
    }
}
