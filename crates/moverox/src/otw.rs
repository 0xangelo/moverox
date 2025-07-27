use moverox_traits::MoveDatatype;
use serde::{Deserialize, Serialize};

/// Generic type signaling a one-time-witness type argument.
///
/// None of address, module and name are known at compile time, only that there are no type
/// parameters.
///
/// Use this when you want to instantiate a datatype like `Balance<phantom T>` that expects its
/// type argument to be a Move one-time-witness.
///
/// # Examples
/// ```
/// use moverox::Otw;
/// use moverox::traits::{MoveDatatype, MoveType};
/// use serde::{Deserialize, Serialize};
///
/// #[derive(MoveDatatype, Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq, Hash)]
/// #[move_(address = "0x2", module = balance)]
/// pub struct Balance<T> {
///     value: u64,
///     _otw: std::marker::PhantomData<T>,
/// }
///
/// let address = "0x2".parse().unwrap();
/// let module = "sui".parse().unwrap();
/// let name = "SUI".parse().unwrap();
/// let sui_type = Otw::type_tag(address, module, name);
/// let balance_type = Balance::<Otw>::type_tag(sui_type);
/// ```
#[derive(MoveDatatype, Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[move_(nameless)]
pub struct Otw {
    dummy_field: bool,
}

impl Otw {
    pub const fn new() -> Self {
        Self { dummy_field: false }
    }
}
