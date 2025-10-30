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

    pub fn signature_string(&self, named_args: bool) -> String {
        let Self {
            entry,
            fun_kw,
            ident,
            generics,
            args: FunctionArgs(ParenthesisGroupContaining { content: args }),
            ret,
            body: _,
        } = self;

        fn to_string(t: &impl ToTokens) -> String {
            t.tokens_to_string()
        }

        let fun = to_string(fun_kw);
        let maybe_entry = entry
            .as_ref()
            .map(|e| to_string(e) + " ")
            .unwrap_or_default();
        let generics = generics.as_ref().map(to_string).unwrap_or_default();

        let args = args
            .iter()
            .map(|d| {
                let type_ = to_string(&d.value.type_);
                if named_args {
                    format!("{}: {type_}", d.value.ident)
                } else {
                    type_
                }
            })
            .reduce(|a, b| a + ", " + &b)
            .unwrap_or_default();

        let ret = ret.as_ref().map(to_string).unwrap_or_default();
        format!("{maybe_entry}{fun} {ident}{generics}({args}){ret}")
            .replace(" ,", ",")
            .replace(",)", ")")
            .replace(" <", "<")
            .replace("< ", "<")
            .replace(" >", ">")
            .replace(" :", ":")
            .replace(":: ", "::")
            .replace("& ", "&")
    }
}
