#!/usr/bin/env bash

REPO_ROOT="$(git rev-parse --show-toplevel)"

cd $REPO_ROOT

rename-crate ()
{
  FROM=$1
  TO=$2

  FROM_SNAKE=$(echo $FROM | sed "s/-/_/g")
  TO_SNAKE=$(echo $TO | sed "s/-/_/g")

  echo from: $FROM_SNAKE
  echo to: $TO_SNAKE


  echo $(fd "$1" $REPO_ROOT/crates)

  cargo ws rename --from $FROM $TO
  mv $REPO_ROOT/crates/$FROM $REPO_ROOT/crates/$TO

  sed -i -E "s/$FROM/$TO/g" $(fd --extension toml)
  sed -i -E "s/$FROM/$TO/g" $(fd --extension rs)
  sed -i -E "s/$FROM_SNAKE/$TO_SNAKE/g" $(fd --extension toml)
  sed -i -E "s/$FROM_SNAKE/$TO_SNAKE/g" $(fd --extension rs)
  # Cleanup
  sed -i -E 's/, package = "[0-9A-Za-z_\-]+"//g' $(fd --extension toml)
}

rename-crate $1 $2
