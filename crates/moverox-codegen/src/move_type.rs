use std::collections::HashMap;

use move_syn::{ItemPath, Type};
use proc_macro2::{Ident, TokenStream};
use quote::{ToTokens, quote};
use unsynn::ToTokens as _;

/// Generate Rust-equivalent type, substituting Move named addresses with Rust paths to oxidized
/// Move packages using `address_map`.
pub(super) fn to_rust_with_substitutions(
    this: &Type,
    address_map: &HashMap<Ident, TokenStream>,
) -> TokenStream {
    let type_args = this
        .type_args
        .as_ref()
        .map_or_else(TokenStream::new, |type_args| {
            let types = type_args
                .types()
                .map(|ty| to_rust_with_substitutions(ty, address_map));
            quote!(<#(#types),*>)
        });

    let path = if let ItemPath::Full {
        named_address,
        module,
        item: type_,
        ..
    } = &this.path
    {
        let prefix: &dyn ToTokens = address_map
            .get(named_address)
            .map_or(named_address, |path| path);
        quote!(#prefix::#module::#type_)
    } else {
        this.path.to_token_stream()
    };

    quote! {
        #path #type_args
    }
}
