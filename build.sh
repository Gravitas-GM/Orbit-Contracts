#!/usr/bin/env bash

project_dir=`dirname $0`
RUST_LOG=${RUST_LOG-info} cargo run --manifest-path "${project_dir}/codegen/Cargo.toml" --quiet -- $@