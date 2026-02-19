{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = {
    flake-utils,
    nixpkgs,
    rust-overlay,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        overlays = [(import rust-overlay)];
        pkgs = (import nixpkgs) {
          inherit overlays system;
        };
        rustToolChain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
      in {
        # For `nix develop`:
        devShell = pkgs.mkShell {
          buildInputs = [rustToolChain];
        };
      }
    );
}
