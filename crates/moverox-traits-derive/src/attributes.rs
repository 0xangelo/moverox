#![expect(
    clippy::option_if_let_else,
    reason = "Generated code by `#[darling(default)]`"
)]

use darling::FromDeriveInput;
use syn::{Ident, Path, parse_quote};

#[derive(FromDeriveInput)]
#[darling(attributes(move_))]
pub(crate) struct MoveAttributes {
    #[darling(rename = "crate")]
    pub(crate) thecrate: Option<Path>,
    pub(crate) address: Option<String>,
    pub(crate) module: Option<Ident>,
    #[darling(default)]
    pub(crate) nameless: bool,
}

impl MoveAttributes {
    pub(crate) fn thecrate(&self) -> Path {
        self.thecrate
            .as_ref()
            .cloned()
            .unwrap_or_else(|| parse_quote!(::moverox_traits))
    }
}
