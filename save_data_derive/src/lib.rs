extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    self, DeriveInput, Fields, Ident, Variant, __private::Span, parse_macro_input,
    punctuated::Punctuated, token::Comma,
};

#[proc_macro_derive(SaveData)]
pub fn save_data_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let result = match ast.data {
        syn::Data::Struct(ref s) => impl_save_data_struct(&ast, &s.fields),
        syn::Data::Enum(ref e) => impl_save_data_enum(&ast, &e.variants),
        _ => panic!("union not supported"),
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
            #field_name: <#field_type as crate::save_data::SaveData>::deserialize(input)?
        }
    });

    let name = &ast.ident;
    let gen = quote! {
        impl crate::save_data::SaveData for #name {
            fn deserialize(input: &mut crate::save_data::SaveCursor) -> anyhow::Result<Self> {
                Ok(Self {
                    #(#fields),*
                })
            }

            fn draw_raw_ui(&mut self, _ui: &crate::ui::Ui, _ident: &'static str) {}
        }
    };
    gen.into()
}

fn impl_save_data_enum(
    ast: &syn::DeriveInput, variants: &Punctuated<Variant, Comma>,
) -> TokenStream {
    let name = &ast.ident;

    // Exception
    let deserialize_enum_from_repr = if name == "EndGameState" {
        Ident::new("deserialize_enum_from_u32", Span::call_site())
    } else {
        Ident::new("deserialize_enum_from_u8", Span::call_site())
    };

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
        impl crate::save_data::SaveData for #name {
            fn deserialize(input: &mut crate::save_data::SaveCursor) -> anyhow::Result<Self> {
                Self::#deserialize_enum_from_repr(input)
            }

            fn draw_raw_ui(&mut self, ui: &crate::ui::Ui, ident: &'static str) {
                #(#let_variants)*

                let items = vec![#(#array_variants),*];

                let (mut edit_item, current_item) = match self {
                    #(#match_variants),*
                };

                ui.draw_enum(ident, &mut edit_item, &items);

                if edit_item != current_item
                {
                    *self = match edit_item {
                        #(#edit_variants),*,
                        _ => unreachable!(),
                    }
                }
            }
        }
    };
    gen.into()
}
