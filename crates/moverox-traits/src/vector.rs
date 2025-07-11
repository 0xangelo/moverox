use moverox_types::TypeTag;

use crate::{MoveType, MoveTypeTag, TypeTagError};

impl<T: MoveType> MoveType for Vec<T> {
    type TypeTag = VecTypeTag<T::TypeTag>;
}

#[derive(Clone, PartialOrd, Ord, PartialEq, Eq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct VecTypeTag<T: MoveTypeTag>(T);

impl<T: MoveTypeTag> From<VecTypeTag<T>> for TypeTag {
    fn from(value: VecTypeTag<T>) -> Self {
        Self::Vector(Box::new(value.0.into()))
    }
}

impl<T: MoveTypeTag> TryFrom<TypeTag> for VecTypeTag<T> {
    type Error = TypeTagError;

    fn try_from(value: TypeTag) -> Result<Self, Self::Error> {
        match value {
            TypeTag::Vector(type_) => Ok(Self((*type_).try_into()?)),
            _ => Err(TypeTagError::Variant {
                expected: "Vector(_)".to_owned(),
                got: value,
            }),
        }
    }
}
