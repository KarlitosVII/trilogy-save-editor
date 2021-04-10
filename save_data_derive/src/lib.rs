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

    match ast.data {
        syn::Data::Struct(ref s) => impl_save_data_struct(&ast, &s.fields),
        syn::Data::Enum(ref e) => impl_save_data_enum(&ast, &e.variants),
        _ => panic!("union not supported"),
    }
}

fn impl_save_data_struct(ast: &syn::DeriveInput, fields: &Fields) -> TokenStream {
    let fields = match *fields {
        syn::Fields::Named(ref fields) => &fields.named,
        _ => panic!("non named fields not supported"),
    };

    let name = &ast.ident;

    let deserialize_fields = fields.iter().map(|f| {
        let field_name = &f.ident;
        let field_type = &f.ty;

        quote! {
            #field_name: <#field_type as crate::save_data::SaveData>::deserialize(cursor)?
        }
    });

    let serialize_fields = fields.iter().map(|f| {
        let field_name = &f.ident;
        let field_type = &f.ty;

        quote! {
            <#field_type as crate::save_data::SaveData>::serialize(&self.#field_name, output)?;
        }
    });

    let draw_fields = fields.iter().map(|f| {
        let field_name = &f.ident;
        let field_string = field_name.as_ref().unwrap().to_string();

        quote! {
            self.#field_name.draw_raw_ui(gui, #field_string).await;
        }
    });

    let gen = quote! {
        #[async_trait::async_trait(?Send)]
        impl crate::save_data::SaveData for #name {
            fn deserialize(cursor: &mut crate::save_data::SaveCursor) -> anyhow::Result<Self> {
                Ok(Self {
                    #(#deserialize_fields),*
                })
            }

            fn serialize(&self, output: &mut Vec<u8>) -> anyhow::Result<()>
            {
                #(#serialize_fields)*
                Ok(())
            }

            async fn draw_raw_ui(&mut self, gui: &crate::gui::Gui, ident: &str) {
                gui.draw_struct(ident, async {
                    #(#draw_fields)*
                }).await;
            }
        }
    };
    gen.into()
}

fn impl_save_data_enum(
    ast: &syn::DeriveInput, variants: &Punctuated<Variant, Comma>,
) -> TokenStream {
    let name = &ast.ident;

    // Exception
    let deserialize_enum_from_repr = Ident::new(
        if name == "EndGameState" {
            "deserialize_enum_from_u32"
        } else {
            "deserialize_enum_from_u8"
        },
        Span::call_site(),
    );

    let serialize_enum_to_repr = Ident::new(
        if name == "EndGameState" { "serialize_enum_to_u32" } else { "serialize_enum_to_u8" },
        Span::call_site(),
    );

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
        #[async_trait::async_trait(?Send)]
        impl crate::save_data::SaveData for #name {
            fn deserialize(cursor: &mut crate::save_data::SaveCursor) -> anyhow::Result<Self> {
                Self::#deserialize_enum_from_repr(cursor)
            }

            fn serialize(&self, output: &mut Vec<u8>) -> anyhow::Result<()>
            {
                Self::#serialize_enum_to_repr(self, output)
            }

            async fn draw_raw_ui(&mut self, gui: &crate::gui::Gui, ident: &str) {
                #(#let_variants)*

                let items = [#(#array_variants),*];

                let (mut edit_item, current_item) = match self {
                    #(#match_variants),*
                };

                gui.draw_edit_enum(ident, &mut edit_item, &items).await;

                if edit_item != current_item
                {
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
