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
    SignedHalfWord,
    SignedByte,
}

#[derive(Clone)]
pub struct LDR {
    reg: RegId,
    mem: ArmOpMem,
    mode: Mode,
}

impl LDR {
    pub fn new(operands: Vec<ArmOperand>, mode: Mode) -> Self {
        Self {
            reg: operands[0].reg_id().unwrap(),
            mem: operands[1].op_mem_value().unwrap(),
            mode,
        }
    }
}

impl Instruction for LDR {
    fn poll(&self, station: &ReservationStation) -> PollResult {
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

    fn source_registers(&self) -> Vec<RegId> {
        arm_op_mem_regs(&self.mem)
    }

    fn dest_registers(&self) -> Vec<RegId> {
        vec![self.reg]
    }
}
