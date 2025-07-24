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
    let move_files = {
        let mut paths = builder.collect_move_files()?;
        paths.sort();
        paths
    };

    let files_found = move_files
        .iter()
        .map(|path| path.file_name().unwrap().display())
        .join("\n");
    insta::assert_snapshot!(files_found, @r"
    address.move
    ascii.move
    bcs.move
    bit_vector.move
    bool.move
    debug.move
    fixed_point32.move
    hash.move
    macros.move
    option.move
    string.move
    type_name.move
    u128.move
    u16.move
    u256.move
    u32.move
    u64.move
    u8.move
    unit_test.move
    uq32_32.move
    uq64_64.move
    vector.move
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
    let move_files = {
        let mut paths = builder.collect_move_files()?;
        paths.sort();
        paths
    };

    let files_found = move_files
        .iter()
        .map(|path| path.file_name().unwrap().display())
        .join("\n");
    insta::assert_snapshot!(files_found, @r"
    accumulator.move
    address.move
    authenticator_state.move
    bag.move
    balance.move
    bcs.move
    borrow.move
    clock.move
    coin.move
    config.move
    deny_list.move
    display.move
    dynamic_field.move
    dynamic_object_field.move
    event.move
    hex.move
    linked_table.move
    math.move
    object.move
    object_bag.move
    object_table.move
    package.move
    party.move
    pay.move
    priority_queue.move
    prover.move
    random.move
    sui.move
    table.move
    table_vec.move
    token.move
    transfer.move
    tx_context.move
    types.move
    url.move
    vec_map.move
    vec_set.move
    versioned.move
    ");

    let modules = builder.parse_files(&move_files)?;
    let rust_code = builder.generate_rust_str(&modules)?;

    let sui_framework = prettyplease::unparse(&syn::parse_file(&rust_code).unwrap());
    insta::assert_snapshot!("Sui", sui_framework);
    Ok(())
}
