/// Taken from
/// <https://github.com/cargo-public-api/cargo-public-api?tab=readme-ov-file#-as-a-ci-check>
#[test]
fn public_api() {
    // Build rustdoc JSON
    let rustdoc_json = rustdoc_json::Builder::default()
        .all_features(true)
        .build()
        .unwrap();

    // Derive the public API from the rustdoc JSON
    let public_api = public_api::Builder::from_rustdoc_json(rustdoc_json)
        .omit_blanket_impls(true)
        .omit_auto_trait_impls(true)
        .omit_auto_derived_impls(true)
        .build()
        .unwrap();

    // Assert that the public API looks correct
    insta::assert_snapshot!(public_api);
}
