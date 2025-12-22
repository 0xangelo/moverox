use cfg_if::cfg_if;
use unsynn::*;

use crate::{Generics, kw};

#[cfg(feature = "fun-sig")]
mod signature;

#[cfg(feature = "fun-sig")]
pub use self::signature::FunctionArg;

unsynn! {
    pub struct Function {
        entry: Option<kw::Entry>,
        fun_kw: kw::Fun,
        ident: Ident,
        generics: Option<Generics>,
        args: Arguments,
        ret: Option<Returns>,
        body: BraceGroup,
    }

    pub struct NativeFun {
        native_kw: kw::Native,
        fun_kw: kw::Fun,
        ident: Ident,
        generics: Option<Generics>,
        args: Arguments,
        ret: Option<Returns>,
        semicolon: Semicolon
    }
}

cfg_if!(if #[cfg(feature = "fun-sig")] {
    use self::signature::Arguments;
    use self::signature::Returns;
} else {
    type Arguments = ParenthesisGroup;

    unsynn! {
        /// `: T`, `: &T`, `: &mut T`, `: (...)`
        struct Returns {
            colon: Colon,
            type_: ReturnType,
        }

        enum ReturnType {
            One(crate::MaybeRefType),
            Many(ParenthesisGroup)
        }
    }
});

impl Function {
    pub const fn is_entry(&self) -> bool {
        self.entry.is_some()
    }

    pub const fn ident(&self) -> &Ident {
        &self.ident
    }

    pub const fn generics(&self) -> Option<&Generics> {
        self.generics.as_ref()
    }
}

impl NativeFun {
    pub const fn ident(&self) -> &Ident {
        &self.ident
    }

    pub const fn generics(&self) -> Option<&Generics> {
        self.generics.as_ref()
    }
}
