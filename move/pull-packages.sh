#!/usr/bin/env bash

REPO_ROOT="$(git rev-parse --show-toplevel)"
DIR=$( realpath $( dirname $0 ) )

SUI_REPO=mystenlabs/sui
SUI_TAG=mainnet-v1.64.2
DEEP_REPO=MystenLabs/deepbookv3
DEEP_TAG=v6.0.0


set -ex

cd $DIR

copy_sources() {
  local repo=$1
  local tag=$2
  local sources=${@:3}

  local local_path="$REPO_ROOT/.move/$repo/$tag"

  if [ ! -e $local_path ]
  then
    echo "$local_path doesn't exist; cloning from $repo at $tag"
    git clone https://github.com/${repo}.git --depth 1 --branch $tag $local_path
  fi

  for src in $sources
  do
    local dest=$(basename $src)
    mkdir -p ./$dest
    cp -r $local_path/$src/sources ./$dest/
    cp $local_path/$src/Move.toml ./$dest/
  done
}

# === Sui repo ===

copy_sources $SUI_REPO $SUI_TAG \
  crates/sui-framework/packages/move-stdlib \
  crates/sui-framework/packages/sui-framework

# === Deepbook repo ===

copy_sources $DEEP_REPO $DEEP_TAG \
  packages/deepbook \
  packages/deepbook_margin \
  packages/margin_liquidation
