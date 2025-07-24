#!/usr/bin/env bash

REPO_ROOT="$(git rev-parse --show-toplevel)"

cd $REPO_ROOT

if [ -z "$1" ]
then
  SCOPE=""
else
  SCOPE="-p $1"
fi

set -ex

RUSTDOCFLAGS="-A rustdoc::redundant-explicit-links -D warnings -Zunstable-options --generate-link-to-definition" \
  RUSTC_BOOTSTRAP=1 \
  cargo +nightly doc $SCOPE --all-features ${@:2}
