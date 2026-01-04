use std::collections::{HashMap, HashSet};

use move_syn::{FieldsKind, ItemPath};
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

    let mut phantoms = unused_phantoms(this);
    let variants = this
        .variants()
        // HACK: pipe unused phantom parameters into the first variant to become phantom data fields
        .map(|var| variant_to_rust(var, &std::mem::take(&mut phantoms), address_map));

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

/// Collect `this` enum's phantom types that aren't used in any of its field types.
fn unused_phantoms(this: &move_syn::Enum) -> Vec<Ident> {
    let Some(generics) = this.generics.as_ref() else {
        return Vec::new(); // short-circuit if the enum has no generics
    };

    let maybe_phantom_leaf_types: HashSet<_> = enum_leaf_types(this)
        .filter_map(|path| match path {
            ItemPath::Ident(ident) => Some(ident),
            _ => None,
        })
        .collect();

    generics
        .phantoms()
        .filter(|&ident| !maybe_phantom_leaf_types.contains(ident))
        .cloned()
        .collect()
}

/// Find all type parameters of `this` enum's fields that are 'leaf's, recursively.
///
/// A type parameter is a 'leaf' if it has no type parameters itself
fn enum_leaf_types(this: &move_syn::Enum) -> Box<dyn Iterator<Item = &ItemPath> + '_> {
    this.variants()
        .flat_map(|var| &var.fields)
        .flat_map(|fields| match fields {
            FieldsKind::Positional(positional) => {
                leaf_types_recursive(positional.fields().map(|field| &field.ty).boxed())
            }
            FieldsKind::Named(named) => {
                leaf_types_recursive(named.fields().map(|field| &field.ty).boxed())
            }
        })
        .boxed()
}

fn leaf_types_recursive<'a>(
    types: Box<dyn Iterator<Item = &'a move_syn::Type> + 'a>,
) -> Box<dyn Iterator<Item = &'a ItemPath> + 'a> {
    types
        .into_iter()
        .flat_map(|t| {
            t.type_args.as_ref().map_or_else(
                || std::iter::once(&t.path).boxed(),
                |t_args| leaf_types_recursive(t_args.types().boxed()),
            )
        })
        .boxed()
}

fn variant_to_rust(
    this: &move_syn::EnumVariant,
    phantoms: &[Ident],
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
    let default_fields = (!phantoms.is_empty()).then(|| {
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
            K::Named(named) => {
                named_fields::to_rust(named, phantoms.iter(), address_map, visibility)
            }
            K::Positional(positional) => positional_fields::to_rust(
                positional,
                phantoms.iter(),
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

trait BoxedIter<'a>: Iterator + 'a {
    fn boxed(self) -> Box<dyn Iterator<Item = Self::Item> + 'a>
    where
        Self: Sized,
    {
        Box::new(self)
    }
}

impl<'a, T: Iterator + 'a> BoxedIter<'a> for T {}
