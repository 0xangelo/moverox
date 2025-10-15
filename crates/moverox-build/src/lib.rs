#![cfg_attr(all(doc, not(doctest)), feature(doc_cfg))]

//! Build script utilities to oxidize an entire Move package.

use std::collections::HashMap;
use std::error::Error as StdError;
use std::fs;
use std::io::Write as _;
use std::path::{Path, PathBuf};

use move_syn::sanitize_for_tokenizer;
use move_syn::unsynn::{IParse as _, Ident, Span, ToTokens as _, TokenStream};
use moverox_codegen::ModuleGen as _;

#[cfg(test)]
mod tests;

const MOVE_FILE_EXT: &str = "move";

type Result<T, E = Box<dyn StdError + Send + Sync + 'static>> = ::std::result::Result<T, E>;

/// Initialize a builder for oxidizing the Move package at `pkg_path`, being available to import
/// later via the `name`.
pub fn move_package(pkg_path: impl AsRef<Path>, name: &str) -> Builder<'_> {
    Builder {
        pkg_path: pkg_path.as_ref().to_owned(),
        name,
        with_implicit_iota_imports: false,
        with_implicit_sui_imports: false,
        emit_rerun_if_changed: std::env::var_os("CARGO").is_some(),
        out_dir: None,
        moverox_path: "::moverox".to_token_stream(),
        address_map: Default::default(),
        published_at: None,
    }
}

pub struct Builder<'a> {
    pkg_path: PathBuf,
    name: &'a str,
    with_implicit_iota_imports: bool,
    with_implicit_sui_imports: bool,
    emit_rerun_if_changed: bool,
    out_dir: Option<PathBuf>,
    moverox_path: TokenStream,
    address_map: HashMap<Ident, TokenStream>,
    published_at: Option<&'a str>,
}

impl<'a> Builder<'a> {
    /// Add `iota` implicit imports as explicit `use` statements to the module.
    ///
    /// Adapted from the `sui` equivalents.
    pub const fn with_implicit_iota_imports(mut self) -> Self {
        self.with_implicit_iota_imports = true;
        self
    }

    /// Add `sui` implicit imports as explicit `use` statements to the module.
    ///
    /// This is done after reading the Move files and before generating the Rust code. Without this,
    /// datatypes with field types like `UID` which aren't explicitly imported in Move modules will
    /// fail to compile because they will not have the full path to the Rust equivalent.
    ///
    /// [Reference](https://move-book.com/programmability/sui-framework#implicit-imports)
    pub const fn with_implicit_sui_imports(mut self) -> Self {
        self.with_implicit_sui_imports = true;
        self
    }

    /// Path to the module containing the necessary exports that the generated code needs.
    ///
    /// Defaults to `::moverox`.
    ///
    /// `rust_path` must point to a crate/module which exports:
    /// - a `types` module with `Address` and `U256` types from `moverox-types`
    /// - a `traits` module with `HasKey`, `MoveDatatype` and `MoveType` traits from `moverox-traits`
    /// - the `serde` crate
    pub fn moverox_path(mut self, rust_path: &str) -> Self {
        self.moverox_path = rust_path.to_token_stream();
        self
    }

    /// Map a Move named address to the path of an oxidized Move package.
    ///
    /// This is necessary for Move code that depends on other packages. The idea is to 'oxidize'
    /// those dependency packages first and them substitute their paths for the Move paths in the
    /// source code this builder is processing.
    ///
    /// # Panics
    ///
    /// If `named_address` is not a valid identifier
    pub fn map_address(mut self, named_address: &str, rust_path: &str) -> Self {
        self.address_map.insert(
            Ident::new(named_address, Span::call_site()),
            rust_path.to_token_stream(),
        );
        self
    }

    pub const fn published_at(mut self, hex_address: &'a str) -> Self {
        self.published_at = Some(hex_address);
        self
    }

