mod memory;
mod registers;
mod simulator;
mod instructions;

use std::path::PathBuf;
use clap::{App, Arg};
use memory::Memory;
use crate::simulator::Simulator;
use elf::types::PT_LOAD;

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
        .arg(Arg::with_name("print")
            .value_name("print")
            .short("p")
            .long("print")
            .help("Prints instructions as they are executed")
            .takes_value(false))
        .get_matches();

    let path = PathBuf::from(matches.value_of("program").unwrap());
    let elf_file = match elf::File::open_path(&path) {
        Ok(f) => f,
        Err(e) => panic!("Error opening file: {:#?}", e),
    };
    let text_scn = match elf_file.get_section(".text") {
        Some(s) => s,
        None => panic!("Failed to get .text section in elf file"),
    };

    let load = elf_file.phdrs.iter()
        .filter(|x| x.progtype == PT_LOAD).next().expect("Couldn't find LOAD");
    let mut prog =  vec![0; load.filesz as usize];
    let base = text_scn.shdr.offset as usize;
    prog[base..base + text_scn.data.len()].copy_from_slice(text_scn.data.as_slice());

    let entry = elf_file.ehdr.entry as u32 - 1;
    let actual_entry = entry - load.vaddr as u32;

    let memory = Memory::initialise(prog, DEFAULT_STACK_SIZE);
    let mut simulator = Simulator::new(memory, actual_entry);
    simulator.run(matches.is_present("print"));
}
