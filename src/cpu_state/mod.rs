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
    fetched_instruction: Option<Vec<u8>>,
    decoded_instruction: Option<DecodedInstruction>,
    pub should_terminate: ShouldTerminate,
    pub next_instr_addr: u32,
    pub fetched_instr_addr: Option<u32>,
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
            fetched_instr_addr: None,
        }
    }

    pub fn flush_pipeline(&mut self) {
        self.fetched_instruction = None;
        self.decoded_instruction = None;
    }
}

#[derive(Clone)]
struct DecodedInstruction {
    imp: Rc<dyn Instruction>,
    cc: ArmCC,
    string: String,
    length: u32,
    address: u32,
}
