#!/bin/bash
set -e

if ! command -v rustup &>/dev/null; then
  echo 'Downloading rustup...'
  curl https://sh.rustup.rs -sSf | sh -s -- -y
  source $HOME/.cargo/env
fi

for sim in scalar pipelined
do
  cargo run --release -- -s ${sim} programs/test1.elf
  cargo run --release -- -s ${sim} programs/test2.elf
  cargo run --release -- -s ${sim} programs/fibonacci.elf
  cargo run --release -- -s ${sim} programs/factorial.elf
done

