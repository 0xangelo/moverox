#!/usr/bin/env bash

REPO_ROOT="$(git rev-parse --show-toplevel)"
DIR="$(realpath $(dirname $0))"

cd $REPO_ROOT

if [ -z "$1" ]
then
  SCOPE=""
else
  SCOPE="-p $1"
fi

set -ex

cargo +nightly hack $SCOPE --feature-powerset --exclude-features default clippy --no-deps --lib -- -D warnings
cargo nextest run $SCOPE --all-features -j 8 --no-tests=warn
cargo test $SCOPE --all-features --doc

$DIR/docs.sh $1

if [ -z "$1" ]
then
  cargo ws exec cargo rdme --check --intralinks-strip-links
fi
