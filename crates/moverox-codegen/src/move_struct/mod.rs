use std::collections::HashMap;

use proc_macro2::{Ident, TokenStream};
use quote::quote;
use unsynn::LiteralString;

mod braced;
mod tuple;

use braced::BracedStructExt as _;
use tuple::TupleStructExt as _;

use crate::generics::GenericsExt;

pub(super) trait StructGen {
    /// The full Rust struct declaration, its `new` constructor and potentially its `HasKey`
    /// implementation.
    fn to_rust(
        &self,
        thecrate: &TokenStream,
        package: Option<&LiteralString>,
        module: Option<&Ident>,
        address_map: &HashMap<Ident, TokenStream>,
    ) -> TokenStream;
}

impl StructGen for move_syn::Struct {
    fn to_rust(
        &self,
        thecrate: &TokenStream,
        package: Option<&LiteralString>,
        module: Option<&Ident>,
        address_map: &HashMap<Ident, TokenStream>,
    ) -> TokenStream {
        let decl = self.rust_declaration(thecrate, package, module, address_map);
        let impl_new = self.impl_new(address_map);
        let impl_has_key_maybe = self.impl_has_key(thecrate).unwrap_or_default();
        quote! {
            #decl
            #impl_new
            #impl_has_key_maybe
        }
    }
}

trait StructExt {
    fn rust_declaration(
        &self,
        thecrate: &TokenStream,
        package: Option<&LiteralString>,
        module: Option<&Ident>,
        address_map: &HashMap<Ident, TokenStream>,
    ) -> TokenStream;

    /// The Rust code for the struct's `new` constructor.
    fn impl_new(&self, address_map: &HashMap<Ident, TokenStream>) -> TokenStream;

    /// If this is a braced struct and `key` is one of its abilities, then return the
    /// `moverox_traits::HasKey` implementation for it.
    fn impl_has_key(&self, thecrate: &TokenStream) -> Option<TokenStream>;

    /// Any additional derives to prepend to the standard ones.
    ///
    /// Currently only `Default` if this struct is empty. Avoids the `clippy::new_without_default`
    /// lint.
    fn extra_derives(&self) -> Option<TokenStream>;

    /// If any generic types, `<T, ...>`, else nothing.
    ///
    /// No bounds on the types; this is usually for usage after the datatype identifier.
    fn type_generics(&self) -> TokenStream;

    /// Identifiers of each phantom type of the struct.
    fn phantom_types(&self) -> impl Iterator<Item = &Ident>;
}

impl StructExt for move_syn::Struct {
    fn rust_declaration(
        &self,
        thecrate: &TokenStream,
        package: Option<&LiteralString>,
        module: Option<&Ident>,
        address_map: &HashMap<Ident, TokenStream>,
    ) -> TokenStream {
        use move_syn::StructKind as K;
        let Self { ident, kind, .. } = self;

        let extra_attrs: TokenStream = package
            .into_iter()
            .map(unsynn::ToTokens::to_token_stream)
            .map(|addr| quote!(#[move_(address = #addr)]))
            .chain(module.map(|ident| quote!(#[move_(module = #ident)])))
            .collect();
        let extra_derives = self.extra_derives().unwrap_or_default();
        let generics = self.type_generics();
        let contents = match kind {
            K::Braced(braced) => braced.to_rust_contents(self.phantom_types(), address_map),
            K::Tuple(tuple) => tuple.to_rust_contents(self.phantom_types(), address_map),
        };
        // NOTE: this has to be formatted as a string first, so that `quote!` will turn it into a
        // string literal later, which is what `#[serde(crate = ...)]` accepts
        let serde_crate = format!("{thecrate}::serde").replace(" ", "");
        quote! {
            #[derive(
                #extra_derives
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
            pub struct #ident #generics #contents
        }
    }

    fn impl_new(&self, address_map: &HashMap<Ident, TokenStream>) -> TokenStream {
        use move_syn::StructKind;
        let Self { ident, kind, .. } = self;
        let generics = self.type_generics();
        let (args, assignments) = match kind {
            StructKind::Braced(braced) => braced.impl_new(self.phantom_types(), address_map),
            StructKind::Tuple(tuple) => tuple.impl_new(self.phantom_types(), address_map),
        };
        quote! {
            impl #generics #ident #generics {
                #[allow(clippy::just_underscores_and_digits, clippy::too_many_arguments)]
                pub const fn new(#args) -> Self {
                    Self #assignments
                }
            }
        }
    }

    /// Impl `moverox_traits::HasKey` if the struct has the 'key' ability.
    ///
    /// # Note
    ///
    /// This assumes the Move adapter (platform) is similar to Sui/IOTA in that:
    /// - objects are required to have an `id: UID` field
    /// - `UID` has a `id: ID` field
    /// - `ID` has a `bytes: address` field
    ///
    /// Hence this generates code to return that innermost `bytes` field.
    fn impl_has_key(&self, thecrate: &TokenStream) -> Option<TokenStream> {
        use move_syn::Ability;
        if !self.abilities().any(|a| matches!(a, Ability::Key(_))) {
            return None;
        }
        let ident = &self.ident;
        let generics = self.type_generics();
        Some(quote! {
            impl #generics #thecrate::traits::HasKey for  #ident #generics {
                fn address(&self) -> #thecrate::types::Address {
                    self.id.id.bytes
                }
            }
        })
    }

    fn extra_derives(&self) -> Option<TokenStream> {
        use move_syn::StructKind;
        let is_empty = match &self.kind {
            StructKind::Braced(braced) => braced.is_empty(),
            StructKind::Tuple(tuple) => tuple.is_empty(),
        };
        is_empty.then_some(quote!(Default,))
    }

    fn type_generics(&self) -> TokenStream {
        self.generics
            .as_ref()
            .map(GenericsExt::to_rust)
            .unwrap_or_default()
    }

    fn phantom_types(&self) -> impl Iterator<Item = &Ident> {
        self.generics
            .as_ref()
            .into_iter()
            .flat_map(GenericsExt::phantoms)
    }
}
