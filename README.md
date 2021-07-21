# Processor Simulator

[![Build status](https://github.com/jacob-pro/processor-simulator/actions/workflows/rust.yml/badge.svg)](https://github.com/jacob-pro/processor-simulator/actions)

Bristol COMS30046 - Advanced Computer Architecture

Confirmed mark: 75

## About

This is a partial simulator of the Cortex-M0 CPU - using the ARM Thumb instruction set.

It is capable of running small C programs along with the newlib standard library, 
that are compiled to elf binaries (see [./programs/Makefile](./programs/Makefile)).

I am using the [Capstone](https://github.com/capstone-rust/capstone-rs) framework
to disassemble the ARM instructions.

## Usage

Run `./mvb.sh` to compile and run all the example programs with each simulator type.

```
USAGE:
    simulator.exe [OPTIONS] <program>

ARGS:
    <program>    Choose the name of the program to run

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --debug <debug>    Level of debug information printed [default: 0]
    -s, --sim <sim>        Choose which simulator type [scalar, pipelined, outoforder]
        --stack <stack>    Set stack size in bytes [default: 4096]
    -u, --units <units>    Specify how many stations / execution units [default: 4]
```

