use super::{Instruction, ShouldTerminate};
use crate::simulator::Simulator;
use capstone::arch::arm::{ArmOperand, ArmOpMem};
use crate::instructions::util::ArmOperandExt;
use capstone::prelude::*;

pub enum Mode {
    Word,
    HalfWord,
    Byte,
}

pub struct STR {
    reg: RegId,
    mem: ArmOpMem,
    mode: Mode,
}

impl STR {
    pub fn new(operands: Vec<ArmOperand>, mode: Mode) -> Self {
        Self { reg: operands[0].reg_id().unwrap(), mem: operands[1].op_mem_value().unwrap(), mode }
    }
}

impl Instruction for STR {
    fn execute(&self, sim: &mut Simulator) -> ShouldTerminate {
        let mem_addr = sim.registers.eval_op_mem(&self.mem);
        let reg_val = *sim.registers.get_by_id(self.reg);
        let reg_val = match self.mode {
            Mode::Word => {reg_val}
            Mode::HalfWord => {reg_val as u16 as u32}
            Mode::Byte => {reg_val as u8 as u32}
        };
        sim.memory.write_bytes(mem_addr, &reg_val.to_le_bytes());
        false
    }
}
