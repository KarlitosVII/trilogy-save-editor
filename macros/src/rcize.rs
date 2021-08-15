use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{format_ident, quote, quote_spanned};
use syn::parse_macro_input;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::Colon2;
use syn::{
    self, AngleBracketedGenericArguments, Data, DataStruct, DeriveInput, Fields, GenericArgument,
    Path, PathArguments, PathSegment, Token, Type, TypePath, Visibility,
};

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
                        // Type<T> => Type<RcUi<T>>
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

                        // T => RcUi<T>
                        rcize_full_type(&mut field.ty);

                        quote
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

fn rcize_inner_type(ty: &mut Type) {
    match ty {
        Type::Path(path_type) => {
            let ty = path_type.path.segments.iter().next().unwrap().clone();
            let segments = &mut path_type.path.segments;

            let old_type = &ty.ident;
            match old_type.to_string().as_str() {
                "Vec" | "Option" => {
                    // Vec<T> => Vec<RcUi<T>>
                    let mut punctuated = Punctuated::new();
                    punctuated.push(PathSegment {
                        ident: old_type.clone(),
                        arguments: path_to_args(&[Path {
                            leading_colon: None,
                            segments: rc_ui(ty.arguments),
                        }]),
                    });
                    *segments = punctuated;
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
                        ident: old_type.clone(),
                        arguments: path_to_args(&[
                            k,
                            Path { leading_colon: None, segments: rc_ui(path_to_args(&[v])) },
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

fn rcize_full_type(ty: &mut Type) {
    // T => RcUi<T>
    match ty {
        Type::Path(path_type) => {
            let ty = path_type.path.segments.iter().next().unwrap().clone();
            let segments = &mut path_type.path.segments;

            *segments = rc_ui(path_to_args(&[ty.into()]));
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

fn rc_ui(arguments: PathArguments) -> Punctuated<PathSegment, Colon2> {
    let mut punctuated = Punctuated::new();
    punctuated.extend([
        format_ident!("crate").into(),
        format_ident!("gui").into(),
        PathSegment { ident: format_ident!("RcUi"), arguments },
    ]);
    punctuated
}
