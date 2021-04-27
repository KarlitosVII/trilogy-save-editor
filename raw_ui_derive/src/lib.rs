extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    self, DeriveInput, Fields, Ident, Variant, __private::Span, parse_macro_input,
    punctuated::Punctuated, token::Comma,
};

#[proc_macro_derive(RawUi)]
pub fn raw_ui_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    match ast.data {
        syn::Data::Struct(ref s) => impl_raw_ui_struct(&ast, &s.fields),
        syn::Data::Enum(ref e) => impl_raw_ui_enum(&ast, &e.variants),
        _ => panic!("union not supported"),
    }
}

fn impl_raw_ui_struct(ast: &syn::DeriveInput, fields: &Fields) -> TokenStream {
    let fields = match *fields {
        syn::Fields::Named(ref fields) => &fields.named,
        _ => panic!("non named fields not supported"),
    };

    let name = &ast.ident;

    let draw_fields = fields.iter().filter_map(|f| {
        if f.ident.as_ref().unwrap().to_string().starts_with('_') {
            None
        } else {
            let field_name = &f.ident;
            let field_string = field_name.as_ref().unwrap().to_string();
            Some(quote! {
                (&mut self.#field_name  as &mut dyn crate::save_data::RawUi, #field_string)
            })
        }
    });

    let gen = quote! {
        #[automatically_derived]
        impl crate::save_data::RawUi for #name {
            fn draw_raw_ui(&mut self, gui: &crate::gui::Gui, ident: &str) {
                let mut fields = [#(#draw_fields),*];
                gui.draw_struct(ident, &mut fields);
            }
        }
    };
    gen.into()
}

fn impl_raw_ui_enum(
    ast: &syn::DeriveInput, variants: &Punctuated<Variant, Comma>,
) -> TokenStream {
    let name = &ast.ident;

    // Variants
    let let_variants = variants.iter().enumerate().map(|(i, v)| {
        let variant_name = &v.ident.to_string();
        let var = Ident::new(&format!("variant_{}", i), Span::call_site());
        quote! {
            let #var = imgui::im_str!(#variant_name);
        }
    });

    let array_variants = variants.iter().enumerate().map(|(i, _)| {
        let var = Ident::new(&format!("variant_{}", i), Span::call_site());
        quote! {
            #var.as_ref()
        }
    });

    let match_variants = variants.iter().enumerate().map(|(i, v)| {
        let variant = &v.ident;
        quote! {
            #name::#variant => (#i, #i)
        }
    });

    let edit_variants = variants.iter().enumerate().map(|(i, v)| {
        let variant = &v.ident;
        quote! {
            #i => #name::#variant
        }
    });

    let gen = quote! {
        #[automatically_derived]
        impl crate::save_data::RawUi for #name {
            fn draw_raw_ui(&mut self, gui: &crate::gui::Gui, ident: &str) {
                #(#let_variants)*

                let items = [#(#array_variants),*];

                let (mut edit_item, current_item) = match self {
                    #(#match_variants),*
                };

                if gui.draw_edit_enum(ident, &mut edit_item, &items) {
                    *self = match edit_item {
                        #(#edit_variants),*,
                        _ => unreachable!(),
                    };
                }
            }
        }
    };
    gen.into()
}
