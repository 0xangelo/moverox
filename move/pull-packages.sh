#!/usr/bin/env bash

REPO_ROOT="$(git rev-parse --show-toplevel)"
DIR=$( realpath $( dirname $0 ) )

SUI_TAG=mainnet-v1.51.5
SUI_REPO=mystenlabs/sui

set -e

cd $DIR

LOCAL_PATH="$REPO_ROOT/.move/$SUI_REPO/$SUI_TAG"
if [ ! -e $LOCAL_PATH ]
then
  echo "$LOCAL_PATH doesn't exist; cloning from $SUI_REPO at $SUI_TAG"
  git clone https://github.com/${SUI_REPO}.git --depth 1 --branch $SUI_TAG $LOCAL_PATH
fi

mkdir -p ./move-stdlib
cp -r $LOCAL_PATH/crates/sui-framework/packages/move-stdlib/sources ./move-stdlib/
cp -r $LOCAL_PATH/crates/sui-framework/packages/move-stdlib/Move.toml ./move-stdlib/
mkdir -p ./sui-framework
cp -r $LOCAL_PATH/crates/sui-framework/packages/sui-framework/sources ./sui-framework/
cp -r $LOCAL_PATH/crates/sui-framework/packages/sui-framework/Move.toml ./sui-framework/
