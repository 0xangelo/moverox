use std::collections::HashSet;

use move_syn::Attributes;
use quote::quote;
use unsynn::{CommaDelimitedVec, IParse as _, Ident, ToTokens as _, TokenStream};

use crate::Result;

#[expect(clippy::result_large_err, reason = "Error from the unsynn crate")]
mod grammar {
    use unsynn::*;

    mod kw {
        use unsynn::unsynn;

        unsynn! {
            pub(super) keyword Moverox = "moverox";
            pub(super) keyword Otw = "OTW";
            // NOTE: we cannot use `type` here since the Move parser will complain because it's a
            // reserved keyword
            pub(super) keyword Type = "type_";
        }
    }

    unsynn! {
        /// Allows parsing `moverox(...)` attributes inside `#[ext(...)]`
        ///
        /// # Example
        ///
        /// ```move
        /// #[ext(moverox(type(T = OTW)))]
        /// public struct BalanceUpdated<phantom T> {}
        /// ```
        pub(crate) struct Annotation {
            kw: kw::Moverox,
            contents: ParenthesisGroupContaining<CommaDelimitedVec<Setting>>
        }

        /// The different accepted attributes inside `moverox(...)`
        pub(super) enum Setting {
            /// Currrently only type defaults
            Type(Type)
        }

        /// Custom attribute to set defaults for type parameters of a datatype.
        pub(super) struct Type {
            kw: kw::Type,
            contents: ParenthesisGroupContaining<CommaDelimitedVec<TypeDefault>>,
        }

        /// An instance of a type parameter default.
        struct TypeDefault {
            /// Identifier of the annotated datatype's type parameter
            ident: Ident,
            assign: Assign,
            /// For now, only defaults like `{T} = OTW` are supported
            default: kw::Otw,
        }

        /// A single comma-separated entry inside an `#[ext(...)]` group.
        ///
        /// `#[ext(...)]` is a shared namespace: other tooling attaches its own sub-attributes
        /// (e.g. `versioned(...)`, `dynamic_field(...)`) alongside `moverox(...)`. We parse the
        /// whole group and keep only the `moverox(...)` entries, ignoring the rest.
        pub(crate) enum ExtEntry {
            Moverox(Annotation),
            Other(OtherEntry),
        }

        /// Any non-`moverox` `#[ext(...)]` entry. Consumes everything up to the next comma,
        /// so it accepts every shape other tooling might use — `versioned(EFoo)`,
        /// `dynamic_field(name = ID, value = ID)`, `foo = b"bar"`, a bare `dev_inspect`, …
        pub(crate) struct OtherEntry {
            tokens: Vec<Cons<Except<Comma>, TokenTree>>,
        }
    }

    impl Annotation {
        pub(super) fn settings(&self) -> impl Iterator<Item = &Setting> + '_ {
            self.contents
                .content
                .iter()
                .map(|delimited| &delimited.value)
        }
    }

    impl Setting {
        pub(super) fn otw_types(&self) -> impl Iterator<Item = &Ident> + '_ {
            let Self::Type(ty) = self;
            ty.contents
                .content
                .iter()
                .map(|delimited| &delimited.value.ident)
        }
    }
}

/// Filter and parse Move attributes into Rust docs (1st) and OTW type defaults (2nd).
pub(super) fn extract(attrs: &[Attributes]) -> Result<(TokenStream, HashSet<Ident>)> {
    let (move_docs, other): (Vec<_>, Vec<_>) = attrs.iter().partition(|attr| attr.is_doc());

    let rust_docs = move_docs.into_iter().map(process_doc).collect();

    let custom: Vec<_> = other.into_iter().flat_map(as_moverox).collect();
    let mut otw_types = HashSet::new();
    for ident in custom
        .iter()
        .flat_map(|custom| custom.settings())
        .flat_map(|setting| setting.otw_types())
    {
        if otw_types.contains(ident) {
            return Err(format!("Type {ident} declared twice").into());
        }
        otw_types.insert(ident.to_owned());
    }

    Ok((rust_docs, otw_types))
}

pub(super) fn as_moverox(attr: &Attributes) -> impl Iterator<Item = self::grammar::Annotation> {
    // An `#[ext(...)]` group may carry sibling sub-attributes from other tooling
    // (e.g. `#[ext(moverox(type_(T = OTW)), versioned(EFoo))]`). Parse the whole
    // comma-delimited group and keep only the `moverox(...)` entries, so a sibling
    // attribute never causes the `moverox(...)` annotation to be dropped.
    attr.external_attributes()
        .filter_map(|ext| {
            ext.to_token_iter()
                .parse_all::<CommaDelimitedVec<self::grammar::ExtEntry>>()
                .ok()
        })
        .flat_map(|entries| {
            entries.into_iter().filter_map(|entry| match entry.value {
                self::grammar::ExtEntry::Moverox(annotation) => Some(annotation),
                self::grammar::ExtEntry::Other(_) => None,
            })
        })
}

fn process_doc(attr: &Attributes) -> TokenStream {
    let inner = attr.contents().to_token_stream();
    // NOTE: disable when compiling doctests to avoid Rust interpreting code blocks as
    // runnable tests.
    quote!(#[cfg_attr(not(doctest), #inner)])
}