    /// Enable or disable emitting
    /// [`cargo:rerun-if-changed=PATH`](https://doc.rust-lang.org/cargo/reference/build-scripts.html#rerun-if-changed)
    /// instructions for Cargo.
    ///
    /// If set, writes instructions to `stdout` for Cargo so that it understands
    /// when to rerun the build script. By default, this setting is enabled if
    /// the `CARGO` environment variable is set. The `CARGO` environment
    /// variable is set by Cargo for build scripts. Therefore, this setting
    /// should be enabled automatically when run from a build script. However,
    /// the method of detection is not completely reliable since the `CARGO`
    /// environment variable can have been set by anything else. If writing the
    /// instructions to `stdout` is undesirable, you can disable this setting
    /// explicitly.
    pub const fn emit_rerun_if_changed(mut self, enable: bool) -> Self {
        self.emit_rerun_if_changed = enable;
        self
    }

    /// Configures the output directory where generated Rust files will be written.
    ///
    /// If unset, defaults to the `OUT_DIR` environment variable. `OUT_DIR` is set by Cargo when
    /// executing build scripts, so `out_dir` typically does not need to be configured.
    pub fn out_dir(mut self, path: impl Into<PathBuf>) -> Self {
        self.out_dir = Some(path.into());
        self
    }

    pub fn build(self) -> Result<()> {
        let move_files = self.collect_move_files()?;

        let modules = self.parse_files(&move_files)?;

        let rust_code = self.generate_rust_str(&modules)?;

        let target = self
            .out_dir
            .map_or_else(default_out_dir, Ok)?
            .join(format!("{}.rs", self.name));

        let mut file = fs::File::create(&target)?;
        file.write_all(b"// This file is @generated by moverox-build.\n")?;
        file.write_all(rust_code.as_bytes())?;

        Ok(())
    }

    fn collect_move_files(&self) -> Result<Vec<PathBuf>> {
        let move_sources = self.pkg_path.join("sources");

        let mut move_files = vec![];
        for entry in fs::read_dir(move_sources)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().is_none_or(|ext| ext != MOVE_FILE_EXT) {
                continue;
            }
            if self.emit_rerun_if_changed {
                // Tell Cargo to rerun the build script if .move files change
                println!("cargo:rerun-if-changed={}", path.display());
            }
            move_files.push(path);
        }
        Ok(move_files)
    }

    /// Parse the Move files and apply any modifications pre-Rust code generation.
    fn parse_files(&self, move_files: &[PathBuf]) -> Result<Vec<move_syn::Module>> {
        let mut move_modules = Vec::with_capacity(move_files.len());
        for path in move_files {
            // Read the .move file
            let contents = sanitize_for_tokenizer(&fs::read_to_string(path)?);

            // Parse to IR
            let parsed_file: move_syn::File = contents
                .into_token_iter()
                .parse_all()
                .map_err(|e| anyhow::anyhow!(e.to_string()))?;

            for mut module in parsed_file.into_modules() {
                if self.with_implicit_iota_imports {
                    module.with_implicit_iota_imports();
                }
                if self.with_implicit_sui_imports {
                    module.with_implicit_sui_imports();
                }
                module.fully_qualify_datatype_field_types();
                move_modules.push(module);
            }
        }
        Ok(move_modules)
    }

    fn generate_rust_str(&self, move_modules: &[move_syn::Module]) -> Result<String> {
        let mut address_map = self.address_map.clone();
        // If any type path starts with one of the named addresses of the package's modules,
        // substitute that named address prefix with `super`, since oxidized modules will all
        // be right under the same 'super' module.
        for module in move_modules {
            address_map.insert(module.named_address.clone(), "super".to_token_stream());
        }

        let package_address = self
            .published_at
            .map(move_syn::unsynn::LiteralString::from_str);

        // Collect generated Rust code
        let mut generated_code = String::new();
        for module in move_modules {
            // Skip module generation if no datatypes are found
            if !module.items().any(|item| item.kind.is_datatype()) {
                continue;
            }
            let rust_code = module
                .to_rust(&self.moverox_path, package_address.as_ref(), &address_map)
                .to_string();
            generated_code.push_str(&rust_code);
            generated_code.push('\n');
        }
        Ok(generated_code)
    }
}

fn default_out_dir() -> Result<PathBuf> {
    Ok(std::env::var_os("OUT_DIR")
        .ok_or("OUT_DIR environment variable is not set")?
        .into())
}
