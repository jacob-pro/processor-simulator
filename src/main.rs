mod cpu_state;
mod instructions;
mod memory;
mod registers;
mod simulators;

use crate::cpu_state::CpuState;
use crate::simulators::{NonPipelinedSimulator, PipelinedSimulator, Simulator};
use anyhow::{anyhow, Context};
use capstone::prelude::*;
use clap::{App, Arg};
use elf::types::PT_LOAD;
use memory::Memory;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};
use std::time::Instant;

#[derive(FromPrimitive, PartialEq, PartialOrd, Debug)]
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

fn main() -> anyhow::Result<()> {
    let default_stack_size = DEFAULT_STACK_SIZE.to_string();
    let matches = App::new("Processor Simulator")
        .version("1.0")
        .author("Jacob Halsey")
        .arg(
            Arg::with_name("program")
                .value_name("FILE")
                .help("Choose the name of the program to run")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("debug")
                .value_name("debug")
                .short("d")
                .long("debug")
                .help("Level of debug information")
                .default_value("0")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("stack")
                .value_name("stack")
                .long("stack")
                .help("Set stack size in bytes")
                .takes_value(true)
                .default_value(default_stack_size.as_str()),
        )
        .arg(
            Arg::with_name("no-pipeline")
                .value_name("no-pipeline")
                .long("no-pipeline")
                .takes_value(false),
        )
        .get_matches();

    let debug_level: DebugLevel = FromPrimitive::from_u32(
        matches
            .value_of("debug")
            .unwrap()
            .parse::<u32>()
            .with_context(|| "--debug must be an integer")?,
    )
    .with_context(|| "Unsupported debug level")?;
    let stack_size: u32 = matches
        .value_of("stack")
        .unwrap()
        .parse()
        .with_context(|| "--stack must be an integer")?;

    let path = PathBuf::from(matches.value_of("program").unwrap());
    let elf_file = elf::File::open_path(&path)
        .map_err(|e| anyhow!(format!("{:?}", e)))
        .with_context(|| "Reading elf binary")?;

    let mut elf_file_bytes = Vec::new();
    File::open(&path)
        .unwrap()
        .read_to_end(&mut elf_file_bytes)
        .with_context(|| "Reading elf file contents")?;

    let mut memory = Memory::default();
    memory.mmap(_STACK - stack_size, vec![0; stack_size as usize], true);

    // https://wiki.osdev.org/ELF#Loading_ELF_Binaries
    for header in elf_file.phdrs.iter() {
        if header.progtype == PT_LOAD {
            let mut data = vec![0; header.memsz as usize];
            let elf_offset = header.offset as usize;
            let end = elf_offset + header.filesz as usize;

            data[0..header.filesz as usize].copy_from_slice(&elf_file_bytes[elf_offset..end]);

            let write = (header.flags.0 & 0b10) > 0;
            memory.mmap(header.vaddr as u32, data, write);
        }
    }

    let entry = elf_file.ehdr.entry as u32;
    if debug_level >= DebugLevel::Minimal {
        println!("DEBUG MODE: {:?}", debug_level);
        println!("Entry point at {:#X}", entry & 0xFFFFFFFE);
    }

    let memory = Arc::new(RwLock::new(memory));
    let state = CpuState::new(memory, entry);

    let sim: Box<dyn Simulator> = if matches.is_present("no-pipeline") {
        Box::new(NonPipelinedSimulator {})
    } else {
        Box::new(PipelinedSimulator {})
    };

    println!("Starting {}...\n", sim.name());
    let start_time = Instant::now();
    sim.run(state, &debug_level);
    println!(
        "Simulator ran for {} seconds",
        start_time.elapsed().as_millis() as f64 / 1000.0
    );
    Ok(())
}

thread_local! {
    pub static CAPSTONE: Capstone = Capstone::new()
                .arm()
                .mode(arch::arm::ArchMode::Thumb)
                .endian(capstone::Endian::Little)
                .detail(true)
                .build()
                .unwrap()
}
