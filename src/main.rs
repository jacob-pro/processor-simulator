mod memory;
mod registers;
mod simulator;
mod instructions;

use std::path::PathBuf;
use clap::{App, Arg};
use memory::Memory;
use crate::simulator::Simulator;
use elf::types::PT_LOAD;
use std::fs::File;
use std::io::Read;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

#[derive(FromPrimitive, PartialEq, PartialOrd)]
pub enum DebugLevel {
    Off = 0,
    Minimal = 1,
    Full = 2,
}

/*
 The top of the stack as defined by the linker script
 " .stack 0x80000 : { _stack = .; *(.stack) } "
 `readelf -s basic.elf | grep _stack` = 00080000
 newlib will deal with the stack pointer automatically
 */
const _STACK: u32 = 0x80000;
const DEFAULT_STACK_SIZE: u32 = 4096;

fn main() {

    let default_stack_size = DEFAULT_STACK_SIZE.to_string();
    let matches = App::new("Processor Simulator")
        .version("1.0")
        .author("Jacob Halsey")
        .arg(Arg::with_name("program")
            .value_name("FILE")
            .help("Choose the name of the program to run")
            .required(true)
            .takes_value(true))
        .arg(Arg::with_name("debug")
            .value_name("debug")
            .short("d")
            .long("debug")
            .help("Level of debug information")
            .default_value("0")
            .takes_value(true))
        .arg(Arg::with_name("stack")
            .value_name("stack")
            .long("stack")
            .help("Set stack size in bytes")
            .takes_value(true)
            .default_value(default_stack_size.as_str()))
        .get_matches();

    let debug_level: DebugLevel = FromPrimitive::from_u32(matches.value_of("debug")
        .unwrap().parse::<u32>()
        .expect("--debug must be integer")).expect("Invalid debug level");
    let stack_size: u32 = matches.value_of("stack").unwrap().parse()
        .expect("--stack must be an integer");

    let path = PathBuf::from(matches.value_of("program").unwrap());
    let elf_file = match elf::File::open_path(&path) {
        Ok(f) => f,
        Err(e) => panic!("Error opening file: {:#?}", e),
    };
    let mut elf_file_bytes = Vec::new();
    File::open(&path).unwrap().read_to_end(&mut elf_file_bytes).unwrap();

    let mut memory = Memory::default();
    memory.mmap(_STACK - stack_size,  vec![0; stack_size as usize], true);

    // https://wiki.osdev.org/ELF#Loading_ELF_Binaries
    for header in elf_file.phdrs.iter() {
        if header.progtype == PT_LOAD {

            let mut data =  vec![0; header.memsz as usize];
            let elf_offset = header.offset as usize;
            let end = elf_offset + header.filesz as usize;

            data[0..header.filesz as usize]
                .copy_from_slice(&elf_file_bytes[elf_offset..end]);

            let write = (header.flags.0 & 0b10) > 0;
            memory.mmap(header.vaddr as u32, data, write);
        }
    }

    let entry = elf_file.ehdr.entry as u32;
    if debug_level >= DebugLevel::Minimal {
        //println!("Entry point at {:#X}", entry - 1);
    }

    let mut simulator = Simulator::new(memory, entry);
    simulator.run(debug_level);
}
