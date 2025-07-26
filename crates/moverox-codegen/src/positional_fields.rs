use std::collections::HashMap;

use quote::quote;
use unsynn::{Ident, ToTokens as _, TokenStream};

use crate::move_type;

/// Transform `(T, U, V)` in Move to the equivalent in Rust.
///
/// `bool_if_empty` controls whether a `bool` field is generated if there are no Move fields.
/// `visibility` controls whether a `pub` visibility modifier is added to the field.
pub(super) fn to_rust<'a>(
    this: &move_syn::PositionalFields,
    phantoms: impl Iterator<Item = &'a Ident>,
    address_map: &HashMap<Ident, TokenStream>,
    bool_if_empty: bool,
    visibility: bool,
) -> TokenStream {
    let move_fields = this.fields();

    let phantom_data = phantoms.map(|ty| {
        quote! {
            #[serde(skip)]
            ::std::marker::PhantomData<#ty>
        }
    });

    let rs_fields: &mut dyn Iterator<Item = TokenStream> = if this.is_empty() && bool_if_empty {
        &mut std::iter::once(quote!(bool)).chain(phantom_data)
    } else {
        &mut move_fields
            .map(|d| {
                let attrs = d.attrs.to_token_stream();
                let ty = move_type::to_rust_with_substitutions(&d.ty, address_map);
                #[expect(clippy::obfuscated_if_else)]
                let vis = visibility.then(|| quote!(pub)).unwrap_or_default();
                quote! {
                    #attrs #vis #ty
                }
            })
            .chain(phantom_data)
    };

    quote! {
        ( #(#rs_fields),* )
    }
}
