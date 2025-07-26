use std::fmt::Display;

use indoc::indoc;
use move_syn::ItemKind;
use unsynn::IParse as _;

use crate::*;

fn from_module(s: &str) -> impl Display {
    let ast: Module = s.to_token_iter().parse_all().unwrap();
    prettyplease::unparse(
        &syn::parse_file(
            &ast.to_rust(&quote!(::moverox), None, &Default::default())
                .to_string(),
        )
        .unwrap(),
    )
}

fn from_struct(s: &str) -> impl Display {
    let ast: Item = s.to_token_iter().parse_all().unwrap();
    assert!(matches!(
        ast,
        Item {
            kind: ItemKind::Struct(_),
            ..
        }
    ));
    prettyplease::unparse(
        &syn::parse_file(
            &ast.to_rust(&quote!(::moverox), None, None, &Default::default())
                .to_string(),
        )
        .unwrap(),
    )
}

fn from_enum(s: &str) -> impl Display {
    let ast: Item = s.to_token_iter().parse_all().unwrap();
    assert!(matches!(
        ast,
        Item {
            kind: ItemKind::Enum(_),
            ..
        }
    ));
    let content = ast
        .to_rust(&quote!(::moverox), None, None, &Default::default())
        .to_string();
    prettyplease::unparse(&syn::parse_file(&content).unwrap())
}

//=Test cases=======================================================================================

#[test]
fn empty_enum() {
    let move_enum = indoc! {"
        public enum Single {
            Only,
        }
    "};
    insta::assert_snapshot!(from_enum(move_enum), @r#"
    #[derive(
        Clone,
        Debug,
        PartialEq,
        Eq,
        Hash,
        ::moverox::traits::MoveDatatype,
        ::moverox::serde::Deserialize,
        ::moverox::serde::Serialize,
    )]
    #[move_(crate = ::moverox::traits)]
    #[serde(crate = "::moverox::serde")]
    #[allow(non_snake_case)]
    pub enum Single {
        Only,
    }
    "#
    );
}

