#![cfg_attr(nightly, feature(doc_cfg))]

//! Generate Rust code from Move IR parsed by move-syn.
//!
//! Defines extension traits to generate Rust code from Move intermediate representation.
//!
//! `thecrate` in arguments here is the path to a crate/module which exports:
//! - a `types` module with `Address` and `U256` types from `moverox-types`
//! - a `traits` module with `HasKey`, `MoveDatatype` and `MoveType` traits from `moverox-traits`
//! - the `serde` crate

use std::collections::HashMap;

use move_syn::{Attributes, Item, Module};
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use unsynn::LiteralString;

mod attributes;
mod generics;
mod move_enum;
mod move_struct;
mod move_type;
mod named_fields;
mod positional_fields;
#[cfg(test)]
mod tests;

use self::move_struct::StructGen as _;

type BoxError = Box<dyn std::error::Error + 'static>;
type Result<T = (), E = BoxError> = std::result::Result<T, E>;

#[sealed::sealed]
pub trait ModuleGen {
    fn to_rust(
        &self,
        thecrate: &TokenStream,
        package: Option<&LiteralString>,
        address_map: &HashMap<Ident, TokenStream>,
    ) -> Result<TokenStream>;
}

#[sealed::sealed]
impl ModuleGen for Module {
    fn to_rust(
        &self,
        thecrate: &TokenStream,
        package: Option<&LiteralString>,
        address_map: &HashMap<Ident, TokenStream>,
    ) -> Result<TokenStream> {
        let (docs, other) = crate::attributes::extract(&self.attrs)
            .map_err(|err| format!("Parsing `moverox` attributes: {err}"))?;

        if !other.is_empty() {
            return Err("Move modules cannot have custom `moverox` attributes".into());
        }

        let ident = &self.ident;
        let item_ctx = ItemContext {
            thecrate,
            package,
            module: Some(ident),
            address_map,
        };
        let datatypes: TokenStream = self
            .items()
            .map(|item| item.to_rust(item_ctx))
            .collect::<Result<_>>()?;

        Ok(quote! {
            #docs
            #[allow(rustdoc::all)]
            pub mod #ident {
                #[allow(non_camel_case_types, unused)]
                type address = #thecrate::types::Address;
                #[allow(non_camel_case_types, unused)]
                type u256 = #thecrate::types::U256;
                #[allow(non_camel_case_types, unused)]
                type vector<T> = ::std::vec::Vec<T>;

                #datatypes
            }
        })
    }
}

/// Context for Rust code generation from a Move item.
#[derive(Clone, Copy)]
pub struct ItemContext<'a> {
    /// Path to a crate/module which exports:
    /// - a `types` module with `Address` and `U256` types from `moverox-types`
    /// - a `traits` module with `HasKey`, `MoveDatatype` and `MoveType` traits from `moverox-traits`
    /// - the `serde` crate
    /// - an `Otw` type
    pub thecrate: &'a TokenStream,
    /// Move package address as an `0x`-prefixed hex string.
    pub package: Option<&'a LiteralString>,
    /// Move module name.
    pub module: Option<&'a Ident>,
    /// Mapping of Move named addresses to Rust paths.
    ///
    /// Used to map Move datatype paths to Rust-equivalents.
    pub address_map: &'a HashMap<Ident, TokenStream>,
}

#[sealed::sealed]
pub trait ItemGen {
    fn to_rust(&self, ctx: ItemContext<'_>) -> Result<TokenStream>;
}

#[sealed::sealed]
impl ItemGen for Item {
    fn to_rust(&self, ctx: ItemContext<'_>) -> Result<TokenStream> {
        use move_syn::ItemKind as K;
        let Self { attrs, kind, .. } = self;

        let (docs, generated) = match kind {
            K::Struct(s) => {
                let err_ctx = |err| format!("struct {}: {err}", s.ident);
                let (docs, otw_types) = crate::attributes::extract(attrs).map_err(err_ctx)?;
                let generated = s.to_rust(otw_types, ctx).map_err(err_ctx)?;
                (docs, generated)
            }
            K::Enum(e) => {
                let err_ctx = |err| format!("enum {}: {err}", e.ident);
                let (docs, otw_types) = crate::attributes::extract(attrs).map_err(err_ctx)?;
                let generated = self::move_enum::to_rust(e, otw_types, ctx).map_err(err_ctx)?;
                (docs, generated)
            }
            _ => return non_datatype_gen(attrs),
        };

        Ok(quote! {
            #docs
            #generated
        })
    }
}

fn non_datatype_gen(attrs: &[Attributes]) -> Result<TokenStream> {
    if attrs.iter().flat_map(self::attributes::as_moverox).count() > 0 {
        return Err(
            "Move items other than enums/structs cannot be annotated with custom \
            `moverox` attributes"
                .into(),
        );
    }
    Ok(TokenStream::new())
}
