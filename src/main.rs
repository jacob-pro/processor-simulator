mod memory;
mod registers;
mod simulator;

use std::path::PathBuf;
use clap::{App, Arg};
use memory::Memory;
use crate::simulator::Simulator;

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

    let memory = Memory::initialise(text_scn.data.clone(), DEFAULT_STACK_SIZE);
    let mut simulator = Simulator::new(memory);
    simulator.run();
}
