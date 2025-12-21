use std::collections::HashMap;

use unsynn::*;

use crate::{Generics, HasGenerics, ItemKind, MaybeRefType, Typed, kw, mutate_delimited_vec};

unsynn! {
    pub(super) struct Arguments(ParenthesisGroupContaining<CommaDelimitedVec<FunctionArg>>);

    /// E.g., `name: T`, `mut name: T`, `name: &T`, `name: &mut T`.
    pub struct FunctionArg {
        mut_: Option<kw::Mut>,
        ident: Ident,
        colon: Colon,
        type_: MaybeRefType,
    }

    /// `: T`, `: &T`, `: &mut T`, `: (...)`
    pub(super) struct Returns {
        colon: Colon,
        type_: ReturnType,
    }

    pub(super) enum ReturnType {
        One(MaybeRefType),
        Many(ParenthesisGroupContaining<CommaDelimitedVec<MaybeRefType>>),
    }
}

impl crate::Module {
    /// Resolve all function signature types to their fully-qualified paths.
    pub fn fully_qualify_fun_signature_types(&mut self) -> &mut Self {
        // Collect all imported types and their paths
        let imports: HashMap<_, _> = self
            .items()
            .filter_map(|item| match &item.kind {
                ItemKind::Import(import) => Some(import),
                _ => None,
            })
            .flat_map(|import| import.flatten())
            .collect();

        // Resolve datatype fields' types
        for item in &mut self.contents.content {
            match &mut item.kind {
                ItemKind::Function(fun) => {
                    let generics = &fun.type_param_idents();
                    fun.map_types(|ty| ty.resolve(&imports, generics));
                }
                ItemKind::NativeFun(native) => {
                    let generics = &native.type_param_idents();
                    native.map_types(|ty| ty.resolve(&imports, generics));
                }
                _ => (),
            }
        }

        self
    }
}

impl HasGenerics for super::Function {
    fn generics(&self) -> Option<&Generics> {
        self.generics.as_ref()
    }
}

impl HasGenerics for super::NativeFun {
    fn generics(&self) -> Option<&Generics> {
        self.generics.as_ref()
    }
}

impl Typed for super::Function {
    fn map_types(&mut self, mut f: impl FnMut(&mut crate::Type)) {
        mutate_delimited_vec(&mut self.args.0.content, |arg| f(&mut arg.type_.r#type));
        if let Some(ret) = self.ret.as_mut() {
            ret.map_types(f);
        }
    }
}

impl Typed for super::NativeFun {
    fn map_types(&mut self, mut f: impl FnMut(&mut crate::Type)) {
        mutate_delimited_vec(&mut self.args.0.content, |arg| f(&mut arg.type_.r#type));
        if let Some(ret) = self.ret.as_mut() {
            ret.map_types(f);
        }
    }
}

impl Typed for Returns {
    fn map_types(&mut self, mut f: impl FnMut(&mut crate::Type)) {
        match &mut self.type_ {
            ReturnType::One(maybe_ref_type) => f(&mut maybe_ref_type.r#type),
            ReturnType::Many(parenthesis_group) => {
                mutate_delimited_vec(&mut parenthesis_group.content, |maybe_ref_type| {
                    f(&mut maybe_ref_type.r#type)
                })
            }
        }
    }
}

impl super::Function {
    /// The input arguments to this function.
    pub fn arguments(&self) -> impl ExactSizeIterator<Item = &FunctionArg> {
        self.args.0.content.iter().map(|d| &d.value)
    }

    /// The output types from this function.
    pub fn returns(&self) -> impl ExactSizeIterator<Item = &MaybeRefType> {
        MaybeRefTypeIter::new(self.ret.as_ref())
    }
}

impl super::NativeFun {
    /// The input arguments to this function.
    pub fn arguments(&self) -> impl ExactSizeIterator<Item = &FunctionArg> {
        self.args.0.content.iter().map(|d| &d.value)
    }

    /// The output types from this function.
    pub fn returns(&self) -> impl ExactSizeIterator<Item = &MaybeRefType> {
        MaybeRefTypeIter::new(self.ret.as_ref())
    }
}

impl FunctionArg {
    pub const fn ident(&self) -> &Ident {
        &self.ident
    }

    pub const fn type_(&self) -> &MaybeRefType {
        &self.type_
    }
}

struct MaybeRefTypeIter<'a> {
    inner: Option<&'a Returns>,
    idx: usize,
}

impl<'a> MaybeRefTypeIter<'a> {
    const fn new(inner: Option<&'a Returns>) -> Self {
        Self { inner, idx: 0 }
    }
}

impl<'a> Iterator for MaybeRefTypeIter<'a> {
    type Item = &'a MaybeRefType;

    fn next(&mut self) -> Option<Self::Item> {
        match &self.inner {
            None => None,
            Some(Returns {
                type_: ReturnType::One(maybe_ref_type),
                ..
            }) if self.idx == 0 => {
                self.idx += 1;
                Some(maybe_ref_type)
            }
            Some(Returns {
                type_: ReturnType::One(_),
                ..
            }) => None,
            Some(Returns {
                type_: ReturnType::Many(parenthesis_group),
                ..
            }) if self.idx < parenthesis_group.content.len() => {
                let item = &parenthesis_group.content[self.idx].value;
                self.idx += 1;
                Some(item)
            }
            Some(Returns {
                type_: ReturnType::Many(_),
                ..
            }) => None,
        }
    }
}

impl<'a> ExactSizeIterator for MaybeRefTypeIter<'a> {
    fn len(&self) -> usize {
        match &self.inner {
            None => 0,
            Some(Returns {
                type_: ReturnType::One(_),
                ..
            }) => 1,
            Some(Returns {
                type_: ReturnType::Many(parenthesis_group),
                ..
            }) => parenthesis_group.content.len(),
        }
    }
}
