use heck::TitleCase;
use quote::{quote, quote_spanned};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::Comma;
use syn::{self, DeriveInput, Fields, Variant};

#[allow(clippy::enum_variant_names)]
pub enum Derive {
    RawUi,
    RawUiRoot,
    RawUiChildren,
}

pub fn impl_struct(
    ast: &DeriveInput, fields: &Fields, raw_ui_impl: Derive,
) -> proc_macro2::TokenStream {
    let fields = match *fields {
        Fields::Named(ref fields) => &fields.named,
        _ => panic!("non named fields not supported"),
    };

    let name = &ast.ident;

    let view_fields = fields.iter().filter_map(|field| {
        (!field.ident.as_ref().unwrap().to_string().starts_with('_')).then(|| {
            let field_name = &field.ident;
            let field_string = field_name.as_ref().unwrap().to_string().to_title_case();
            quote_spanned! {field.span()=>
                crate::gui::raw_ui::RawUi::view(&self.borrow().#field_name, #field_string)
            }
        })
    });

    match raw_ui_impl {
        Derive::RawUi => quote! {
            impl crate::gui::raw_ui::RawUi for crate::gui::RcUi<#name> {
                fn view(&self, label: &str) -> yew::Html {
                    self.view_opened(label, false)
                }

                fn view_opened(&self, label: &str, opened: bool) -> yew::Html {
                    use crate::gui::components::raw_ui::RawUiStruct;
                    let fields = [#(#view_fields),*];
                    yew::html! {
                        <RawUiStruct label={label.to_owned()} {opened}>
                            { for fields }
                        </RawUiStruct>
                    }
                }
            }
        },
        Derive::RawUiRoot => quote! {
            impl crate::gui::raw_ui::RawUi for crate::gui::RcUi<#name> {
                fn view(&self, label: &str) -> yew::Html {
                    self.view_opened(label, false)
                }

                fn view_opened(&self, _: &str, _: bool) -> yew::Html {
                    use crate::gui::components::Table;
                    let fields = [#(#view_fields),*];
                    yew::html! {
                        <Table>
                            { for fields }
                        </Table>
                    }
                }
            }
        },
        Derive::RawUiChildren => quote! {
            impl crate::gui::raw_ui::RawUiChildren for crate::gui::RcUi<#name> {
                fn children(&self) -> Vec<yew::Html> {
                    vec![#(#view_fields),*]
                }
            }
        },
    }
}

pub fn impl_enum(
    ast: &DeriveInput, variants: &Punctuated<Variant, Comma>,
) -> proc_macro2::TokenStream {
    let name = &ast.ident;

    let from_variants = variants.iter().enumerate().map(|(i, v)| {
        let variant = &v.ident;
        quote! {
            #i => #name::#variant
        }
    });

    let from_usize = variants.iter().enumerate().map(|(i, v)| {
        let variant = &v.ident;
        quote_spanned! {v.span()=>
            #name::#variant => #i
        }
    });

    let array_variants = variants.iter().map(|v| {
        let variant_name = if name == "ItemLevel" {
            // ItemLevel exception
            v.ident.to_string()
        } else {
            v.ident.to_string().to_title_case()
        };
        quote_spanned! {v.span()=>
            #variant_name
        }
    });

    quote! {
        impl #name {
            pub fn variants() -> &'static [&'static str] {
                &[#(#array_variants),*]
            }
        }

        impl From<usize> for #name {
            fn from(idx: usize) -> Self {
                match idx {
                    #(#from_variants),*,
                    _ => unreachable!(),
                }
            }
        }

        impl From<#name> for usize {
            fn from(from: #name) -> Self {
                match from {
                    #(#from_usize),*,
                }
            }
        }

        impl crate::gui::raw_ui::RawUi for crate::gui::RcUi<#name> {
            fn view(&self, label: &str) -> yew::Html {
                use crate::gui::components::raw_ui::RawUiEnum;

                yew::html!{
                    <RawUiEnum<#name> label={label.to_owned()} items={#name::variants()} value={self.clone()} />
                }
            }
        }
    }
}
