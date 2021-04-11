use super::Instruction;
use crate::instructions::util::{arm_op_mem_regs, ArmOperandExt};
use crate::instructions::PollResult;
use crate::station::ReservationStation;
use capstone::arch::arm::{ArmOpMem, ArmOperand};
use capstone::prelude::*;

#[derive(Clone)]
pub enum Mode {
    Word,
    HalfWord,
    Byte,
}

#[derive(Clone)]
pub struct STR {
    reg: RegId,
    mem: ArmOpMem,
    mode: Mode,
}

impl STR {
    pub fn new(operands: Vec<ArmOperand>, mode: Mode) -> Self {
        Self {
            reg: operands[0].reg_id().unwrap(),
            mem: operands[1].op_mem_value().unwrap(),
            mode,
        }
    }
}

impl Instruction for STR {
    fn poll(&self, station: &ReservationStation) -> PollResult {
        let mem_addr = station.eval_ldr_str_op_mem(&self.mem);
        let reg_val = station.read_by_id(self.reg);
        match self.mode {
            Mode::Word => station
                .memory
                .write()
                .unwrap()
                .write_bytes(mem_addr, &reg_val.to_le_bytes()),
            Mode::HalfWord => station
                .memory
                .write()
                .unwrap()
                .write_bytes(mem_addr, &(reg_val as u16).to_le_bytes()),
            Mode::Byte => station
                .memory
                .write()
                .unwrap()
                .write_bytes(mem_addr, &(reg_val as u8).to_le_bytes()),
        };
        PollResult::Complete(vec![])
    }

    fn source_registers(&self) -> Vec<RegId> {
        let mut src = vec![self.reg];
        src.append(&mut arm_op_mem_regs(&self.mem));
        src
    }

    fn dest_registers(&self) -> Vec<RegId> {
        vec![]
    }
}
