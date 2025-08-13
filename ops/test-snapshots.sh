#!/usr/bin/env bash

DIR="$(realpath $(dirname $0))"
cd $DIR

REPO_ROOT="$(git rev-parse --show-toplevel)"
cd $REPO_ROOT

# For cleaner logs; we use explicit links for `cargo rdme`
export RUSTDOCFLAGS="-A rustdoc::redundant-explicit-links"

set -ex

cargo insta test --review --unreferenced=delete -j 4 --all-features --lib --bins --tests
