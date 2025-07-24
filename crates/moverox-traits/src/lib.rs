#![cfg_attr(all(doc, not(doctest)), feature(doc_auto_cfg))]

//! Traits for rusty Move types and their type tags.
//!
//! The core items are [`MoveType`](crate::MoveType) and [`MoveTypeTag`](crate::MoveTypeTag). These
//! are useful trait bounds to use when dealing with generic off-chain Move type representations.
//! They are implemented for the primitive types that correspond to Move's primitives
//! (integers/bool).
//!
//! For Move structs, [`MoveDatatype`](crate::MoveDatatype) should be used as it has an
//! associated [`MoveDatatypeTag`](crate::MoveDatatypeTag). The
//! [`MoveDatatype`](moverox_traits_derive::MoveDatatype) derive macro is exported for automatically
//! creating a `MoveDatatypeTag` implementation from normal Rust struct declarations.
use std::error::Error as StdError;
use std::fmt::Debug;

#[cfg(feature = "derive")]
pub use moverox_traits_derive::MoveDatatype;
use moverox_types::{Address, IdentStr, Identifier, StructTag, TypeTag};

#[cfg(feature = "derive")]
#[doc(hidden)]
pub mod external;
mod primitives;
mod vector;

pub use self::primitives::{
    AddressTypeTag,
    BoolTypeTag,
    U8TypeTag,
    U16TypeTag,
    U32TypeTag,
    U64TypeTag,
    U128TypeTag,
    U256TypeTag,
};
pub use self::vector::VecTypeTag;

// =============================================================================
//  MoveType
// =============================================================================

/// Trait marking a Move data type. Has a specific way to construct a `TypeTag`.
pub trait MoveType {
    type TypeTag: MoveTypeTag;
}

pub trait MoveTypeTag: Into<TypeTag> + TryFrom<TypeTag, Error = TypeTagError> {}

impl<T> MoveTypeTag for T where T: Into<TypeTag> + TryFrom<TypeTag, Error = TypeTagError> {}

// =============================================================================
//  MoveDatatype
// =============================================================================

/// Trait marking a Move struct type. Has a specific way to construct a `StructTag`.
pub trait MoveDatatype: MoveType<TypeTag = Self::StructTag> {
    type StructTag: MoveDatatypeTag;
}

pub trait MoveDatatypeTag:
    Into<StructTag> + TryFrom<StructTag, Error = StructTagError> + MoveTypeTag
{
}

impl<T> MoveDatatypeTag for T where
    T: Into<StructTag> + TryFrom<StructTag, Error = StructTagError> + MoveTypeTag
{
}

// =============================================================================
//  Abilities
// =============================================================================

pub trait HasKey: MoveDatatype {
    fn address(&self) -> Address;
}

// =============================================================================
//  Static attributes
// =============================================================================

/// Struct tag with a constant address.
pub trait ConstAddress {
    const ADDRESS: Address;
}

/// Struct tag with a constant module.
pub trait ConstModule {
    const MODULE: &IdentStr;
}

/// Struct tag with a constant name.
pub trait ConstName {
    const NAME: &IdentStr;
}

// =============================================================================
//  Errors
// =============================================================================

#[derive(thiserror::Error, Debug)]
pub enum TypeTagError {
    #[error("Wrong TypeTag variant: expected {expected}, got {got}")]
    Variant { expected: String, got: TypeTag },
    #[error("StructTag params: {0}")]
    StructTag(#[from] StructTagError),
}

#[derive(thiserror::Error, Debug)]
pub enum StructTagError {
    #[error("Wrong address: expected {expected}, got {got}")]
    Address { expected: Address, got: Address },
    #[error("Wrong module: expected {expected}, got {got}")]
    Module {
        expected: Identifier,
        got: Identifier,
    },
    #[error("Wrong name: expected {expected}, got {got}")]
    Name {
        expected: Identifier,
        got: Identifier,
    },
    #[error("Wrong type parameters: {0}")]
    TypeParams(#[from] TypeParamsError),
}

#[derive(thiserror::Error, Debug)]
pub enum TypeParamsError {
    #[error("Wrong number of generics: expected {expected}, got {got}")]
    Number { expected: usize, got: usize },
    #[error("Wrong type for generic: {0}")]
    TypeTag(Box<TypeTagError>),
}

impl From<TypeTagError> for TypeParamsError {
    fn from(value: TypeTagError) -> Self {
        Self::TypeTag(Box::new(value))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ParseTypeTagError {
    #[error("Parsing TypeTag: {0}")]
    FromStr(Box<dyn StdError + Send + Sync + 'static>),
    #[error("Converting from TypeTag: {0}")]
    TypeTag(#[from] TypeTagError),
}

impl ParseTypeTagError {
    fn from_str(err: impl StdError + Send + Sync + 'static) -> Self {
        Self::FromStr(err.into())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ParseStructTagError {
    #[error("Parsing StructTag: {0}")]
    FromStr(Box<dyn StdError + Send + Sync + 'static>),
    #[error("Converting from StructTag: {0}")]
    StructTag(#[from] StructTagError),
}
