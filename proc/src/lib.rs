extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro_hack::proc_macro_hack;
use style_shared::{Style, Styles};
use quote::ToTokens;

#[proc_macro_hack]
pub fn styles(s: TokenStream) -> TokenStream {
    let styles: Styles = match syn::parse(s) {
        Ok(s) => s,
        Err(e) => return e.to_compile_error().into(),
    };
    styles.to_token_stream().into()
}

#[proc_macro_hack]
pub fn property(s: TokenStream) -> TokenStream {
    let style: Style = match syn::parse(s) {
        Ok(s) => s,
        Err(e) => return e.to_compile_error().into(),
    };
    style.to_token_stream().into()
}

#[proc_macro_hack]
pub fn property_value(s: TokenStream) -> TokenStream {
    todo!("{:?}", s)
}
