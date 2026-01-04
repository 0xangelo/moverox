use std::collections::HashMap;

use quote::quote;
use unsynn::{Ident, LiteralString, ToTokens as _, TokenStream};

use crate::generics::GenericsExt;
use crate::{named_fields, positional_fields};

/// The full Rust struct declaration and its `new` constructor.
pub(super) fn to_rust(
    this: &move_syn::Enum,
    thecrate: &TokenStream,
    package: Option<&LiteralString>,
    module: Option<&Ident>,
    address_map: &HashMap<Ident, TokenStream>,
) -> TokenStream {
    let move_syn::Enum {
        ident, generics, ..
    } = this;

    let extra_attrs: TokenStream = package
        .into_iter()
        .map(unsynn::ToTokens::to_token_stream)
        .map(|addr| quote!(#[move_(address = #addr)]))
        .chain(module.map(|ident| quote!(#[move_(module = #ident)])))
        .collect();

    let rust_generics = generics
        .as_ref()
        .map(GenericsExt::to_rust)
        .unwrap_or_default();

    // HACK: pipe potential phantom parameters into the first variant to become phantom data fields
    let mut phantoms: Option<Vec<_>> = generics.as_ref().and_then(|generics| {
        let mut phantoms = generics.phantoms().peekable();
        phantoms
            .peek()
            .is_some()
            .then(|| phantoms.cloned().collect())
    });
    let variants = this
        .variants()
        .map(|var| variant_to_rust(var, phantoms.take().as_deref(), address_map));

    // NOTE: this has to be formatted as a string first, so that `quote!` will turn it into a
    // string literal later, which is what `#[serde(crate = ...)]` accepts
    let serde_crate = format!("{thecrate}::serde").replace(" ", "");
    quote! {
        #[derive(
            Clone,
            Debug,
            PartialEq,
            Eq,
            Hash,
            #thecrate::traits::MoveDatatype,
            #thecrate::serde::Deserialize,
            #thecrate::serde::Serialize,
        )]
        #[move_(crate = #thecrate::traits)]
        #[serde(crate = #serde_crate)]
        #extra_attrs
        #[allow(non_snake_case)]
        pub enum #ident #rust_generics {
            #(#variants),*
        }
    }
}

fn variant_to_rust(
    this: &move_syn::EnumVariant,
    phantoms: Option<&[Ident]>,
    address_map: &HashMap<Ident, TokenStream>,
) -> TokenStream {
    use move_syn::FieldsKind as K;
    let move_syn::EnumVariant {
        attrs,
        ident,
        fields,
    } = this;
    let attrs = attrs
        .iter()
        .filter(|attr| attr.is_doc())
        .map(|attr| attr.to_token_stream());

    // Move enum variants can have empty fields
    let bool_if_empty = false;
    // Public enums in Rust already have all their variants' fields public
    let visibility = false;

    // If the variant is a unit (empty) one but there are phantom parameters, make it into a
    // positional fields one with just the phantom data
    let default_fields = phantoms.map(|phantoms| {
        positional_fields::to_rust(
            &Default::default(),
            phantoms.iter(),
            address_map,
            bool_if_empty,
            visibility,
        )
    });

    let fields = fields
        .as_ref()
        .map(|kind| match kind {
            K::Named(named) => named_fields::to_rust(
                named,
                phantoms.into_iter().flatten(),
                address_map,
                visibility,
            ),
            K::Positional(positional) => positional_fields::to_rust(
                positional,
                phantoms.into_iter().flatten(),
                address_map,
                bool_if_empty,
                visibility,
            ),
        })
        .or(default_fields)
        .unwrap_or_default();

    quote! {
        #(#attrs)*
        #ident #fields
    }
}
