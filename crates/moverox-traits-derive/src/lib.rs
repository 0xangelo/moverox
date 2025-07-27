#![cfg_attr(all(doc, not(doctest)), feature(doc_auto_cfg))]

//! Derive macros for `moverox-traits`.

mod move_datatype;

use move_datatype::impl_move_datatype;
use proc_macro::TokenStream;

/// Derives `moverox_traits` trait implementations for an oxidized Move datatype.
///
/// Creates the `_TypeTag` struct related to the type being annotated, with conversion traits
/// between the former and the generic `StructTag` type, with errors like 'expected module to be x'
/// or 'expected struct name to be y', if we know those things at compile time (see the
/// [Attributes](#attributes) section for configurations around those checks).
///
/// # Attributes
///
/// - `#[move_(crate = ...)]`: sets the path of the `moverox_traits` crate, which can be useful if
///   using this inside other macros.
/// - `#[move_(address = "...")]`: sets a static package address for the generated type tag.
///   Deserialization of the latter will fail if the package addresses do not match.
/// - `#[move_(module = "...")]`: sets a static module name for the generated type tag.
///   Deserialization of the latter will fail if the module names do not match.
/// - `#[move_(nameless)]`: make the datatype name dynamic for the generated type tag. Upon the
///   deserializing the latter, any Move datatype name will be accepted. Otherwise, deserialization
///   will fail if the incoming datatype name is not equal to the Rust type's name.
///
/// # Type tag derivation
///
/// For a struct `Name<T: MoveType>`, the macro will create a `NameTypeTag` struct with fields:
/// - `address: Address`, unless the `#[move_(address = "...")]` attribute is present
/// - `module: Identifier`, unless the `#[move_(module = "...")]` attribute is present
/// - `name: Identifier` only if the `#[move_(nameless)]` attribute is present
/// - `type_t: <T as MoveType>::TypeTag`
///
/// The macro will also create custom `Into<StructTag>`, `Into<TypeTag>`, `TryFrom<&StructTag>`,
/// `TryFrom<StructTag>`, `TryFrom<&TypeTag>`, `TryFrom<TypeTag>`, `Display` and `FromStr` impls for
/// `NameTypeTag`.
///
/// # Derived traits
///
/// For the annotated type:
/// - `moverox_traits::MoveDatatype`
///
/// For the generated `_TypeTag` struct:
/// - `moverox_traits::ConstAddress` if `#[move_(address = "...")]` is specified
/// - `moverox_traits::ConstModule` if `#[move_(module = "...")]` is specified
/// - `moverox_traits::ConstName` if `#[move_(nameless)]` was **not** specified
#[proc_macro_derive(MoveDatatype, attributes(move_))]
pub fn move_datatype_derive_macro(item: TokenStream) -> TokenStream {
    impl_move_datatype(item.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
