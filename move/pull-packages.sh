#!/usr/bin/env bash

REPO_ROOT="$(git rev-parse --show-toplevel)"
DIR=$( realpath $( dirname $0 ) )

SUI_TAG=mainnet-v1.51.5
SUI_REPO=mystenlabs/sui

set -e

cd $REPO_ROOT

LOCAL_PATH="$REPO_ROOT/.move/$SUI_REPO/$SUI_TAG"
if [ ! -e $LOCAL_PATH ]
then
  echo "$LOCAL_PATH doesn't exist; cloning from $SUI_REPO at $SUI_TAG"
  git clone https://github.com/${SUI_REPO}.git --depth 1 --branch $SUI_TAG $LOCAL_PATH
fi

mkdir -p ./crates/moverox-sui/move/move-stdlib
cp -r $LOCAL_PATH/crates/sui-framework/packages/move-stdlib/sources ./crates/moverox-sui/move/move-stdlib/
cp -r $LOCAL_PATH/crates/sui-framework/packages/move-stdlib/Move.toml ./crates/moverox-sui/move/move-stdlib/
ln -s ../crates/moverox-sui/move/move-stdlib ./move/move-stdlib

mkdir -p ./crates/moverox-sui/move/sui-framework
cp -r $LOCAL_PATH/crates/sui-framework/packages/sui-framework/sources ./crates/moverox-sui/move/sui-framework
cp -r $LOCAL_PATH/crates/sui-framework/packages/sui-framework/Move.toml ./crates/moverox-sui/move/sui-framework
ln -s ../crates/moverox-sui/move/sui-framework ./move/sui-framework
