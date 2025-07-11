//! Re-exports for derive
pub use moverox_types::{Address, IdentStr, Identifier, StructTag, TypeTag, U256, const_address};
pub use serde::{Deserialize, Serialize};
pub use serde_with::{DeserializeFromStr, SerializeDisplay};
pub use {derive_where, serde_with};

pub const fn const_ident(s: &'static str) -> &'static IdentStr {
    IdentStr::cast(s)
}
