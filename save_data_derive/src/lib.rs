extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{self, parse_macro_input, DeriveInput, Fields};

#[proc_macro_derive(SaveData)]
pub fn save_data_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let result = match ast.data {
        syn::Data::Struct(ref s) => impl_save_data_struct(&ast, &s.fields),
        syn::Data::Enum(_) => impl_save_data_enum(&ast),
        _ => panic!("enum/union not supported"),
    };
    result.into()
}

fn impl_save_data_struct(ast: &syn::DeriveInput, fields: &Fields) -> TokenStream {
    let fields = match *fields {
        syn::Fields::Named(ref fields) => &fields.named,
        _ => panic!("non named fields not supported"),
    };

    let fields = fields.into_iter().map(|f| {
        let field_name = &f.ident;
        let field_type = &f.ty;

        quote! {
            #field_name: <#field_type as SaveData>::deserialize(input)?,
        }
    });

    let name = &ast.ident;
    let gen = quote! {
        impl SaveData for #name {
            fn deserialize(input: &mut SaveCursor) -> Result<Self> {
                Ok(Self {
                    #(#fields)*
                })
            }
        }
    };
    gen.into()
}

fn impl_save_data_enum(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl SaveData for #name {
            fn deserialize(input: &mut SaveCursor) -> Result<Self> {
                Self::deserialize_enum_from_u8(input)
            }
        }
    };
    gen.into()
}
