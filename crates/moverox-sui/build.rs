use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let move_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("move");

    moverox_build::move_package(move_dir.join("move-stdlib"), "std")
        .published_at("0x1")
        .build()?;

    moverox_build::move_package(move_dir.join("sui-framework"), "sui")
        .published_at("0x2")
        .with_implicit_sui_imports()
        .map_address("std", "crate::move_stdlib")
        .build()?;
    Ok(())
}
