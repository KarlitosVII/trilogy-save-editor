use heck::TitleCase;
use proc_macro::TokenStream;
use quote::quote;
use syn::{
    self, parse_macro_input, punctuated::Punctuated, token::Comma, DeriveInput, Fields, Variant,
};

#[proc_macro_derive(RawUi)]
pub fn raw_ui_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    match ast.data {
        syn::Data::Struct(ref s) => impl_raw_ui_struct(&ast, &s.fields),
        syn::Data::Enum(ref e) => impl_raw_ui_enum(&ast, &e.variants),
        _ => panic!("union not supported"),
    }
    .into()
}

fn impl_raw_ui_struct(ast: &syn::DeriveInput, fields: &Fields) -> proc_macro2::TokenStream {
    let fields = match *fields {
        syn::Fields::Named(ref fields) => &fields.named,
        _ => panic!("non named fields not supported"),
    };

    let name = &ast.ident;

    let draw_lets = fields.iter().filter_map(|f| {
        (!f.ident.as_ref().unwrap().to_string().starts_with('_')).then(|| {
            let field_name = &f.ident;
            quote! {
               let #field_name = &mut self.#field_name;
            }
        })
    });

    let draw_fields = fields.iter().filter_map(|f| {
        (!f.ident.as_ref().unwrap().to_string().starts_with('_')).then(|| {
            let field_name = &f.ident;
            let field_string = field_name.as_ref().unwrap().to_string().to_title_case();
            quote! {
                Box::new(move || { crate::save_data::RawUi::draw_raw_ui(#field_name, gui, #field_string) })
            }
        })
    });

    quote! {
        #[automatically_derived]
        impl crate::save_data::RawUi for #name {
            fn draw_raw_ui(&mut self, gui: &crate::gui::Gui, ident: &str) {
                #(#draw_lets)*
                let fields: &mut [Box<dyn FnMut()>] = &mut [#(#draw_fields),*];
                gui.draw_struct(ident, fields);
            }
        }
    }
}

fn impl_raw_ui_enum(
    ast: &syn::DeriveInput, variants: &Punctuated<Variant, Comma>,
) -> proc_macro2::TokenStream {
    let name = &ast.ident;

    let array_variants = variants.iter().map(|v| {
        let variant_name = &v.ident.to_string();
        quote! {
            imgui::im_str!(#variant_name)
        }
    });

    let edit_variants = variants.iter().enumerate().map(|(i, v)| {
        let variant = &v.ident;
        quote! {
            #i => #name::#variant
        }
    });

    quote! {
        #[automatically_derived]
        impl crate::save_data::RawUi for #name {
            fn draw_raw_ui(&mut self, gui: &crate::gui::Gui, ident: &str) {
                const ITEMS: &[&imgui::ImStr] = &[#(#array_variants),*];

                let mut edit_item = self.clone() as usize;

                if gui.draw_edit_enum(ident, &mut edit_item, ITEMS) {
                    *self = match edit_item {
                        #(#edit_variants),*,
                        _ => unreachable!(),
                    };
                }
            }
        }
    }
}

#[proc_macro_derive(RawUiMe1Legacy)]
pub fn raw_ui_me1_legacy_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    match ast.data {
        syn::Data::Struct(ref s) => impl_raw_ui_me1_legacy(&ast, &s.fields),
        _ => panic!("enum / union not supported"),
    }
    .into()
}

fn impl_raw_ui_me1_legacy(ast: &syn::DeriveInput, fields: &Fields) -> proc_macro2::TokenStream {
    let fields = match *fields {
        syn::Fields::Named(ref fields) => &fields.named,
        _ => panic!("non named fields not supported"),
    };

    let name = &ast.ident;

    let draw_lets = fields.iter().filter_map(|f| {
        (!f.ident.as_ref().unwrap().to_string().starts_with('_')).then(|| {
            let field_name = &f.ident;
            quote! {
               let #field_name = &mut self.#field_name;
            }
        })
    });

    let draw_fields = fields.iter().filter_map(|f| {
        (!f.ident.as_ref().unwrap().to_string().starts_with('_')).then(|| {
            let field_name = &f.ident;
            let field_string = field_name.as_ref().unwrap().to_string().to_title_case();
            quote! {
                Box::new(move || { crate::save_data::RawUi::draw_raw_ui(#field_name, gui, #field_string) })
            }
        })
    });

    quote! {
        #[automatically_derived]
        impl crate::save_data::RawUiMe1Legacy for #name {
            fn draw_fields<'a>(&'a mut self, gui: &'a crate::gui::Gui) -> Vec<Box<dyn FnMut() + 'a>> {
                #(#draw_lets)*
                vec![#(#draw_fields),*]
            }
        }
    }
}
