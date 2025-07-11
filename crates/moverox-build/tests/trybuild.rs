use std::path::Path;

use indoc::indoc;
use testresult::TestResult;

#[test]
fn trybuild() -> TestResult {
    let tempdir = tempfile::tempdir()?;

    let main = tempdir.path().join("main.rs");
    let contents = indoc! {"
        mod move_stdlib;
        mod sui_framework;

        fn main() {}
    "};
    std::fs::write(&main, contents)?;

    let pkg_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("move")
        .join("move-stdlib");

    moverox_build::move_package(pkg_path, "move_stdlib")
        .published_at("0x1")
        .out_dir(tempdir.path())
        .build()?;

    let pkg_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("move")
        .join("sui-framework");

    moverox_build::move_package(pkg_path, "sui_framework")
        .published_at("0x2")
        .with_implicit_sui_imports()
        .map_address("std", "crate::move_stdlib")
        .out_dir(tempdir.path())
        .build()?;

    let cases = trybuild::TestCases::new();
    cases.pass(main);
    Ok(())
}
