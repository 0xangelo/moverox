use std::borrow::Cow;
use std::collections::HashMap;

use quote::quote;
use unsynn::{Ident, ToTokens as _, TokenStream};

use crate::move_type;

/// Move named field converted to Rust.
#[derive(Clone)]
pub(super) struct Rust<'a> {
    pub(super) attrs: TokenStream,
    pub(super) ident: Cow<'a, Ident>,
    pub(super) ty: TokenStream,
}

/// `{ name: T, .. }` in Move to Rust.
///
/// `visibility` controls whether a `pub` visibility modifier is added to the field.
pub(super) fn to_rust<'a>(
    this: &'a move_syn::NamedFields,
    phantoms: impl Iterator<Item = &'a Ident>,
    address_map: &HashMap<Ident, TokenStream>,
    visibility: bool,
) -> TokenStream {
    let mut move_fields = to_rust_fields(this, address_map);

    let phantom_data = phantoms.map(|ty| {
        let field = Ident::new(&format!("_{ty}"), ty.span());
        quote! {
            #[serde(skip)]
            #field: ::std::marker::PhantomData<#ty>
        }
    });

    let rs_fields: &mut dyn Iterator<Item = TokenStream> = if let Some(field) = move_fields.next() {
        &mut std::iter::once(field)
            .chain(move_fields)
            .map(|Rust { attrs, ident, ty }| {
                #[expect(clippy::obfuscated_if_else)]
                let vis = visibility.then(|| quote!(pub)).unwrap_or_default();
                quote! {
                    #attrs
                    #vis #ident: #ty
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

pub(super) fn to_rust_fields<'a>(
    this: &'a move_syn::NamedFields,
    address_map: &HashMap<Ident, TokenStream>,
) -> impl Iterator<Item = Rust<'a>> + Clone {
    this.fields().map(
        |move_syn::NamedField {
             attrs, ident, ty, ..
         }| {
            Rust {
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
