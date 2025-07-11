/// Include oxidized Move package.
///
/// You must specify the Move package name supplied to `moverox_build::move_package`.
///
/// ```rust,ignore
/// mod package {
///     moverox::include_oxidized!("package");
/// }
/// ```
///
/// # Note:
/// **This only works if the `moverox_build` output directory has been unmodified**.
/// The default output directory is set to the [`OUT_DIR`] environment variable.
/// If the output directory has been modified, the following pattern may be used
/// instead of this macro.
///
/// ```rust,ignore
/// mod package {
///     include!("/relative/move/directory/package.rs");
/// }
/// ```
/// You can also use a custom environment variable using the following pattern.
/// ```rust,ignore
/// mod package {
///     include!(concat!(env!("MOVEROX"), "/package.rs"));
/// }
/// ```
///
/// [`OUT_DIR`]: https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-build-scripts
#[macro_export]
macro_rules! include_oxidized {
    ($package: tt) => {
        include!(concat!(env!("OUT_DIR"), concat!("/", $package, ".rs")));
    };
}
