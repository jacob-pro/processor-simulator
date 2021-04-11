use super::Instruction;
use crate::cpu_state::station::ReservationStation;
use crate::instructions::util::ArmOperandExt;
use crate::instructions::util::RegisterSet;
use crate::instructions::PollResult;
use capstone::arch::arm::{ArmOpMem, ArmOperand};
use capstone::prelude::*;
use std::collections::HashSet;

#[derive(Clone)]
pub enum Mode {
    Word,
    HalfWord,
    Byte,
    SignedHalfWord,
    SignedByte,
}

#[derive(Clone)]
pub struct LDR {
    reg: RegId,
    mem: ArmOpMem,
    mode: Mode,
    waited: bool,
}

impl LDR {
    pub fn new(operands: Vec<ArmOperand>, mode: Mode) -> Self {
        Self {
            reg: operands[0].reg_id().unwrap(),
            mem: operands[1].op_mem_value().unwrap(),
            mode,
            waited: false,
        }
    }
}

impl Instruction for LDR {
    fn poll(&self, station: &ReservationStation) -> PollResult {
        if !self.waited {
            // Takes 2 cycles
            let mut cloned = self.clone();
            cloned.waited = true;
            return PollResult::Again(Box::new(cloned));
        }

        let mem_addr = station.eval_ldr_str_op_mem(&self.mem);
        let val_at_addr = match self.mode {
            Mode::Word => station.memory.read().unwrap().read_u32(mem_addr),
            Mode::HalfWord => station.memory.read().unwrap().read_u16(mem_addr) as u32,
            Mode::Byte => station.memory.read().unwrap().read_byte(mem_addr) as u32,
            Mode::SignedHalfWord => station.memory.read().unwrap().read_u16(mem_addr) as i32 as u32,
            Mode::SignedByte => station.memory.read().unwrap().read_byte(mem_addr) as i32 as u32,
        };
        PollResult::Complete(vec![(self.reg, val_at_addr)])
    }

    fn source_registers(&self) -> HashSet<RegId> {
        self.mem.registers()
    }

    fn dest_registers(&self) -> HashSet<RegId> {
        hashset![self.reg]
    }
}
