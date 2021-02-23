# Processor Simulator

Bristol COMS30046 - Advanced Computer Architecture

- Simple Scalar Processor: Friday 12 noon in Week 17 = Friday, March 5th 2021
- Superscalar Processor: Friday 12 noon in Week 22 = Friday, April 30th 2021

## About

This is a partial simulator of the Cortex-M0 CPU - using the ARM Thumb instruction set.

It is capable of running small C programs along with the newlib standard library, 
that are compiled to elf binaries (see [./programs/Makefile](./programs/Makefile)).

I am using the [Capstone](https://github.com/capstone-rust/capstone-rs) framework
to disassemble the ARM instructions into a readable / parsable form.

Example usage:
`cargo run -- programs/test.elf`
