#!/bin/bash

if ! command -v rustup &>/dev/null; then
  echo 'Downloading rustup...'
  curl https://sh.rustup.rs -sSf | sh -s -- -y
  source $HOME/.cargo/env
fi

cargo run -- programs/test.elf
cargo run -- programs/fibonacci.elf
