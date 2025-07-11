#![cfg_attr(all(doc, not(doctest)), feature(doc_auto_cfg))]

//! Oxidized Sui Move framework packages.

/// Oxidized `MoveStdlib` @ `0x1`.
pub mod move_stdlib {
    moverox::include_oxidized!("std");
}

/// Oxidized `Sui` @ `0x2`.
#[expect(clippy::module_inception, reason = "`sui` module inside")]
pub mod sui {
    moverox::include_oxidized!("sui");
}
