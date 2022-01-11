use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{format_ident, quote, quote_spanned};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::Colon2;
use syn::{
    self, AngleBracketedGenericArguments, Data, DataStruct, DeriveInput, Fields, GenericArgument,
    Path, PathArguments, PathSegment, Token, Type, TypePath, Visibility,
};
use syn::{parse_macro_input, Field};

pub fn rcize_fields(input: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(input as DeriveInput);

    if !matches!(ast.data, syn::Data::Struct(_)) {
        panic!("enum / union not supported");
    }

    // Rc-ize all fields + Implements all getters and mutables
    let impl_getters = {
        let getters = match &mut ast.data {
            Data::Struct(DataStruct { fields: Fields::Named(ref mut fields), .. }) => {
                fields.named.iter_mut().filter_map(|field| {
                    (!field.ident.as_ref().unwrap().to_string().starts_with('_')).then(|| {
                        if type_is_primitive(&field.ty) {
                            rcize_primitive(field)
                        } else {
                            rcize_struct(field)
                        }
                    })
                })
            }
            _ => panic!("non named fields not supported"),
        };

        quote! { #(#getters)* }
    };

    let name = &ast.ident;

    (quote! {
        #ast

        #[allow(dead_code)]
        impl #name {
            #impl_getters
        }
    })
    .into()
}

fn type_is_primitive(ty: &Type) -> bool {
    match ty {
        Type::Path(path_type) => {
            let ty = path_type.path.segments.iter().next().unwrap().clone().ident;
            matches!(ty.to_string().as_str(), "i32" | "u8" | "u32" | "f32" | "bool")
        }
        _ => unreachable!(),
    }
}

fn arg_is_primitive(args: &PathArguments) -> bool {
    match args {
        PathArguments::AngleBracketed(args) => match args.args.last().unwrap() {
            GenericArgument::Type(ty) => type_is_primitive(ty),
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
}

fn rcize_primitive(field: &mut Field) -> proc_macro2::TokenStream {
    let field_name = &field.ident;
    let set_field_name = format_ident!("set_{}", field_name.as_ref().unwrap());
    let field_type = &field.ty;

    let quote = if let Visibility::Public(_) = field.vis {
        quote_spanned! {field.span()=>
            pub fn #field_name(&self) -> #field_type {
                self.#field_name.get()
            }

            pub fn #set_field_name(&mut self, val: #field_type) {
                self.#field_name.set(val)
            }
        }
    } else {
        quote_spanned! {field.span()=>}
    };

    // T => RcCell<T>
    rcize_full_type(&mut field.ty, true);

    quote
}

fn rcize_struct(field: &mut Field) -> proc_macro2::TokenStream {
    // Type<T> => Type<RcRef<T>>
    rcize_inner_type(&mut field.ty);

    let field_name = &field.ident;
    let field_name_mut = format_ident!("{}_mut", field_name.as_ref().unwrap());
    let field_type = &field.ty;

    let quote = if let Visibility::Public(_) = field.vis {
        quote_spanned! {field.span()=>
            pub fn #field_name(&self) -> std::cell::Ref<'_, #field_type> {
                self.#field_name.borrow()
            }

            pub fn #field_name_mut(&mut self) -> std::cell::RefMut<'_, #field_type> {
                self.#field_name.borrow_mut()
            }
        }
    } else {
        quote_spanned! {field.span()=>}
    };

    // T => RcRef<T>
    rcize_full_type(&mut field.ty, false);

    quote
}

fn rcize_inner_type(ty: &mut Type) {
    match ty {
        Type::Path(path_type) => {
            let ty = path_type.path.segments.iter().next().unwrap().clone();
            let segments = &mut path_type.path.segments;

            let old_type = &ty.ident;
            match old_type.to_string().as_str() {
                "Vec" | "Option" => {
                    // Vec<T> => Vec<RcRef<T>>
                    let is_primitive = arg_is_primitive(&ty.arguments);

                    let mut punctuated = Punctuated::new();
                    punctuated.push(PathSegment {
                        ident: old_type.clone(),
                        arguments: path_to_args(&[Path {
                            leading_colon: None,
                            segments: if is_primitive {
                                rc_cell(ty.arguments)
                            } else {
                                rc_ref(ty.arguments)
                            },
                        }]),
                    });
                    *segments = punctuated;
                }
                "IndexMap" => {
                    // IndexMap<K, V> => IndexMap<K, RcRef<V>>
                    let is_primitive = arg_is_primitive(&ty.arguments);

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
                        ident: old_type.clone(),
                        arguments: path_to_args(&[
                            k,
                            Path {
                                leading_colon: None,
                                segments: if is_primitive {
                                    rc_cell(path_to_args(&[v]))
                                } else {
                                    rc_ref(path_to_args(&[v]))
                                },
                            },
                        ]),
                    });
                    *segments = punctuated;
                }
                _ => (),
            }
        }
        _ => panic!("only Path type is supported"),
    };
}

fn rcize_full_type(ty: &mut Type, is_primitive: bool) {
    // T => RcCell<T> or T => RcRef<T>
    match ty {
        Type::Path(path_type) => {
            let ty = path_type.path.segments.iter().next().unwrap().clone();
            let segments = &mut path_type.path.segments;

            *segments = if is_primitive {
                rc_cell(path_to_args(&[ty.into()]))
            } else {
                rc_ref(path_to_args(&[ty.into()]))
            };
        }
        _ => panic!("only Path type is supported"),
    };
}

fn path_to_args(paths: &[Path]) -> PathArguments {
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
}

fn rc_cell(arguments: PathArguments) -> Punctuated<PathSegment, Colon2> {
    let mut punctuated = Punctuated::new();
    punctuated.extend([
        format_ident!("crate").into(),
        format_ident!("save_data").into(),
        PathSegment { ident: format_ident!("RcCell"), arguments },
    ]);
    punctuated
}

fn rc_ref(arguments: PathArguments) -> Punctuated<PathSegment, Colon2> {
    let mut punctuated = Punctuated::new();
    punctuated.extend([
        format_ident!("crate").into(),
        format_ident!("save_data").into(),
        PathSegment { ident: format_ident!("RcRef"), arguments },
    ]);
    punctuated
}
