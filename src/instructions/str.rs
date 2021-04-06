use super::Instruction;
use crate::cpu_state::execute::ExecuteChanges;
use crate::cpu_state::CpuState;
use crate::instructions::util::ArmOperandExt;
use capstone::arch::arm::{ArmOpMem, ArmOperand};
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
        Self {
            reg: operands[0].reg_id().unwrap(),
            mem: operands[1].op_mem_value().unwrap(),
            mode,
        }
    }
}

impl Instruction for STR {
    fn execute(&self, sim: &CpuState, _changes: &mut ExecuteChanges) {
        let mem_addr = sim.registers.eval_ldr_str_op_mem(&self.mem);
        let reg_val = sim.registers.read_by_id(self.reg);
        match self.mode {
            Mode::Word => sim
                .memory
                .write()
                .unwrap()
                .write_bytes(mem_addr, &reg_val.to_le_bytes()),
            Mode::HalfWord => sim
                .memory
                .write()
                .unwrap()
                .write_bytes(mem_addr, &(reg_val as u16).to_le_bytes()),
            Mode::Byte => sim
                .memory
                .write()
                .unwrap()
                .write_bytes(mem_addr, &(reg_val as u8).to_le_bytes()),
        };
    }
}
