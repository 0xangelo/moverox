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

/// Marker for a Move type with its associated type tag.
pub trait MoveType {
    type TypeTag: MoveTypeTag;
}

/// A specialized Move type tag, convertible from/to a generic [`TypeTag`] by reference.
pub trait MoveTypeTag {
    fn to_type_tag(&self) -> TypeTag;

    fn from_type_tag(value: &TypeTag) -> Result<Self, TypeTagError>
    where
        Self: Sized;
}

impl<T> MoveTypeTag for T
where
    T: MoveDatatypeTag,
{
    fn from_type_tag(value: &TypeTag) -> Result<Self, TypeTagError>
    where
        Self: Sized,
    {
        match value {
            TypeTag::Struct(stag) => Ok(Self::from_struct_tag(stag)?),
            other => Err(TypeTagError::Variant {
                expected: "Struct(_)".to_owned(),
                got: type_tag_variant_name(other),
            }),
        }
    }

    fn to_type_tag(&self) -> TypeTag {
        TypeTag::Struct(Box::new(self.to_struct_tag()))
    }
}

// =============================================================================
//  MoveDatatype
// =============================================================================

/// Marker for a Move datatype with its associated struct tag.
pub trait MoveDatatype: MoveType<TypeTag = Self::StructTag> {
    type StructTag: MoveDatatypeTag;
}

/// A specialized Move type tag for datatypes, convertible from/to a generic [`StructTag`] by
/// reference.
pub trait MoveDatatypeTag: MoveTypeTag {
    fn to_struct_tag(&self) -> StructTag;

    fn from_struct_tag(value: &StructTag) -> Result<Self, StructTagError>
    where
        Self: Sized;
}

// =============================================================================
//  Abilities
// =============================================================================

/// An oxidized object, i.e., originally a Move type with the `key` ability.
pub trait HasKey {
    /// This object's address on-chain.
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

/// [`MoveType`] with a constant type tag.
pub trait ConstTypeTag: MoveType {
    const TYPE_TAG: Self::TypeTag;
}

impl<T> ConstTypeTag for T
where
    T: ConstStructTag,
{
    const TYPE_TAG: Self::TypeTag = Self::STRUCT_TAG;
}

/// [`MoveDatatype`] with a constant struct tag.
pub trait ConstStructTag: MoveDatatype {
    const STRUCT_TAG: Self::StructTag;
}

// =============================================================================
//  Errors used in traits
// =============================================================================

#[derive(thiserror::Error, Debug)]
pub enum TypeTagError {
    #[error("Wrong TypeTag variant: expected {expected}, got {got}")]
    Variant { expected: String, got: String },
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

// =============================================================================
//  Errors used in derived impls
// =============================================================================

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

// =============================================================================
//  Internals
// =============================================================================

fn type_tag_variant_name(this: &TypeTag) -> String {
    match this {
        TypeTag::U8 => "U8",
        TypeTag::U16 => "U16",
        TypeTag::U32 => "U32",
        TypeTag::U64 => "U64",
        TypeTag::U128 => "U128",
        TypeTag::U256 => "U256",
        TypeTag::Bool => "Bool",
        TypeTag::Address => "Address",
        TypeTag::Signer => "Signer",
        TypeTag::Vector(_) => "Vector",
        TypeTag::Struct(_) => "Struct",
    }
    .to_owned()
}
