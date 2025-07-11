#!/usr/bin/env bash
# Generate README from a crate's top-level doc
# Tip: apply this to all crates using
# cargo ws exec $(realpath dev/rdme.sh)

pwd
cargo rdme --force --intralinks-strip-links
