use super::Instruction;
use crate::cpu_state::execute::ExecuteChanges;
use crate::cpu_state::CpuState;
use crate::instructions::util::ArmOperandExt;
use crate::instructions::NextInstructionState;
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
    fn poll(&self, state: &CpuState, changes: &mut ExecuteChanges) -> NextInstructionState {
        let mem_addr = state.registers.eval_ldr_str_op_mem(&self.mem);
        let val_at_addr = match self.mode {
            Mode::Word => state.memory.read().unwrap().read_u32(mem_addr),
            Mode::HalfWord => state.memory.read().unwrap().read_u16(mem_addr) as u32,
            Mode::Byte => state.memory.read().unwrap().read_byte(mem_addr) as u32,
            Mode::SignedHalfWord => state.memory.read().unwrap().read_u16(mem_addr) as i32 as u32,
            Mode::SignedByte => state.memory.read().unwrap().read_byte(mem_addr) as i32 as u32,
        };
        changes.register_change(self.reg, val_at_addr);
        None
    }
}
