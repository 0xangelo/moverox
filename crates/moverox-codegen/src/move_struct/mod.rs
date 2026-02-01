use std::collections::{HashMap, HashSet};

use proc_macro2::{Ident, TokenStream};
use quote::quote;

mod braced;
mod tuple;

use braced::BracedStructExt as _;
use tuple::TupleStructExt as _;

use crate::generics::GenericsExt;
use crate::{ItemContext, Result};

pub(super) trait StructGen {
    /// The full Rust struct declaration, its `new` constructor and potentially its `HasKey`
    /// implementation.
    fn to_rust(&self, otw_types: HashSet<Ident>, ctx: ItemContext<'_>) -> Result<TokenStream>;
}

impl StructGen for move_syn::Struct {
    fn to_rust(&self, otw_types: HashSet<Ident>, ctx: ItemContext<'_>) -> Result<TokenStream> {
        let decl = self.rust_declaration(otw_types, ctx)?;
        let impl_new = self.impl_new(ctx.address_map);
        let impl_has_key_maybe = self.impl_has_key(ctx.thecrate).unwrap_or_default();
        Ok(quote! {
            #decl
            #impl_new
            #impl_has_key_maybe
        })
    }
}

trait StructExt {
    fn rust_declaration(
        &self,
        otw_types: HashSet<Ident>,
        ctx: ItemContext<'_>,
    ) -> Result<TokenStream>;

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
    fn generics(&self) -> TokenStream;

    /// Generics for the main struct declaration. These may have defaults set.
    fn type_generics(
        &self,
        thecrate: &TokenStream,
        otw_types: HashSet<Ident>,
    ) -> Result<TokenStream>;

    /// Identifiers of each phantom type of the struct.
    fn phantom_types(&self) -> impl Iterator<Item = &Ident>;
}

impl StructExt for move_syn::Struct {
    fn rust_declaration(
        &self,
        otw_types: HashSet<Ident>,
        ctx: ItemContext<'_>,
    ) -> Result<TokenStream> {
        use move_syn::StructKind as K;
        let Self { ident, kind, .. } = self;

        let extra_attrs: TokenStream = ctx
            .package
            .into_iter()
            .map(unsynn::ToTokens::to_token_stream)
            .map(|addr| quote!(#[move_(address = #addr)]))
            .chain(ctx.module.map(|ident| quote!(#[move_(module = #ident)])))
            .collect();
        let extra_derives = self.extra_derives().unwrap_or_default();
        let type_generics = self.type_generics(ctx.thecrate, otw_types)?;
        let contents = match kind {
            K::Braced(braced) => braced.to_rust_contents(self.phantom_types(), ctx.address_map),
            K::Tuple(tuple) => tuple.to_rust_contents(self.phantom_types(), ctx.address_map),
        };
        // NOTE: this has to be formatted as a string first, so that `quote!` will turn it into a
        // string literal later, which is what `#[serde(crate = ...)]` accepts
        let thecrate = ctx.thecrate;
        let serde_crate = format!("{thecrate}::serde").replace(" ", "");
        Ok(quote! {
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
            pub struct #ident #type_generics #contents
        })
    }

    fn impl_new(&self, address_map: &HashMap<Ident, TokenStream>) -> TokenStream {
        use move_syn::StructKind;
        let Self { ident, kind, .. } = self;
        let generics = self.generics();
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
        let generics = self.generics();
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

    fn generics(&self) -> TokenStream {
        self.generics
            .as_ref()
            .map(GenericsExt::to_rust)
            .unwrap_or_default()
    }

    fn type_generics(
        &self,
        thecrate: &TokenStream,
        otw_types: HashSet<Ident>,
    ) -> Result<TokenStream> {
        self.generics
            .as_ref()
            .map(|g| g.type_generics(thecrate, otw_types))
            .transpose()
            .map(Option::unwrap_or_default)
    }

    fn phantom_types(&self) -> impl Iterator<Item = &Ident> {
        self.generics
            .as_ref()
            .into_iter()
            .flat_map(GenericsExt::phantoms)
    }
}
