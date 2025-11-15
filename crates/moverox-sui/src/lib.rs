#![cfg_attr(nightly, feature(doc_cfg))]

//! Oxidized Sui Move framework packages and compatibility layer for `sui_sdk_types`.

#[cfg(feature = "sui-sdk")]
mod sui_sdk;

#[cfg(feature = "sui-sdk")]
pub use sui_sdk::Compat;

/// Oxidized `MoveStdlib` @ `0x1`.
pub mod move_stdlib {
    moverox::include_oxidized!("std");
}

/// Oxidized `Sui` @ `0x2`.
#[expect(clippy::module_inception, reason = "`sui` module inside")]
pub mod sui {
    moverox::include_oxidized!("sui");
}
