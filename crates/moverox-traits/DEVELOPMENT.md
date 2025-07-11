# Future plans

Study what's needed to support Move enums.

It turns out that `StructTag` is also used to represent the fully-qualified type of an
`enum`. This makes sense, since enums share the same 'path' structure as structs:
`0xpackage-id::module_name::Name(<(T),+>)?`.

Since most of the work that this crate does is define traits to annotate Rust
types that represent Move types, there doesn't seem to be any functionality
changes necessary.

For instance, the `MoveStruct` trait is perfectly fine to be used for enums from
a functionality point of view, because all the bounds required are shared
between structs and enums. You can see that, because structs and enums share
`StructTag`, the `MoveStructTag` trait is perfectly fine as well.

Perhaps what we could do then is rename `MoveStruct` to `MoveDatatype`, following
Mysten's naming used in
* `move_core_types::annotated_value`
* The GraphQL `IMoveDatatype` interface

The bulk of the work will be on `movers-type-derive`. I've opened a separate issue for it
