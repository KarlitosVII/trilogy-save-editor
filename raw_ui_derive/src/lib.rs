use heck::TitleCase;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{
    self, parse_macro_input, punctuated::Punctuated, token::Comma, AngleBracketedGenericArguments,
    Data, DataStruct, DeriveInput, Fields, GenericArgument, Ident, Path, PathArguments,
    PathSegment, Token, Type, TypePath, Variant,
};

#[proc_macro_attribute]
pub fn rcize_fields(_: TokenStream, input: TokenStream) -> TokenStream {
    let args = quote! { None };
    rcize_fields_derive(args.into(), input)
}

#[proc_macro_attribute]
pub fn rcize_fields_derive(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(input as DeriveInput);

    if !matches!(ast.data, syn::Data::Struct(_)) {
        panic!("enum / union not supported");
    }

    let path_to_args = |paths: &[Path]| {
        PathArguments::AngleBracketed(AngleBracketedGenericArguments {
            colon2_token: None,
            lt_token: Token![<](Span::call_site()),
            gt_token: Token![>](Span::call_site()),
            args: {
                let mut args = Punctuated::new();
                for path in paths {
                    args.push(GenericArgument::Type(Type::Path(TypePath {
                        qself: None,
                        path: path.clone(),
                    })));
                }
                args
            },
        })
    };

    let rc_ui = |arguments| {
        let mut punctuated = Punctuated::new();
        punctuated.extend([
            Ident::new("crate", Span::call_site()).into(),
            Ident::new("gui", Span::call_site()).into(),
            PathSegment { ident: Ident::new("RcUi", Span::call_site()), arguments },
        ]);
        punctuated
    };

    // Implements all getters and mutables
    let mut data = ast.data.clone();
    let impl_getters = match &mut data {
        Data::Struct(DataStruct { fields: Fields::Named(ref mut fields), .. }) => {
            fields.named.iter_mut().filter_map(|field| {
                (!field.ident.as_ref().unwrap().to_string().starts_with('_')).then(|| {
                    match &mut field.ty {
                        Type::Path(path_type) => {
                            let ty = path_type.path.segments.iter().next().unwrap().clone();
                            let segments = &mut path_type.path.segments;

                            let old_type_name = ty.ident.to_string();
                            *segments = match old_type_name.as_str() {
                                "Vec" | "Option" => {
                                    // Vec<T> => Vec<RcUi<T>>
                                    let mut punctuated = Punctuated::new();
                                    punctuated.push(PathSegment {
                                        ident: Ident::new(&old_type_name, Span::call_site()),
                                        arguments: path_to_args(&[Path {
                                            leading_colon: None,
                                            segments: rc_ui(ty.arguments),
                                        }]),
                                    });
                                    punctuated
                                }
                                "IndexMap" => {
                                    // IndexMap<K, V> => IndexMap<K, RcUi<V>>
                                    let (k, v) = match ty.arguments {
                                        PathArguments::AngleBracketed(ref args) => {
                                            let mut args = args.args.iter();
                                            let mut next = || match args.next().unwrap() {
                                                GenericArgument::Type(Type::Path(ref path)) => path.path.clone(),
                                                _ => unreachable!(),
                                            };
                                            (next(), next())
                                        }
                                        _ => unreachable!(),
                                    };

                                    let mut punctuated = Punctuated::new();
                                    punctuated.push(PathSegment {
                                        ident: Ident::new(&old_type_name, Span::call_site()),
                                        arguments: path_to_args(&[
                                            k,
                                            Path {
                                                leading_colon: None,
                                                segments: rc_ui(path_to_args(&[v])),
                                            },
                                        ]),
                                    });
                                    punctuated
                                }
                                _ => segments.clone(),
                            };
                        }
                        _ => panic!("only Path type is supported"),
                    };

                    let field_name = &field.ident;
                    let field_name_mut = Ident::new(
                        &(field_name.as_ref().unwrap().to_string() + "_mut"),
                        Span::call_site(),
                    );
                    let field_type = &field.ty;
                    let visibility = &field.vis;

                    quote! {
                        #visibility fn #field_name(&self) -> std::cell::Ref<'_, #field_type> {
                            self.#field_name.borrow()
                        }

                        #visibility fn #field_name_mut(&mut self) -> std::cell::RefMut<'_, #field_type> {
                            self.#field_name.borrow_mut()
                        }
                    }
                })
            })
        }
        _ => panic!("non named fields not supported"),
    };

    // Rc-ize all fields
    match &mut ast.data {
        Data::Struct(DataStruct { fields: Fields::Named(ref mut fields), .. }) => fields
            .named
            .iter_mut()
            .filter(|f| !f.ident.as_ref().unwrap().to_string().starts_with('_'))
            .for_each(|field| match &mut field.ty {
                Type::Path(path_type) => {
                    let ty = path_type.path.segments.iter().next().unwrap().clone();
                    let segments = &mut path_type.path.segments;

                    let old_type_name = ty.ident.to_string();
                    *segments = match old_type_name.as_str() {
                        "Vec" | "Option" => {
                            // Vec<T> => RcUI<Vec<RcUi<T>>>
                            let old_type = PathSegment {
                                ident: Ident::new(&old_type_name, Span::call_site()),
                                arguments: path_to_args(&[Path {
                                    leading_colon: None,
                                    segments: rc_ui(ty.arguments),
                                }]),
                            };
                            rc_ui(path_to_args(&[old_type.into()]))
                        }
                        "IndexMap" => {
                            // IndexMap<K, V> => RcUi<IndexMap<K, RcUi<V>>>
                            let (k, v) = match ty.arguments {
                                PathArguments::AngleBracketed(ref args) => {
                                    let mut args = args.args.iter();
                                    let mut next = || match args.next().unwrap() {
                                        GenericArgument::Type(Type::Path(ref path)) => {
                                            path.path.clone()
                                        }
                                        _ => unreachable!(),
                                    };
                                    (next(), next())
                                }
                                _ => unreachable!(),
                            };

                            let old_type = PathSegment {
                                ident: Ident::new(&old_type_name, Span::call_site()),
                                arguments: path_to_args(&[
                                    k,
                                    Path {
                                        leading_colon: None,
                                        segments: rc_ui(path_to_args(&[v])),
                                    },
                                ]),
                            };
                            rc_ui(path_to_args(&[old_type.into()]))
                        }
                        ident => match ty.arguments {
                            PathArguments::None => {
                                // T => RcUi<T>
                                rc_ui(path_to_args(&[ty.into()]))
                            }
                            _ => panic!("{} not supported", ident),
                        },
                    };
                }
                _ => panic!("only Path type is supported"),
            }),
        _ => panic!("non named fields not supported"),
    };

    let derive = match args.into_iter().next() {
        Some(arg) => {
            let derive_name = arg.to_string();
            match derive_name.as_str() {
                "RawUi" | "RawUiRoot" | "RawUiMe1Legacy" => {
                    let derive_name = Ident::new(&derive_name, Span::call_site());
                    quote! { #[derive(#derive_name)] }
                }
                "None" => Default::default(),
                _ => panic!("{} not supported", arg),
            }
        }
        None => panic!("args required"),
    };

    let name = &ast.ident;

    (quote! {
        #derive
        #ast

        impl #name {
            #(#impl_getters)*
        }
    })
    .into()
}

