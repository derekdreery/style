//! Implementation of `syn::parse::Parse` for styles, and associated helper data/functions.
use crate::*;
use syn::{
    parse::{Parse, ParseStream},
    Ident, Token,
};

impl Parse for Style<'static> {
    fn parse(s: ParseStream) -> syn::Result<Self> {
        let name: PropertyName = s.parse()?;
        s.parse::<Token![:]>()?;
        Ok(match name.as_ref() {
            ("display", None, None) => Style::Display(s.parse()?),
            _ => todo!(),
        })
    }
}

impl Parse for Display {
    fn parse(s: ParseStream) -> syn::Result<Self> {
        let val: Ident = s.parse()?;
        Ok(match val.to_string().as_str() {
            "block" => Display::Block,
            "flex" => Display::Flex,
            "inline" => Display::Inline,
            _ => return Err(syn::Error::new(val.span(), "invalid css display value")),
        })
    }
}

/// Only supports css properties up to 3 names. e.g. `first-second-third`
struct PropertyName(String, Option<String>, Option<String>); // todo span

impl PropertyName {
    fn as_ref(&self) -> (&str, Option<&str>, Option<&str>) {
        (
            self.0.as_str(),
            self.1.as_ref().map(|s| s.as_str()),
            self.2.as_ref().map(|s| s.as_str()),
        )
    }
}

impl syn::parse::Parse for PropertyName {
    fn parse(s: ParseStream) -> syn::Result<Self> {
        let first: Ident = s.parse()?;
        match s.parse::<Token![-]>() {
            Ok(_) => (),
            Err(_) => return Ok(PropertyName(first.to_string(), None, None)),
        };
        let second: Ident = s.parse()?;
        match s.parse::<Token![-]>() {
            Ok(_) => (),
            Err(_) => {
                return Ok(PropertyName(
                    first.to_string(),
                    Some(second.to_string()),
                    None,
                ))
            }
        };
        let third: Ident = s.parse()?;
        Ok(PropertyName(
            first.to_string(),
            Some(second.to_string()),
            Some(third.to_string()),
        ))
    }
}
