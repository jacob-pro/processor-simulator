use super::Instruction;
use crate::cpu_state::station::ReservationStation;
use crate::instructions::util::ArmOperandExt;
use crate::instructions::util::RegisterSet;
use crate::instructions::PollResult;
use capstone::arch::arm::{ArmOpMem, ArmOperand};
use capstone::prelude::*;
use std::collections::HashSet;

#[derive(Clone, Debug)]
pub enum Mode {
    Word,
    HalfWord,
    Byte,
}

#[derive(Clone, Debug)]
pub struct STR {
    reg: RegId,
    mem: ArmOpMem,
    mode: Mode,
    waited: bool,
}

impl STR {
    pub fn new(operands: Vec<ArmOperand>, mode: Mode) -> Self {
        Self {
            reg: operands[0].reg_id().unwrap(),
            mem: operands[1].op_mem_value().unwrap(),
            mode,
            waited: false,
        }
    }
}

impl Instruction for STR {
    fn poll(&self, station: &ReservationStation) -> PollResult {
        if !self.waited {
            // Takes 2 cycles
            let mut cloned = self.clone();
            cloned.waited = true;
            return PollResult::Again(Box::new(cloned));
        }

        let mem_addr = station.eval_ldr_str_op_mem(&self.mem);
        let reg_val = station.read_by_id(self.reg);
        match self.mode {
            Mode::Word => station
                .memory
                .write()
                .unwrap()
                .write_bytes(mem_addr, &reg_val.to_le_bytes())
                .unwrap(),
            Mode::HalfWord => station
                .memory
                .write()
                .unwrap()
                .write_bytes(mem_addr, &(reg_val as u16).to_le_bytes())
                .unwrap(),
            Mode::Byte => station
                .memory
                .write()
                .unwrap()
                .write_bytes(mem_addr, &(reg_val as u8).to_le_bytes())
                .unwrap(),
        };
        PollResult::Complete(vec![])
    }

    fn source_registers(&self) -> HashSet<RegId> {
        let mut src = self.mem.registers();
        src.insert(self.reg);
        src
    }

    fn dest_registers(&self) -> HashSet<RegId> {
        hashset![]
    }

    fn hazardous(&self) -> bool {
        true
    }
}
