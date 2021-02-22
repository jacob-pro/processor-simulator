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

const DEFAULT_STACK_SIZE: u32 = 1024;

fn main() {

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
            .help("Prints debug information")
            .takes_value(false))
        .get_matches();
    let debug = matches.is_present("debug");

    let path = PathBuf::from(matches.value_of("program").unwrap());
    let elf_file = match elf::File::open_path(&path) {
        Ok(f) => f,
        Err(e) => panic!("Error opening file: {:#?}", e),
    };
    let mut elf_file_bytes = Vec::new();
    File::open(&path).unwrap().read_to_end(&mut elf_file_bytes).unwrap();

    let mut memory = Memory::new(DEFAULT_STACK_SIZE);

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
    if debug {
        println!("Entry point at {:#X}", entry - 1);
    }

    let mut simulator = Simulator::new(memory, entry);
    simulator.run(debug);
}
