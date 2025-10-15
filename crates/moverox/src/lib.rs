#![cfg_attr(all(doc, not(doctest)), feature(doc_cfg))]

//! Move, oxidized.

pub use {moverox_traits as traits, moverox_types as types, serde};

#[cfg(feature = "bcs")]
mod instance;
mod macros;
mod otw;

#[cfg(feature = "bcs")]
pub use instance::{
    FromRawDatatypeError,
    FromRawInstanceError,
    parse_move_datatype,
    parse_move_instance,
};
pub use otw::{Otw, OtwTypeTag};
