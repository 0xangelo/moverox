use std::path::Path;

use itertools::Itertools as _;
use testresult::TestResult;

use crate::move_package;

#[test]
fn generate_rust_for_move_stdlib() -> TestResult {
    let pkg_path = move_dir()?.join("move-stdlib");

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
    let pkg_path = move_dir()?.join("sui-framework");

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
    bls12381.move
    ecdsa_k1.move
    ecdsa_r1.move
    ecvrf.move
    ed25519.move
    groth16.move
    group_ops.move
    hash.move
    hmac.move
    nitro_attestation.move
    poseidon.move
    vdf.move
    zklogin_verified_id.move
    zklogin_verified_issuer.move
    deny_list.move
    display.move
    dynamic_field.move
    dynamic_object_field.move
    event.move
    hex.move
    kiosk.move
    kiosk_extension.move
    transfer_policy.move
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
    test_scenario.move
    test_utils.move
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

#[test]
fn generate_rust_for_enums() -> TestResult {
    let pkg_path = move_dir()?.join("enums");

    let builder = move_package(pkg_path, "Enums");
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
    main.move
    other.move
    ");

    let modules = builder.parse_files(&move_files)?;
    let rust_code = builder.generate_rust_str(&modules)?;

    let enums = prettyplease::unparse(&syn::parse_file(&rust_code).unwrap());
    insta::assert_snapshot!("Enums", enums);
    Ok(())
}

fn move_dir() -> TestResult<std::path::PathBuf> {
    Ok(Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .ok_or("moverox-build/../")?
        .parent()
        .ok_or("moverox-build/../../")?
        .join("move"))
}