#[allow(clippy::enum_variant_names)]
enum Derive {
    RawUi,
    RawUiRoot,
    RawUiMe1Legacy,
}

#[proc_macro_derive(RawUi)]
pub fn raw_ui_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    match ast.data {
        syn::Data::Struct(ref s) => impl_raw_ui_struct(&ast, &s.fields, Derive::RawUi),
        syn::Data::Enum(ref e) => impl_raw_ui_enum(&ast, &e.variants),
        _ => panic!("union not supported"),
    }
    .into()
}

#[proc_macro_derive(RawUiRoot)]
pub fn raw_ui_derive_root(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    match ast.data {
        syn::Data::Struct(ref s) => impl_raw_ui_struct(&ast, &s.fields, Derive::RawUiRoot),
        _ => panic!("enum / union not supported"),
    }
    .into()
}

#[proc_macro_derive(RawUiMe1Legacy)]
pub fn raw_ui_me1_legacy_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    match ast.data {
        syn::Data::Struct(ref s) => impl_raw_ui_struct(&ast, &s.fields, Derive::RawUiMe1Legacy),
        _ => panic!("enum / union not supported"),
    }
    .into()
}

fn impl_raw_ui_struct(
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
            quote! {
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
                        <RawUiStruct label=label.to_owned() opened=opened>
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
        Derive::RawUiMe1Legacy => quote! {
            impl crate::gui::raw_ui::RawUiMe1Legacy for crate::gui::RcUi<#name> {
                fn children(&self) -> Vec<yew::Html> {
                    vec![#(#view_fields),*]
                }
            }
        },
    }
}

fn impl_raw_ui_enum(
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
        quote! {
            #name::#variant => #i
        }
    });

    let array_variants = variants.iter().map(|variant| {
        let variant_name = &variant.ident.to_string();
        quote! {
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
                    <RawUiEnum<#name> label=label.to_owned() items=#name::variants() value=self.clone() />
                }
            }
        }
    }
}
