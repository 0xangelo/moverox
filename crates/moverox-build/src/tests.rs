use std::path::Path;

use itertools::Itertools as _;
use testresult::TestResult;

use crate::move_package;

#[test]
fn generate_rust_for_move_stdlib() -> TestResult {
    let pkg_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("move")
        .join("move-stdlib");

    let builder = move_package(pkg_path, "MoveStdlib").published_at("0x1");
    let move_files = builder.collect_move_files()?;

    let files_found = move_files
        .iter()
        .map(|path| path.file_name().unwrap().display())
        .join("\n");
    insta::assert_snapshot!(files_found, @r"
    u32.move
    u64.move
    string.move
    bit_vector.move
    bool.move
    uq32_32.move
    fixed_point32.move
    type_name.move
    address.move
    unit_test.move
    u256.move
    hash.move
    vector.move
    u16.move
    u128.move
    option.move
    u8.move
    bcs.move
    macros.move
    debug.move
    ascii.move
    uq64_64.move
    ");

    let modules = builder.parse_files(&move_files)?;
    let rust_code = builder.generate_rust_str(&modules)?;

    let move_stdlib = prettyplease::unparse(&syn::parse_file(&rust_code).unwrap());
    insta::assert_snapshot!("MoveStdLib", move_stdlib);
    Ok(())
}

#[test]
fn generate_rust_for_sui_framework() -> TestResult {
    let pkg_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("move")
        .join("sui-framework");

    let builder = move_package(pkg_path, "Sui")
        .published_at("0x2")
        .with_implicit_sui_imports()
        .map_address("std", "::moverox_sui::std");
    let move_files = builder.collect_move_files()?;

    let files_found = move_files
        .iter()
        .map(|path| path.file_name().unwrap().display())
        .join("\n");
    insta::assert_snapshot!(files_found, @r"
    token.move
    bag.move
    priority_queue.move
    borrow.move
    config.move
    table.move
    dynamic_field.move
    linked_table.move
    vec_set.move
    party.move
    deny_list.move
    url.move
    object_bag.move
    address.move
    authenticator_state.move
    versioned.move
    pay.move
    balance.move
    sui.move
    package.move
    math.move
    table_vec.move
    event.move
    accumulator.move
    hex.move
    tx_context.move
    transfer.move
    clock.move
    display.move
    vec_map.move
    dynamic_object_field.move
    object.move
    coin.move
    bcs.move
    types.move
    object_table.move
    random.move
    prover.move
    ");

    let modules = builder.parse_files(&move_files)?;
    let rust_code = builder.generate_rust_str(&modules)?;

    let sui_framework = prettyplease::unparse(&syn::parse_file(&rust_code).unwrap());
    insta::assert_snapshot!("Sui", sui_framework);
    Ok(())
}
