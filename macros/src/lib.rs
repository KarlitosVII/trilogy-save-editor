#![warn(clippy::all)]

mod raw_ui;
mod rcize;

use proc_macro::TokenStream;
use syn::parse_macro_input;
use syn::{self, DeriveInput};

use crate::raw_ui::Derive;

#[proc_macro_attribute]
pub fn rcize_fields(_: TokenStream, input: TokenStream) -> TokenStream {
    rcize::rcize_fields(input)
}

#[proc_macro_derive(RawUi)]
pub fn raw_ui_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    match ast.data {
        syn::Data::Struct(ref s) => raw_ui::impl_struct(&ast, &s.fields, Derive::RawUi),
        syn::Data::Enum(ref e) => raw_ui::impl_enum(&ast, &e.variants),
        _ => panic!("union not supported"),
    }
    .into()
}

#[proc_macro_derive(RawUiRoot)]
pub fn raw_ui_derive_root(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    match ast.data {
        syn::Data::Struct(ref s) => raw_ui::impl_struct(&ast, &s.fields, Derive::RawUiRoot),
        _ => panic!("enum / union not supported"),
    }
    .into()
}

#[proc_macro_derive(RawUiChildren)]
pub fn raw_ui_children_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    match ast.data {
        syn::Data::Struct(ref s) => raw_ui::impl_struct(&ast, &s.fields, Derive::RawUiChildren),
        _ => panic!("enum / union not supported"),
    }
    .into()
}
