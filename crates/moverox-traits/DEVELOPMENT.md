# Future plans

Study what's needed to support Move enums.

It turns out that `StructTag` is also used to represent the fully-qualified type of an
`enum`. This makes sense, since enums share the same 'path' structure as structs:
`@address::module_name::Name(<(T),+>)?`.

Since most of the work that this crate does is define traits to annotate Rust
types that represent Move types, there doesn't seem to be any functionality
changes necessary.

Hence why we use `MoveDatatype`, following Mysten's naming used in
* `move_core_types::annotated_value`
* The GraphQL `IMoveDatatype` interface

It remains to be seen if the derive crate can handle enums or needs updates.
