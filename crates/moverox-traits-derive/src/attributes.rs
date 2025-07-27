use syn::{Ident, Path, parse_quote};

#[derive(deluxe::ExtractAttributes)]
#[deluxe(attributes(move_))]
pub(crate) struct MoveAttributes {
    #[deluxe(rename = crate)]
    pub(crate) thecrate: Option<Path>,
    pub(crate) address: Option<String>,
    pub(crate) module: Option<Ident>,
    #[deluxe(default = false)]
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