#[test]
fn empty_enum_with_phantoms() {
    let move_enum = indoc! {"
        public enum Single<phantom T> {
            Only,
        }
    "};
    insta::assert_snapshot!(from_enum(move_enum), @r#"
    #[derive(
        Clone,
        Debug,
        PartialEq,
        Eq,
        Hash,
        ::moverox::traits::MoveDatatype,
        ::moverox::serde::Deserialize,
        ::moverox::serde::Serialize,
    )]
    #[move_(crate = ::moverox::traits)]
    #[serde(crate = "::moverox::serde")]
    #[allow(non_snake_case)]
    pub enum Single<T> {
        Only(#[serde(skip)] ::std::marker::PhantomData<T>),
    }
    "#);
}

#[test]
fn enum_with_variants() {
    let move_enum = indoc! {"
        /// `Segment` enum definition.
        /// Defines various string segments.
        public enum Segment has copy, drop {
            /// Empty variant, no value.
            Empty,
            /// Variant with a value (positional style).
            String(String),
            /// Variant with named fields.
            Special {
                content: vector<u8>,
                encoding: u8, // Encoding tag.
            },
        }
    "};
    insta::assert_snapshot!(from_enum(move_enum), @r#"
    /// `Segment` enum definition.
    /// Defines various string segments.
    #[derive(
        Clone,
        Debug,
        PartialEq,
        Eq,
        Hash,
        ::moverox::traits::MoveDatatype,
        ::moverox::serde::Deserialize,
        ::moverox::serde::Serialize,
    )]
    #[move_(crate = ::moverox::traits)]
    #[serde(crate = "::moverox::serde")]
    #[allow(non_snake_case)]
    pub enum Segment {
        /// Empty variant, no value.
        Empty,
        /// Variant with a value (positional style).
        String(String),
        /// Variant with named fields.
        Special { content: vector<u8>, encoding: u8 },
    }
    "#);
}

#[test]
fn enum_with_generics_and_variants() {
    let move_enum = indoc! {"
        public enum Generic<phantom T> has copy, drop, store {
            Unit,
            Tuple(u64),
            Struct {
                value: u64,
            }
        }
    "};
    insta::assert_snapshot!(from_enum(move_enum), @r#"
    #[derive(
        Clone,
        Debug,
        PartialEq,
        Eq,
        Hash,
        ::moverox::traits::MoveDatatype,
        ::moverox::serde::Deserialize,
        ::moverox::serde::Serialize,
    )]
    #[move_(crate = ::moverox::traits)]
    #[serde(crate = "::moverox::serde")]
    #[allow(non_snake_case)]
    pub enum Generic<T> {
        Unit(#[serde(skip)] ::std::marker::PhantomData<T>),
        Tuple(u64),
        Struct { value: u64 },
    }
    "#);
}

#[test]
fn struct_with_keyword_in_field_name() {
    insta::assert_snapshot!(from_struct("public struct Borrow { ref: address, obj: ID }"), @r#"
    #[derive(
        Clone,
        Debug,
        PartialEq,
        Eq,
        Hash,
        ::moverox::traits::MoveDatatype,
        ::moverox::serde::Deserialize,
        ::moverox::serde::Serialize,
    )]
    #[move_(crate = ::moverox::traits)]
    #[serde(crate = "::moverox::serde")]
    #[allow(non_snake_case)]
    pub struct Borrow {
        pub r#ref: address,
        pub obj: ID,
    }
    impl Borrow {
        #[allow(clippy::just_underscores_and_digits, clippy::too_many_arguments)]
        pub const fn new(r#ref: address, obj: ID) -> Self {
            Self { r#ref, obj }
        }
    }
    "#);
}

#[test]
fn empty_struct() {
    insta::assert_snapshot!(from_struct("struct OTW {}"), @r#"
    #[derive(
        Default,
        Clone,
        Debug,
        PartialEq,
        Eq,
        Hash,
        ::moverox::traits::MoveDatatype,
        ::moverox::serde::Deserialize,
        ::moverox::serde::Serialize,
    )]
    #[move_(crate = ::moverox::traits)]
    #[serde(crate = "::moverox::serde")]
    #[allow(non_snake_case)]
    pub struct OTW {
        /// BCS for empty structs actually encodes a single boolean hidden field
        dummy_field: bool,
    }
    impl OTW {
        #[allow(clippy::just_underscores_and_digits, clippy::too_many_arguments)]
        pub const fn new() -> Self {
            Self { dummy_field: false }
        }
    }
    "#);
}

#[test]
fn public_empty_struct() {
    insta::assert_snapshot!(from_struct("public struct OTW {}"), @r#"
    #[derive(
        Default,
        Clone,
        Debug,
        PartialEq,
        Eq,
        Hash,
        ::moverox::traits::MoveDatatype,
        ::moverox::serde::Deserialize,
        ::moverox::serde::Serialize,
    )]
    #[move_(crate = ::moverox::traits)]
    #[serde(crate = "::moverox::serde")]
    #[allow(non_snake_case)]
    pub struct OTW {
        /// BCS for empty structs actually encodes a single boolean hidden field
        dummy_field: bool,
    }
    impl OTW {
        #[allow(clippy::just_underscores_and_digits, clippy::too_many_arguments)]
        pub const fn new() -> Self {
            Self { dummy_field: false }
        }
    }
    "#);
}

#[test]
fn public_empty_struct_with_phantom() {
    insta::assert_snapshot!(from_struct("public struct OTW<phantom T> {}"), @r#"
    #[derive(
        Default,
        Clone,
        Debug,
        PartialEq,
        Eq,
        Hash,
        ::moverox::traits::MoveDatatype,
        ::moverox::serde::Deserialize,
        ::moverox::serde::Serialize,
    )]
    #[move_(crate = ::moverox::traits)]
    #[serde(crate = "::moverox::serde")]
    #[allow(non_snake_case)]
    pub struct OTW<T> {
        /// BCS for empty structs actually encodes a single boolean hidden field
        dummy_field: bool,
        #[serde(skip)]
        _T: ::std::marker::PhantomData<T>,
    }
    impl<T> OTW<T> {
        #[allow(clippy::just_underscores_and_digits, clippy::too_many_arguments)]
        pub const fn new() -> Self {
            Self {
                dummy_field: false,
                _T: ::std::marker::PhantomData,
            }
        }
    }
    "#);
}

