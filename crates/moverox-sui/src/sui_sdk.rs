/// Conversions between [`sui_sdk_types`] and [`moverox::types`].
#[sealed::sealed]
pub trait Compat {
    type To;

    fn from_sui(value: Self::To) -> Self;

    fn into_sui(self) -> Self::To;
}

#[sealed::sealed]
impl Compat for moverox::types::Address {
    type To = sui_sdk_types::Address;

    fn from_sui(value: Self::To) -> Self {
        Self::new(value.into_inner())
    }

    fn into_sui(self) -> Self::To {
        sui_sdk_types::Address::new(self.into_inner())
    }
}

#[sealed::sealed]
impl Compat for moverox::types::Identifier {
    type To = sui_sdk_types::Identifier;

    fn from_sui(value: Self::To) -> Self {
        Self::new(value.as_str()).expect("Compatible identifiers")
    }

    fn into_sui(self) -> Self::To {
        sui_sdk_types::Identifier::new(self.into_inner()).expect("Compatible identifiers")
    }
}

#[sealed::sealed]
impl Compat for moverox::types::StructTag {
    type To = sui_sdk_types::StructTag;

    fn from_sui(value: Self::To) -> Self {
        Self {
            address: Compat::from_sui(*value.address()),
            module: Compat::from_sui(value.module().clone()),
            name: Compat::from_sui(value.name().clone()),
            type_params: value
                .type_params()
                .iter()
                .cloned()
                .map(Compat::from_sui)
                .collect(),
        }
    }

    fn into_sui(self) -> Self::To {
        let Self {
            address,
            module,
            name,
            type_params,
        } = self;
        sui_sdk_types::StructTag::new(
            address.into_sui(),
            module.into_sui(),
            name.into_sui(),
            type_params.into_iter().map(Compat::into_sui).collect(),
        )
    }
}

#[sealed::sealed]
impl Compat for moverox::types::TypeTag {
    type To = sui_sdk_types::TypeTag;
    fn from_sui(value: Self::To) -> Self {
        use sui_sdk_types::TypeTag as T;
        match value {
            T::U8 => Self::U8,
            T::U16 => Self::U16,
            T::U32 => Self::U32,
            T::U64 => Self::U64,
            T::U128 => Self::U128,
            T::U256 => Self::U256,
            T::Bool => Self::Bool,
            T::Address => Self::Address,
            T::Signer => Self::Signer,
            T::Vector(t) => Self::Vector(Box::new(Compat::from_sui(*t))),
            T::Struct(s) => Self::Struct(Box::new(Compat::from_sui(*s))),
        }
    }

    fn into_sui(self) -> Self::To {
        use sui_sdk_types::TypeTag as T;
        match self {
            Self::U8 => T::U8,
            Self::U16 => T::U16,
            Self::U32 => T::U32,
            Self::U64 => T::U64,
            Self::U128 => T::U128,
            Self::U256 => T::U256,
            Self::Bool => T::Bool,
            Self::Address => T::Address,
            Self::Signer => T::Signer,
            Self::Vector(t) => T::Vector(Box::new(t.into_sui())),
            Self::Struct(s) => T::Struct(Box::new(s.into_sui())),
        }
    }
}
