use std::str::FromStr;

use moverox_types::{Address, TypeTag, U256};

use crate::{MoveType, MoveTypeTag, ParseTypeTagError, TypeTagError};

macro_rules! impl_primitive_type_tags {
    ($($typ:ty: ($type_:ident, $variant:ident)),*) => {
        $(
            #[derive(
                Clone,
                Debug,
                PartialEq,
                Eq,
                Hash,
                PartialOrd,
                Ord,
            )]
            #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
            pub struct $type_;

            impl MoveTypeTag for $type_ {
                fn from_type_tag(value: &TypeTag) -> Result<Self, TypeTagError> {
                    match value {
                        TypeTag::$variant => Ok(Self),
                        _ => Err(TypeTagError::Variant {
                            expected: stringify!($variant).to_owned(),
                            got: crate::type_tag_variant_name(value) }
                        )
                    }
                }

                fn to_type_tag(&self) -> TypeTag {
                    TypeTag::$variant
                }
            }

            impl FromStr for $type_ {
                type Err = ParseTypeTagError;

                fn from_str(s: &str) -> Result<Self, Self::Err> {
                    let tag: TypeTag = s.parse().map_err(ParseTypeTagError::from_str)?;
                    Ok(MoveTypeTag::from_type_tag(&tag)?)
                }
            }

            impl MoveType for $typ {
                type TypeTag = $type_;
            }
        )*
    };
}

impl_primitive_type_tags! {
    Address: (AddressTypeTag, Address),
    bool: (BoolTypeTag, Bool),
    u8: (U8TypeTag, U8),
    u16: (U16TypeTag, U16),
    u32: (U32TypeTag, U32),
    u64: (U64TypeTag, U64),
    u128: (U128TypeTag, U128),
    U256: (U256TypeTag, U256)
}