#[test]
fn empty_struct_with_ability() {
    // NOTE: the generated code implements `moverox_traits::HasKey` for the struct because it has
    // the `key` ability. Obviously:
    // * The input isn't valid Sui Move; structs with `key` must have their first field be `id:
    //   UID`
    // * The output isn't valid Rust
    //
    // So, we don't bother to check for this scenario at the macro level
    insta::assert_snapshot!(from_struct("struct OTW has key {}"), @r#"
    #[derive(
        Default,
        Clone,
        Debug,
        PartialEq,
        Eq,
        Hash,
        ::moverox::traits::MoveDatatype,
        ::moverox::serde::Deserialize,
        ::moverox::serde::Serialize,
    )]
    #[move_(crate = ::moverox::traits)]
    #[serde(crate = "::moverox::serde")]
    #[allow(non_snake_case)]
    pub struct OTW {
        /// BCS for empty structs actually encodes a single boolean hidden field
        dummy_field: bool,
    }
    impl OTW {
        #[allow(clippy::just_underscores_and_digits, clippy::too_many_arguments)]
        pub const fn new() -> Self {
            Self { dummy_field: false }
        }
    }
    impl ::moverox::traits::HasKey for OTW {
        fn address(&self) -> ::moverox::types::Address {
            self.id.id.bytes
        }
    }
    "#);
}

#[test]
fn empty_struct_with_abilities() {
    // NOTE: see the note in the `empty_struct_with_ability` test
    insta::assert_snapshot!(from_struct("struct OTW has key, store {}"), @r#"
    #[derive(
        Default,
        Clone,
        Debug,
        PartialEq,
        Eq,
        Hash,
        ::moverox::traits::MoveDatatype,
        ::moverox::serde::Deserialize,
        ::moverox::serde::Serialize,
    )]
    #[move_(crate = ::moverox::traits)]
    #[serde(crate = "::moverox::serde")]
    #[allow(non_snake_case)]
    pub struct OTW {
        /// BCS for empty structs actually encodes a single boolean hidden field
        dummy_field: bool,
    }
    impl OTW {
        #[allow(clippy::just_underscores_and_digits, clippy::too_many_arguments)]
        pub const fn new() -> Self {
            Self { dummy_field: false }
        }
    }
    impl ::moverox::traits::HasKey for OTW {
        fn address(&self) -> ::moverox::types::Address {
            self.id.id.bytes
        }
    }
    "#);
}

#[test]
fn struct_with_non_doc_attribute() {
    insta::assert_snapshot!(from_struct("\
        #[attr]
        struct Name {}
        "), @r#"
    #[derive(
        Default,
        Clone,
        Debug,
        PartialEq,
        Eq,
        Hash,
        ::moverox::traits::MoveDatatype,
        ::moverox::serde::Deserialize,
        ::moverox::serde::Serialize,
    )]
    #[move_(crate = ::moverox::traits)]
    #[serde(crate = "::moverox::serde")]
    #[allow(non_snake_case)]
    pub struct Name {
        /// BCS for empty structs actually encodes a single boolean hidden field
        dummy_field: bool,
    }
    impl Name {
        #[allow(clippy::just_underscores_and_digits, clippy::too_many_arguments)]
        pub const fn new() -> Self {
            Self { dummy_field: false }
        }
    }
    "#);
}

#[test]
fn struct_with_field() {
    insta::assert_snapshot!(from_struct("\
        public struct Admin has key {
            id: UID
        }
        "), @r#"
    #[derive(
        Clone,
        Debug,
        PartialEq,
        Eq,
        Hash,
        ::moverox::traits::MoveDatatype,
        ::moverox::serde::Deserialize,
        ::moverox::serde::Serialize,
    )]
    #[move_(crate = ::moverox::traits)]
    #[serde(crate = "::moverox::serde")]
    #[allow(non_snake_case)]
    pub struct Admin {
        pub id: UID,
    }
    impl Admin {
        #[allow(clippy::just_underscores_and_digits, clippy::too_many_arguments)]
        pub const fn new(id: UID) -> Self {
            Self { id }
        }
    }
    impl ::moverox::traits::HasKey for Admin {
        fn address(&self) -> ::moverox::types::Address {
            self.id.id.bytes
        }
    }
    "#);
}

#[test]
fn struct_with_field_and_phantom() {
    insta::assert_snapshot!(from_struct("\
        public struct Admin<phantom T> has key {
            id: UID
        }
        "), @r#"
    #[derive(
        Clone,
        Debug,
        PartialEq,
        Eq,
        Hash,
        ::moverox::traits::MoveDatatype,
        ::moverox::serde::Deserialize,
        ::moverox::serde::Serialize,
    )]
    #[move_(crate = ::moverox::traits)]
    #[serde(crate = "::moverox::serde")]
    #[allow(non_snake_case)]
    pub struct Admin<T> {
        pub id: UID,
        #[serde(skip)]
        _T: ::std::marker::PhantomData<T>,
    }
    impl<T> Admin<T> {
        #[allow(clippy::just_underscores_and_digits, clippy::too_many_arguments)]
        pub const fn new(id: UID) -> Self {
            Self {
                id,
                _T: ::std::marker::PhantomData,
            }
        }
    }
    impl<T: ::moverox::traits::MoveType> ::moverox::traits::HasKey for Admin<T> {
        fn address(&self) -> ::moverox::types::Address {
            self.id.id.bytes
        }
    }
    "#);
}

