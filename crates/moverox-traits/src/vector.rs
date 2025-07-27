use moverox_types::TypeTag;

use crate::{MoveType, MoveTypeTag, TypeTagError};

impl<T: MoveType> MoveType for Vec<T> {
    type TypeTag = VecTypeTag<T::TypeTag>;
}

#[derive(Clone, PartialOrd, Ord, PartialEq, Eq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct VecTypeTag<T: MoveTypeTag>(pub T);

impl<T: MoveTypeTag> MoveTypeTag for VecTypeTag<T> {
    fn from_type_tag(value: &TypeTag) -> Result<Self, TypeTagError> {
        match value {
            TypeTag::Vector(type_) => Ok(Self(MoveTypeTag::from_type_tag(type_)?)),
            _ => Err(TypeTagError::Variant {
                expected: "Vector(_)".to_owned(),
                got: crate::type_tag_variant_name(value),
            }),
        }
    }

    fn to_type_tag(&self) -> TypeTag {
        TypeTag::Vector(Box::new(self.0.to_type_tag()))
    }
}
