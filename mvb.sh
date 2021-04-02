#!/bin/bash
set -e

if ! command -v rustup &>/dev/null; then
  echo 'Downloading rustup...'
  curl https://sh.rustup.rs -sSf | sh -s -- -y
  source $HOME/.cargo/env
fi

cargo run --release -- programs/test.elf
cargo run --release -- programs/fibonacci.elf
cargo run --release -- programs/factorial.elf