#[test]
fn struct_with_fields() {
    insta::assert_snapshot!(from_struct("\
        public struct Admin has key {
            id: UID,
            sender: address,
            object: ID
        }
        "), @r#"
    #[derive(
        Clone,
        Debug,
        PartialEq,
        Eq,
        Hash,
        ::moverox::traits::MoveDatatype,
        ::moverox::serde::Deserialize,
        ::moverox::serde::Serialize,
    )]
    #[move_(crate = ::moverox::traits)]
    #[serde(crate = "::moverox::serde")]
    #[allow(non_snake_case)]
    pub struct Admin {
        pub id: UID,
        pub sender: address,
        pub object: ID,
    }
    impl Admin {
        #[allow(clippy::just_underscores_and_digits, clippy::too_many_arguments)]
        pub const fn new(id: UID, sender: address, object: ID) -> Self {
            Self { id, sender, object }
        }
    }
    impl ::moverox::traits::HasKey for Admin {
        fn address(&self) -> ::moverox::types::Address {
            self.id.id.bytes
        }
    }
    "#);
}

#[test]
fn struct_with_annotated_fields() {
    insta::assert_snapshot!(from_struct("\
        /// A general 'object admin'.
        public struct Admin has key {
            id: UID,
            /// Transaction sender with irrevokable privileged access.
            sender: address,
            /// Object being admistrated. Never changes after construction.
            object: ID
        }
        "), @r#"
    /// A general 'object admin'.
    #[derive(
        Clone,
        Debug,
        PartialEq,
        Eq,
        Hash,
        ::moverox::traits::MoveDatatype,
        ::moverox::serde::Deserialize,
        ::moverox::serde::Serialize,
    )]
    #[move_(crate = ::moverox::traits)]
    #[serde(crate = "::moverox::serde")]
    #[allow(non_snake_case)]
    pub struct Admin {
        pub id: UID,
        /// Transaction sender with irrevokable privileged access.
        pub sender: address,
        /// Object being admistrated. Never changes after construction.
        pub object: ID,
    }
    impl Admin {
        #[allow(clippy::just_underscores_and_digits, clippy::too_many_arguments)]
        pub const fn new(id: UID, sender: address, object: ID) -> Self {
            Self { id, sender, object }
        }
    }
    impl ::moverox::traits::HasKey for Admin {
        fn address(&self) -> ::moverox::types::Address {
            self.id.id.bytes
        }
    }
    "#);
}

#[test]
fn empty_tuple_struct() {
    insta::assert_snapshot!(from_struct("public struct Wut()"), @r#"
    #[derive(
        Default,
        Clone,
        Debug,
        PartialEq,
        Eq,
        Hash,
        ::moverox::traits::MoveDatatype,
        ::moverox::serde::Deserialize,
        ::moverox::serde::Serialize,
    )]
    #[move_(crate = ::moverox::traits)]
    #[serde(crate = "::moverox::serde")]
    #[allow(non_snake_case)]
    pub struct Wut(bool);
    impl Wut {
        #[allow(clippy::just_underscores_and_digits, clippy::too_many_arguments)]
        pub const fn new() -> Self {
            Self(false)
        }
    }
    "#);
}

#[test]
fn empty_tuple_struct_with_ability() {
    insta::assert_snapshot!(from_struct("public struct Wut() has drop;"), @r#"
    #[derive(
        Default,
        Clone,
        Debug,
        PartialEq,
        Eq,
        Hash,
        ::moverox::traits::MoveDatatype,
        ::moverox::serde::Deserialize,
        ::moverox::serde::Serialize,
    )]
    #[move_(crate = ::moverox::traits)]
    #[serde(crate = "::moverox::serde")]
    #[allow(non_snake_case)]
    pub struct Wut(bool);
    impl Wut {
        #[allow(clippy::just_underscores_and_digits, clippy::too_many_arguments)]
        pub const fn new() -> Self {
            Self(false)
        }
    }
    "#);
}

