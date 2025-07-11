<!-- cargo-rdme start -->

Generate Rust code from Move IR parsed by move-syn.

Defines extension traits to generate Rust code from Move intermediate representation.

`thecrate` in arguments here is the path to a crate/module which exports:
- a `types` module with `Address` and `U256` types from `moverox-types`
- a `traits` module with `HasKey`, `MoveDatatype` and `MoveType` traits from `moverox-traits`
- the `serde` crate

<!-- cargo-rdme end -->
