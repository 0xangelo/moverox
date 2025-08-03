use std::path::Path;

use itertools::Itertools as _;
use testresult::TestResult;

use crate::{File, ItemKind};

mod move_stdlib;
mod sui_framework;

macro_rules! test_files {
    ($package:literal { $($file: ident),* $(,)? }) => {$(
        #[test]
        fn $file() -> TestResult {
            use unsynn::{IParse as _, ToTokens as _};
            let path = move_dir()?
                .join($package)
                .join("sources")
                .join(concat!(stringify!($file), ".move"));
            let content = std::fs::read_to_string(path)?;
            let ast: File = crate::sanitize_for_tokenizer(&content)
                .as_str()
                .to_token_iter()
                .parse_all()?;
            insta::assert_snapshot!(flat_imports(ast));
            Ok(())
        }
    )*};
}

pub(crate) use test_files;

fn flat_imports(ast: File) -> String {
    ast.into_modules()
        .flat_map(|module| module.into_items())
        .filter_map(|item| match item.kind {
            ItemKind::Import(import) => Some(import),
            _ => None,
        })
        .flat_map(|import| import.flatten().collect_vec())
        .map(|(ident, import)| format!("{ident} -> {import}"))
        .join("\n")
}

fn move_dir() -> TestResult<std::path::PathBuf> {
    Ok(Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .ok_or("../")?
        .parent()
        .ok_or("../")?
        .join("move"))
}
