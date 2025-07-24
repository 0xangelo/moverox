<!-- cargo-rdme start -->

Traits for rusty Move types and their type tags.

The core items are `MoveType` and `MoveTypeTag`. These
are useful trait bounds to use when dealing with generic off-chain Move type representations.
They are implemented for the primitive types that correspond to Move's primitives
(integers/bool).

For Move structs, `MoveDatatype` should be used as it has an
associated `MoveDatatypeTag`. The
[`MoveDatatype`](moverox_traits_derive::MoveDatatype) derive macro is exported for automatically
creating a `MoveDatatypeTag` implementation from normal Rust struct declarations.

<!-- cargo-rdme end -->
