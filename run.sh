#!/usr/bin/env bash
set -eEuo pipefail
IFS=$'\n\t'

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"


cd $SCRIPT_DIR/plugin
cargo build
cd $SCRIPT_DIR/host
cargo run -- $SCRIPT_DIR/plugin/target/wasm32-unknown-unknown/debug/plugin.wasm
