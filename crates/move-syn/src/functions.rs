use unsynn::*;

use crate::{Generics, MaybeRefType, kw};

cfg_if::cfg_if!(if #[cfg(feature = "fun-args")] {
    unsynn! {
        pub struct Function {
            entry: Option<kw::Entry>,
            fun_kw: kw::Fun,
            ident: Ident,
            generics: Option<Generics>,
            args: FunctionArgs,
            ret: Option<FunctionReturn>,
            body: BraceGroup,
        }

        struct FunctionArgs(ParenthesisGroupContaining<CommaDelimitedVec<FunctionArg>>);

        /// E.g., `name: T`, `mut name: T`, `name: &T`, `name: &mut T`.
        struct FunctionArg {
            mut_: Option<kw::Mut>,
            ident: Ident,
            colon: Colon,
            type_: MaybeRefType,
        }

        enum ReturnType {
            One(MaybeRefType),
            Many(ParenthesisGroupContaining<CommaDelimitedVec<MaybeRefType>>)
        }
   }
} else {
    unsynn! {
        pub struct Function {
            entry: Option<kw::Entry>,
            fun_kw: kw::Fun,
            ident: Ident,
            generics: Option<Generics>,
            args: ParenthesisGroup,
            ret: Option<FunctionReturn>,
            body: BraceGroup,
        }

        enum ReturnType {
            One(MaybeRefType),
            Many(ParenthesisGroup)
        }
    }
});

unsynn! {
    /// `: T`, `: &T`, `: &mut T`, `: (...)`
    struct FunctionReturn {
        colon: Colon,
        type_: ReturnType,
    }
}

impl Function {
    pub const fn ident(&self) -> &Ident {
        &self.ident
    }

    pub const fn is_entry(&self) -> bool {
        self.entry.is_some()
    }
}

#[cfg(feature = "fun-args")]
impl Function {
    pub fn n_returns(&self) -> usize {
        self.ret
            .as_ref()
            .map(|ret| match &ret.type_ {
                ReturnType::One(_) => 1,
                ReturnType::Many(group) => group.content.len(),
            })
            .unwrap_or_default()
    }
}
