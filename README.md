# MoveRox 🤘

Move, oxidized.

[![Crates.io](https://img.shields.io/crates/v/moverox.svg)](https://crates.io/crates/moverox)
[![Docs.rs](https://docs.rs/moverox/badge.svg)](https://docs.rs/moverox)
[![CI](https://github.com/0xangelo/moverox/workflows/CI/badge.svg)](https://github.com/0xangelo/moverox/actions)

Easily convert datatypes (structs/enums) from your Move packages into "oxidized", [BCS]-compatible versions and seamlessly keep them up-to-date as you update your Move code.

[BCS]: https://docs.rs/bcs/latest/bcs/

## Features

Let's take, for example, the dynamic field struct defined in the [Sui] framework package:
```move
// module sui::dynamic_field; 

/// Internal object used for storing the field and value
public struct Field<Name: copy + drop + store, Value: store> has key {
    /// Determined by the hash of the object ID, the field name value and it's type,
    /// i.e. hash(parent.id || name || Name)
    id: UID,
    /// The value for the name of this field
    name: Name,
    /// The value bound to this field
    value: Value,
}
```

### Synchronizing Rust with Move

We can oxidize this struct (and all datatypes in the [Sui] Move package) by using [`moverox-build`]. As the name suggests, that is done in the `build.rs` script for your crate. See [`moverox-sui/build.rs`] for an example of how it's done in practice. The advantage of using a build script is that it will re-run every time the linked Move sources change, **keeping your oxidized Move datatypes in sync with your Move code**. 

To include the generated Rust code in your crate, simply use `moverox::include_oxidized!` like so:
```rust
/// Oxidized `Sui` @ `0x2`.
pub mod sui {
    moverox::include_oxidized!("sui");
}
```

### BCS-compatible oxidized datatypes

The `Field` struct above will be translated into the Rust equivalent:
```rust
// Simplified from `moverox-build`'s output
pub mod dynamic_field {
    /// Internal object used for storing the field and value
    #[derive(
        Clone,
        Debug,
        PartialEq,
        Eq,
        Hash,
        serde::Deserialize,
        serde::Serialize,
        ::moverox::traits::MoveDatatype,
    )]
    #[move_(address = "0x2")]
    #[move_(module = dynamic_field)]
    pub struct Field<Name, Value> {
        /// Determined by the hash of the object ID, the field name value and it's type,
        /// i.e. hash(parent.id || name || Name)
        pub id: super::object::UID,
        /// The value for the name of this field
        pub name: Name,
        /// The value bound to this field
        pub value: Value,
    }
    impl<Name, Value> Field<Name, Value> {
        pub const fn new(id: super::object::UID, name: Name, value: Value) -> Self {
            Self { id, name, value }
        }
    }
    impl<Name, Value> ::moverox::traits::HasKey for Field<Name, Value> {
        fn address(&self) -> ::moverox::types::Address {
            self.id.id.bytes
        }
    }
}
```
Notice:
- the type is declared under `mod dynamic_field`, matching the Move module it comes from
- `serde` traits, since it's BCS-compatible with how the Move type is serialized
- docs are inherited from the Move code
- `id: super::object::UID` because `moverox-build` recognizes `UID` is an implicit import, resolves to its full path and sees that the type is defined in the same package (a `mod object` was also generated and contains the oxidized `UID`)
- `HasKey` trait implementation because the Move type has the `key` ability
- `MoveDatatype` derivation, more on this below

### Smart type tags

Notice `Field` is annotated with `#[derive(MoveDatatype)]`, `#[move_(address = "0x2")]`, and `#[move_(module = dynamic_field)]`. All of that pertains to the last layer of code generation, which is this case produces:
```rust
pub struct FieldTypeTag<Name: MoveTypeTag, Value: MoveTypeTag> {
    pub type_name: Name,
    pub type_value: Value,
}
```
The type above is a specialization of the generic Move [`StructTag`]. Notice that, unlike the latter, `FieldTypeTag`:
- doesn't have an `address` field because of the `#[move_(address = "0x2")]` attribute, therefore it implements [`ConstAddress`] with that value, i.e., we know `Field` is defined in a package published at `0x2`
- doesn't have a `module` field because of the `#[move_(module = dynamic_field)]` attribute, therefore it implements [`ConstModule`] with that value, i.e., we know `Field` is defined in a module named `dynamic_field`
- doesn't have a `name` field because `Field` is obviously the name we expect, therefore it implements [`ConstName`] with that value
- has `type_name` and `type_value` instead of `type_params`, because `Field` can only have 2 type parameters

All of the above is leveraged when parsing a string into a `FieldTypeTag` (or converting a `StructTag` into it). For example, for a `Field<Vec<u8>, u64>` type, the corresponding `FieldTypetag` (`<Field<Vec<u8>, u64> as MoveDatatype>::StructTag`) expects a specific string representation:
```rust
"0xabc::dynamic_field::Field<vector<u8>, u64>" // Converting from StructTag: Wrong address: expected 0x0000000000000000000000000000000000000000000000000000000000000002, got 0x0000000000000000000000000000000000000000000000000000000000000abc
"0x2::field::Field<vector<u8>, u64>" // Converting from StructTag: Wrong module: expected dynamic_field, got field
"0x2::dynamic_field::DynamicField<vector<u8>, u64>" // Converting from StructTag: Wrong name: expected Field, got DynamicField
"0x2::dynamic_field::Field<vector<u8>, u64, u64>" // Converting from StructTag: Wrong type parameters: Wrong number of generics: expected 2, got 3
"0x2::dynamic_field::Field<vector<u8>, u64>" // Passes
```

### Safe deserialization

The `moverox::parse_move_datatype` helper combines the above to safely deserialize raw data returned from RPCs, in `CheckpointData` or in dev-inpect Move call returns. For example, imagine we have the following Move datatypes:
```move
// module manager;

struct ManagerCap has key {
    id: UID,
    last_used: u64,
}

// module oracle;

struct OracleCap has key {
    id: UID,
    last_used: u64,
}
```
These are completely different types, but they share the same BCS representation. By oxidizing them with `moverox-build` and using the `moverox::parse_move_datatype` function, we short-circuit deserialization if the type tag doesn't confirm to our expected format:
```rust
let tag: StructTag = "0xbeef::manager::ManagerCap".parse().unwrap();
let value = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 215, 108, 134, 104, 0, 0, 0, 0,
];

// Simply deserializing the BCS bytes succeeds for any of the two.
assert!(bcs::from_bytes::<OracleCap>(&value).is_ok());
assert!(bcs::from_bytes::<ManagerCap>(&value).is_ok());

// `parse_move_datatype` requires struct tag information and leverages the specialized
// `MoveDatatype::StructTag` to avoid deserializing if the type tags don't match
assert!(parse_move_datatype::<OracleCap>(&tag, &value).is_err());
assert!(parse_move_datatype::<ManagerCap>(&tag, &value).is_ok());
```


### Move syntax parsing

The cornerstone of almost all the crates in this project is the [`move-syn`] crate, which uses [`unsynn`] to parse Move syntax into a structured intermediary representation. This repository leverages it primarily for Rust code generation, but it can be a building block for many other types of applications.


[Sui]: https://github.com/MystenLabs/sui/tree/main/crates/sui-framework/packages/sui-framework
[`moverox-build`]: ./crates/moverox-build
[`moverox-sui/build.rs`]: ./crates/moverox-sui/build.rs
[`StructTag`]: https://docs.rs/moverox-types/latest/moverox_types/struct.StructTag.html
[`ConstAddress`]: https://docs.rs/moverox-traits/latest/moverox_traits/trait.ConstAddress.html
[`ConstModule`]: https://docs.rs/moverox-traits/latest/moverox_traits/trait.ConstModule.html
[`ConstName`]: https://docs.rs/moverox-traits/latest/moverox_traits/trait.ConstName.html
[`move-syn`]: ./crates/move-syn
[`unsynn`]: https://docs.rs/unsynn

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

## Inspirations

This project was mostly inspired by [`tonic-build`] in how it generates Rust code from Protobuf files and the declarative macro [`tonic`] uses for including generated files in crates.

It incorporates a lot of lessons learned from my previous work developing [`af-sui-pkg-sdk`], which had similar goals but exported a declarative macro with lots of limitations and required manual work to keep the generated types updated as the Move code evolved.


[`tonic`]: https://github.com/hyperium/tonic
[`tonic-build`]: https://docs.rs/tonic-build
[`af-sui-pkg-sdk`]: https://docs.rs/af-sui-pkg-sdk
