use std::path::PathBuf;
use capstone::prelude::*;
use clap::{App, Arg};

fn example(code: &[u8]) -> CsResult<()> {
    let cs = Capstone::new()
        .arm()
        .mode(arch::arm::ArchMode::Thumb)
        .endian(capstone::Endian::Little)
        .detail(true)
        .build()?;

    let insns = cs.disasm_all(code, 0x0)?;
    println!("Found {} instructions", insns.len());
    for i in insns.iter() {
        println!("{}", i);
    }
    Ok(())
}

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

    if let Err(err) = example(&text_scn.data) {
        println!("Error: {}", err);
    }
}
