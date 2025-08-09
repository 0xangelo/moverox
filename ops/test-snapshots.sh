#!/usr/bin/env bash

DIR="$(realpath $(dirname $0))"
cd $DIR

REPO_ROOT="$(git rev-parse --show-toplevel)"
cd $REPO_ROOT

set -ex

cargo insta test --review --unreferenced=delete -j 4 --all-features --lib --bins --tests
