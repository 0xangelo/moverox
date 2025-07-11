#!/usr/bin/env bash
# Use ./rdme.sh to sync all crates' README files with their crate-level docs.

DIR=$(realpath $(dirname $0))
# Go to repo root
cd $(git rev-parse --show-toplevel)
cargo ws exec $DIR/rdme.sh
