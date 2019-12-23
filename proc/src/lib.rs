extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro_hack::proc_macro_hack;
use quote::ToTokens;
use style_shared::{Color, DynamicStyles, Style};

#[proc_macro_hack]
pub fn styles(s: TokenStream) -> TokenStream {
    let styles: DynamicStyles = match syn::parse(s) {
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
pub fn color(s: TokenStream) -> TokenStream {
    let color: Color = match syn::parse(s) {
        Ok(s) => s,
        Err(e) => return e.to_compile_error().into(),
    };
    color.to_token_stream().into()
}
