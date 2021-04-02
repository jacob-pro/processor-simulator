pub mod decode;
pub mod execute;
pub mod fetch;

use crate::instructions::{Instruction, ShouldTerminate};
use crate::memory::Memory;
use crate::registers::RegisterFile;
use capstone::arch::arm::ArmCC;
use std::rc::Rc;
use std::sync::{Arc, RwLock};

pub struct CpuState {
    pub memory: Arc<RwLock<Memory>>,
    pub registers: RegisterFile,
    pub next_instr_addr: u32, // Address of instruction waiting to be fetched
    pub fetched_instruction: Option<FetchedInstruction>, // Instruction waiting to be decoded
    pub decoded_instruction: Option<DecodedInstruction>, // Instruction waiting to be executed
    pub should_terminate: ShouldTerminate,
}

impl CpuState {
    pub fn new(memory: Arc<RwLock<Memory>>, entry: u32) -> Self {
        let registers = RegisterFile::new();
        Self {
            memory,
            registers,
            fetched_instruction: None,
            decoded_instruction: None,
            should_terminate: false,
            next_instr_addr: entry,
        }
    }

    pub fn flush_pipeline(&mut self) {
        self.fetched_instruction = None;
        self.decoded_instruction = None;
    }
}

pub struct DecodedInstruction {
    pub imp: Rc<dyn Instruction>,
    pub cc: ArmCC,
    pub string: String,
    pub length: u32,
    pub address: u32,
}

pub struct FetchedInstruction {
    pub bytes: Vec<u8>,
    pub address: u32,
}
