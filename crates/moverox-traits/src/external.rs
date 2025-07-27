//! Re-exports for derive
pub use moverox_types::{Address, IdentStr, Identifier, StructTag, TypeTag, U256, const_address};
#[allow(exported_private_dependencies, clippy::useless_attribute)]
pub use serde;

pub const fn const_ident(s: &'static str) -> &'static IdentStr {
    IdentStr::cast(s)
}
