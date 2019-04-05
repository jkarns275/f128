extern crate f128_internal;
extern crate proc_macro;
use f128_internal::f128;
use proc_macro::TokenStream;
use std::mem::transmute;

#[proc_macro]
pub fn f128(item: TokenStream) -> TokenStream {
    let s = item.to_string();
    match f128::parse(&s[..]) {
        Ok(it) => format!(
            "::std::mem::transmute::<[u8; 16], f128>({:?})",
            it.into_inner()
        )
        .parse()
        .unwrap(),
        Err(_) => panic!(format!("'{:?}' is not a valid f128", s)),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
