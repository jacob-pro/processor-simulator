mod cpu_state;
mod instructions;
mod memory;
mod registers;
mod simulators;

#[macro_use]
extern crate maplit;

use crate::simulators::non_pipelined::NonPipelinedSimulator;
use crate::simulators::out_of_order::OutOfOrderSimulator;
use crate::simulators::pipelined::PipelinedSimulator;
use crate::simulators::Simulator;
use anyhow::{anyhow, Context};
use capstone::prelude::*;
use clap::Clap;
use elf::types::PT_LOAD;
use memory::Memory;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::str::FromStr;
use std::time::Instant;

#[derive(FromPrimitive, PartialEq, PartialOrd, Debug)]
pub enum DebugLevel {
    Off = 0,
    Minimal = 1,
    Full = 2,
}

#[derive(Debug)]
pub enum SimulatorType {
    Scalar,
    Pipelined,
    OutOfOrder,
}

impl FromStr for SimulatorType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "scalar" => Ok(Self::Scalar),
            "pipelined" => Ok(Self::Pipelined),
            "outoforder" => Ok(Self::OutOfOrder),
            _ => Err("Couldn't match SimulatorType".to_string()),
        }
    }
}

/*
The top of the stack as defined by the linker script
" .stack 0x80000 : { _stack = .; *(.stack) } "
`readelf -s basic.elf | grep _stack` = 00080000
newlib will deal with the stack pointer automatically
*/
const _STACK: u32 = 0x80000;

#[derive(Clap)]
#[clap(version = "1.0", author = "Jacob Halsey")]
struct Opts {
    #[clap(about = "Choose the name of the program to run")]
    program: PathBuf,
    #[clap(long, about = "Set stack size in bytes", default_value = "4096")]
    stack: u32,
    #[clap(
        short,
        long,
        about = "Level of debug information printed",
        default_value = "0"
    )]
    debug: u32,
    #[clap(short, long, about = "Choose which simulator type")]
    sim: Option<SimulatorType>,
    #[clap(short, long, about = "Specify how many stations / execution units", default_value = "4")]
    units: usize,
}

fn main() -> anyhow::Result<()> {
    let matches = Opts::parse();

    let debug_level: DebugLevel =
        FromPrimitive::from_u32(matches.debug).with_context(|| "Unsupported debug level")?;

    let elf_file = elf::File::open_path(&matches.program)
        .map_err(|e| anyhow!(format!("{:?}", e)))
        .with_context(|| "Reading elf binary")?;

    let mut elf_file_bytes = Vec::new();
    File::open(&matches.program)
        .unwrap()
        .read_to_end(&mut elf_file_bytes)
        .with_context(|| "Reading elf file contents")?;

    let mut memory = Memory::default();
    memory.mmap(
        _STACK - matches.stack,
        vec![0; matches.stack as usize],
        true,
    );

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

    let sim: Box<dyn Simulator> = match matches.sim.unwrap_or(SimulatorType::OutOfOrder) {
        SimulatorType::Scalar => Box::new(NonPipelinedSimulator {}),
        SimulatorType::Pipelined => Box::new(PipelinedSimulator {}),
        SimulatorType::OutOfOrder => Box::new(OutOfOrderSimulator::new(matches.units)),
    };

    println!("Using: {}\n", sim.name());
    let start_time = Instant::now();
    println!("{}", sim.run(memory, entry, &debug_level));
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
