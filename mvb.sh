#!/bin/bash
set -e

# Check if inside WSL
if cat /proc/version | grep microsoft; then
  CMD="cmd.exe /c"
else
  CMD=
  # Check if Rust installed
  if ! command -v rustup &>/dev/null; then
    echo 'Downloading rustup...'
    curl https://sh.rustup.rs -sSf | sh -s -- -y
    source $HOME/.cargo/env
  fi
fi

# Run all programs with all simulators
for sim in scalar pipelined outoforder
do
  $CMD cargo run -- -s ${sim} programs/test1.elf
  $CMD cargo run -- -s ${sim} programs/test2.elf
  $CMD cargo run -- -s ${sim} programs/fibonacci.elf
  $CMD cargo run -- -s ${sim} programs/factorial.elf
done