#[test]
fn tuple_struct_with_fields_and_ability() {
    insta::assert_snapshot!(from_struct("public struct Wut(u64, address) has drop;"), @r#"
    #[derive(
        Clone,
        Debug,
        PartialEq,
        Eq,
        Hash,
        ::moverox::traits::MoveDatatype,
        ::moverox::serde::Deserialize,
        ::moverox::serde::Serialize,
    )]
    #[move_(crate = ::moverox::traits)]
    #[serde(crate = "::moverox::serde")]
    #[allow(non_snake_case)]
    pub struct Wut(pub u64, pub address);
    impl Wut {
        #[allow(clippy::just_underscores_and_digits, clippy::too_many_arguments)]
        pub const fn new(_0: u64, _1: address) -> Self {
            Self(_0, _1)
        }
    }
    "#);
}

#[test]
fn empty_phantom_generic_tuple_struct() {
    insta::assert_snapshot!(from_struct("public struct Wut<phantom T>()"), @r#"
    #[derive(
        Default,
        Clone,
        Debug,
        PartialEq,
        Eq,
        Hash,
        ::moverox::traits::MoveDatatype,
        ::moverox::serde::Deserialize,
        ::moverox::serde::Serialize,
    )]
    #[move_(crate = ::moverox::traits)]
    #[serde(crate = "::moverox::serde")]
    #[allow(non_snake_case)]
    pub struct Wut<T>(bool, #[serde(skip)] ::std::marker::PhantomData<T>);
    impl<T> Wut<T> {
        #[allow(clippy::just_underscores_and_digits, clippy::too_many_arguments)]
        pub const fn new() -> Self {
            Self(false, ::std::marker::PhantomData)
        }
    }
    "#);
}

#[test]
fn tuple_struct_with_generic_field_type_and_ability() {
    insta::assert_snapshot!(from_struct("public struct Wut<T>(T) has drop;"), @r#"
    #[derive(
        Clone,
        Debug,
        PartialEq,
        Eq,
        Hash,
        ::moverox::traits::MoveDatatype,
        ::moverox::serde::Deserialize,
        ::moverox::serde::Serialize,
    )]
    #[move_(crate = ::moverox::traits)]
    #[serde(crate = "::moverox::serde")]
    #[allow(non_snake_case)]
    pub struct Wut<T>(pub T);
    impl<T> Wut<T> {
        #[allow(clippy::just_underscores_and_digits, clippy::too_many_arguments)]
        pub const fn new(_0: T) -> Self {
            Self(_0)
        }
    }
    "#);
}

#[test]
fn module_with_struct() {
    insta::assert_snapshot!(from_module(
            "
        module package::admin {
            /// A general 'object admin'.
            public struct Admin has key {
                id: UID,
                /// Transaction sender with irrevokable privileged access.
                sender: address,
                /// Object being admistrated. Never changes after construction.
                object: ID
            }
        }
        "
        ), @r#"
    #[allow(rustdoc::all)]
    #[cfg(not(doctest))]
    pub mod admin {
        #[allow(non_camel_case_types, unused)]
        type address = ::moverox::types::Address;
        #[allow(non_camel_case_types, unused)]
        type u256 = ::moverox::types::U256;
        #[allow(non_camel_case_types, unused)]
        type vector<T> = ::std::vec::Vec<T>;
        /// A general 'object admin'.
        #[derive(
            Clone,
            Debug,
            PartialEq,
            Eq,
            Hash,
            ::moverox::traits::MoveDatatype,
            ::moverox::serde::Deserialize,
            ::moverox::serde::Serialize,
        )]
        #[move_(crate = ::moverox::traits)]
        #[serde(crate = "::moverox::serde")]
        #[move_(module = admin)]
        #[allow(non_snake_case)]
        pub struct Admin {
            pub id: UID,
            /// Transaction sender with irrevokable privileged access.
            pub sender: address,
            /// Object being admistrated. Never changes after construction.
            pub object: ID,
        }
        impl Admin {
            #[allow(clippy::just_underscores_and_digits, clippy::too_many_arguments)]
            pub const fn new(id: UID, sender: address, object: ID) -> Self {
                Self { id, sender, object }
            }
        }
        impl ::moverox::traits::HasKey for Admin {
            fn address(&self) -> ::moverox::types::Address {
                self.id.id.bytes
            }
        }
    }
    "#);
}
