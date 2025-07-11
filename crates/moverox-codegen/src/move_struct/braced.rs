use std::borrow::Cow;
use std::collections::HashMap;

use proc_macro2::{Ident, TokenStream};
use quote::quote;
use unsynn::ToTokens as _;

use crate::move_type;

pub(super) trait BracedStructExt {
    /// The contents `{ ... }`, including the braces, for the generated Rust struct.
    fn to_rust_contents<'a>(
        &'a self,
        phantoms: impl Iterator<Item = &'a Ident>,
        address_map: &HashMap<Ident, TokenStream>,
    ) -> TokenStream;

    /// The `pub fn new` implementation for the generated Rust struct.
    fn impl_new<'a>(
        &'a self,
        phantoms: impl Iterator<Item = &'a Ident>,
        address_map: &HashMap<Ident, TokenStream>,
    ) -> (TokenStream, TokenStream);
}

impl BracedStructExt for move_syn::BracedStruct {
    fn to_rust_contents<'a>(
        &'a self,
        phantoms: impl Iterator<Item = &'a Ident>,
        address_map: &HashMap<Ident, TokenStream>,
    ) -> TokenStream {
        let mut move_fields = to_rust_fields(self, address_map);

        let phantom_data = phantoms.map(|ty| {
            let field = Ident::new(&format!("_{ty}"), ty.span());
            quote! {
                #[serde(skip)]
                #field: ::std::marker::PhantomData<#ty>
            }
        });

        let rs_fields: &mut dyn Iterator<Item = TokenStream> =
            if let Some(field) = move_fields.next() {
                &mut std::iter::once(field)
                    .chain(move_fields)
                    .map(|RustNamedField { attrs, ident, ty }| {
                        quote! {
                            #attrs
                            pub #ident: #ty
                        }
                    })
                    .chain(phantom_data)
            } else {
                &mut std::iter::once(quote! {
                    /// BCS for empty structs actually encodes a single boolean hidden field
                    dummy_field: bool
                })
                .chain(phantom_data)
            };

        quote! {
            { #(#rs_fields),* }
        }
    }

    fn impl_new<'a>(
        &'a self,
        phantoms: impl Iterator<Item = &'a Ident>,
        address_map: &HashMap<Ident, TokenStream>,
    ) -> (TokenStream, TokenStream) {
        let mut move_fields = to_rust_fields(self, address_map).peekable();
        let has_fields = move_fields.peek().is_some();

        let args: &mut dyn Iterator<Item = TokenStream> = if has_fields {
            &mut move_fields
                .clone()
                .map(|RustNamedField { ident, ty, .. }| quote!(#ident: #ty))
        } else {
            &mut std::iter::empty()
        };

        let assign_args: &mut dyn Iterator<Item = TokenStream> = if !has_fields {
            &mut std::iter::once(quote!(dummy_field: false))
        } else {
            &mut move_fields
                .clone()
                .map(|RustNamedField { ident, .. }| quote!(#ident))
        };

        let phantom_data = phantoms.map(|ty| {
            let field = Ident::new(&format!("_{ty}"), ty.span());
            quote! {
                #field: ::std::marker::PhantomData
            }
        });

        let args = quote!(#(#args),*);
        let assignments = quote! {
            {
                #(#assign_args,)*
                #( #phantom_data ),*
            }
        };
        (args, assignments)
    }
}

#[derive(Clone)]
struct RustNamedField<'a> {
    attrs: TokenStream,
    ident: Cow<'a, Ident>,
    ty: TokenStream,
}

fn to_rust_fields<'a>(
    this: &'a move_syn::BracedStruct,
    address_map: &HashMap<Ident, TokenStream>,
) -> impl Iterator<Item = RustNamedField<'a>> + Clone {
    this.fields().map(
        |move_syn::NamedField {
             attrs, ident, ty, ..
         }| {
            RustNamedField {
                attrs: attrs
                    .iter()
                    .filter(|attr| attr.is_doc())
                    .map(|attr| attr.to_token_stream())
                    .collect(),
                ident: sanitize_ident(ident),
                ty: move_type::to_rust_with_substitutions(ty, address_map),
            }
        },
    )
}

fn sanitize_ident(ident: &Ident) -> Cow<'_, Ident> {
    let ident_str = ident.to_string();
    // https://doc.rust-lang.org/reference/keywords.html
    match ident_str.as_str() {
        "abstract" | "become" | "box" | "do" | "final" | "macro" | "override" | "priv"
        | "typeof" | "unsized" | "virtual" | "yield" | "try" | "gen" | "as" | "break" | "const"
        | "continue" | "crate" | "else" | "enum" | "extern" | "false" | "fn" | "for" | "if"
        | "impl" | "in" | "let" | "loop" | "match" | "mod" | "move" | "mut" | "pub" | "ref"
        | "return" | "self" | "Self" | "static" | "struct" | "super" | "trait" | "true"
        | "type" | "unsafe" | "use" | "where" | "while" | "async" | "await" | "dyn" => {
            Cow::Owned(Ident::new_raw(&ident_str, ident.span()))
        }
        _ => Cow::Borrowed(ident),
    }
}
