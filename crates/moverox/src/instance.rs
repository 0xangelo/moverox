use std::error::Error as StdError;

use moverox_traits::{
    MoveDatatype,
    MoveDatatypeTag as _,
    MoveType,
    MoveTypeTag as _,
    StructTagError,
    TypeTagError,
};
use moverox_types::{StructTag, TypeTag};
use serde::Deserialize;

/// Parse an instance of a [`MoveType`] from a generic type tag and BCS bytes.
///
/// This function short-circuits if the type tag can't be converted to the [`MoveType::TypeTag`],
/// avoiding deserialization of the BCS bytes in such cases.
pub fn parse_move_instance<T: MoveType + for<'de> Deserialize<'de>>(
    type_tag: &TypeTag,
    bytes: &[u8],
) -> Result<(T::TypeTag, T), FromRawInstanceError> {
    Ok((
        T::TypeTag::from_type_tag(type_tag)?,
        bcs::from_bytes(bytes).map_err(|e| FromRawInstanceError::Bcs(e.into()))?,
    ))
}

/// Parse an instance of a [`MoveDatatype`] from a generic struct tag and BCS bytes.
///
/// This function short-circuits if the type tag can't be converted to the
/// [`MoveDatatype::StructTag`], avoiding deserialization of the BCS bytes in such cases.
pub fn parse_move_datatype<T: MoveDatatype + for<'de> Deserialize<'de>>(
    struct_tag: &StructTag,
    bytes: &[u8],
) -> Result<(T::StructTag, T), FromRawDatatypeError> {
    Ok((
        T::StructTag::from_struct_tag(struct_tag)?,
        bcs::from_bytes(bytes).map_err(|e| FromRawDatatypeError::Bcs(e.into()))?,
    ))
}

/// Error for [`parse_move_instance`].
#[derive(thiserror::Error, Debug)]
pub enum FromRawInstanceError {
    #[error("Converting from TypeTag: {0}")]
    TypeTag(#[from] TypeTagError),
    #[error("Deserializing BCS: {0}")]
    Bcs(Box<dyn StdError + Send + Sync + 'static>),
}

/// Error for [`parse_move_datatype`].
#[derive(thiserror::Error, Debug)]
pub enum FromRawDatatypeError {
    #[error("Converting from StructTag: {0}")]
    StructTag(#[from] StructTagError),
    #[error("Deserializing BCS: {0}")]
    Bcs(Box<dyn StdError + Send + Sync + 'static>),
}
