use moverox_types::TypeTag;

use crate::{ConstTypeTag, MoveType, MoveTypeTag, ParseTypeTagError, TypeTagError};

impl<T: MoveType> MoveType for Vec<T> {
    type TypeTag = VecTypeTag<T::TypeTag>;
}

#[derive(Clone, PartialOrd, Ord, PartialEq, Eq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct VecTypeTag<T: MoveTypeTag>(pub T);

impl<T: MoveTypeTag> MoveTypeTag for VecTypeTag<T> {
    fn as_datatype_tag(&self) -> Option<&dyn crate::MoveDatatypeTag> {
        None
    }

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

impl<T: ConstTypeTag> ConstTypeTag for Vec<T> {
    const TYPE_TAG: VecTypeTag<T::TypeTag> = VecTypeTag(T::TYPE_TAG);
}

impl<T: MoveTypeTag> std::fmt::Display for VecTypeTag<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let stag = MoveTypeTag::to_type_tag(self);
        write!(f, "{}", stag)
    }
}

impl<T: MoveTypeTag> std::str::FromStr for VecTypeTag<T> {
    type Err = ParseTypeTagError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tag = s.parse().map_err(ParseTypeTagError::from_str)?;
        Ok(Self::from_type_tag(&tag)?)
    }
}
