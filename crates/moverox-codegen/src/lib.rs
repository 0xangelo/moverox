#![cfg_attr(all(doc, not(doctest)), feature(doc_auto_cfg))]

//! Generate Rust code from Move IR parsed by move-syn.
//!
//! Defines extension traits to generate Rust code from Move intermediate representation.
//!
//! `thecrate` in arguments here is the path to a crate/module which exports:
//! - a `types` module with `Address` and `U256` types from `moverox-types`
//! - a `traits` module with `HasKey`, `MoveDatatype` and `MoveType` traits from `moverox-traits`
//! - the `serde` crate

use std::collections::HashMap;

use move_syn::{Item, Module};
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

#[sealed::sealed]
pub trait ModuleGen {
    fn to_rust(
        &self,
        thecrate: &TokenStream,
        package: Option<&LiteralString>,
        address_map: &HashMap<Ident, TokenStream>,
    ) -> TokenStream;
}

#[sealed::sealed]
impl ModuleGen for Module {
    fn to_rust(
        &self,
        thecrate: &TokenStream,
        package: Option<&LiteralString>,
        address_map: &HashMap<Ident, TokenStream>,
    ) -> TokenStream {
        let attrs = crate::attributes::to_rust(&self.attrs);
        let ident = &self.ident;
        let datatypes: TokenStream = self
            .items()
            .map(|item| item.to_rust(thecrate, package, Some(ident), address_map))
            .collect();
        quote! {
            #attrs
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
        }
    }
}

#[sealed::sealed]
pub trait ItemGen {
    fn to_rust(
        &self,
        thecrate: &TokenStream,
        package: Option<&LiteralString>,
        module: Option<&Ident>,
        address_map: &HashMap<Ident, TokenStream>,
    ) -> TokenStream;
}

#[sealed::sealed]
impl ItemGen for Item {
    fn to_rust(
        &self,
        thecrate: &TokenStream,
        package: Option<&LiteralString>,
        module: Option<&Ident>,
        address_map: &HashMap<Ident, TokenStream>,
    ) -> TokenStream {
        use move_syn::ItemKind as K;
        let Self { attrs, kind, .. } = self;
        let generated = match kind {
            K::Struct(s) => s.to_rust(thecrate, package, module, address_map),
            K::Enum(e) => self::move_enum::to_rust(e, thecrate, package, module, address_map),
            _ => return Default::default(),
        };
        let attrs = crate::attributes::to_rust(attrs);
        quote! {
            #attrs
            #generated
        }
    }
}
