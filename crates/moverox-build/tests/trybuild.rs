use std::path::Path;

use indoc::indoc;
use tempfile::TempDir;
use testresult::TestResult;

#[test]
fn trybuild() -> TestResult {
    // NOTE: these return the temporary directories so that they are deleted at the end of the
    // tests when dropped.
    let (sui_stdlib, _guard1) = sui_stdlib()?;
    let (enums, _guard2) = enums()?;

    let cases = trybuild::TestCases::new();
    cases.pass(sui_stdlib);
    cases.pass(enums);
    Ok(())
}

fn sui_stdlib() -> TestResult<(impl AsRef<Path>, TempDir)> {
    let tempdir = tempfile::tempdir()?;

    let main = tempdir.path().join("sui_stdlib.rs");
    let contents = indoc! {"
        mod move_stdlib;
        mod sui_framework;

        fn main() {}
    "};
    std::fs::write(&main, contents)?;

    let pkg_path = move_dir()?.join("move-stdlib");
    moverox_build::move_package(pkg_path, "move_stdlib")
        .published_at("0x1")
        .out_dir(tempdir.path())
        .build()?;

    let pkg_path = move_dir()?.join("sui-framework");
    moverox_build::move_package(pkg_path, "sui_framework")
        .published_at("0x2")
        .with_implicit_sui_imports()
        .map_address("std", "crate::move_stdlib")
        .out_dir(tempdir.path())
        .build()?;

    Ok((main, tempdir))
}

fn enums() -> TestResult<(impl AsRef<Path>, TempDir)> {
    let tempdir = tempfile::tempdir()?;

    let main = tempdir.path().join("enums.rs");
    let contents = indoc! {"
        mod enums_;
        mod move_stdlib;

        fn main() {}
    "};
    std::fs::write(&main, contents)?;

    // NOTE: including a copy MoveStdlib for compilation purposes; in practice, this would be
    // reused from another crate like `moverox-sui` instead
    let pkg_path = move_dir()?.join("move-stdlib");
    moverox_build::move_package(pkg_path, "move_stdlib")
        .published_at("0x1")
        .out_dir(tempdir.path())
        .build()?;

    let pkg_path = move_dir()?.join("enums");
    moverox_build::move_package(pkg_path, "enums_")
        .with_implicit_sui_imports()
        .map_address("std", "crate::move_stdlib")
        .out_dir(tempdir.path())
        .build()?;

    Ok((main, tempdir))
}

fn move_dir() -> TestResult<std::path::PathBuf> {
    Ok(Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .ok_or("../")?
        .parent()
        .ok_or("../")?
        .join("move"))
}
