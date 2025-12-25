use move_syn::Attributes;
use quote::quote;
use unsynn::{ToTokens as _, TokenStream};

/// Filter Move attributes and convert them to Rust.
///
/// For now, we process only doc attributes.
pub(super) fn to_rust(attrs: &[Attributes]) -> TokenStream {
    attrs
        .iter()
        .filter(|attr| attr.is_doc())
        .map(|attr| {
            let inner = attr.contents().to_token_stream();
            // NOTE: disable when compiling doctests to avoid Rust interpreting code blocks as
            // runnable tests.
            quote!(#[cfg_attr(not(doctest), #inner)])
        })
        .collect()
}
