use super::Instruction;
use crate::cpu_state::execute::ExecuteChanges;
use crate::cpu_state::CpuState;
use crate::instructions::util::ArmOperandExt;
use crate::instructions::NextInstructionState;
use crate::registers::ids::{PC, SP};
use capstone::arch::arm::ArmOperand;
use capstone::prelude::*;

#[derive(Clone)]
pub struct POP {
    reg_list: Vec<RegId>,
}

impl POP {
    pub fn new(operands: Vec<ArmOperand>) -> Self {
        let reg_list: Vec<RegId> = operands
            .into_iter()
            .map(|x: ArmOperand| x.reg_id().unwrap())
            .collect();
        Self { reg_list }
    }
}

impl Instruction for POP {
    fn poll(&self, state: &CpuState, changes: &mut ExecuteChanges) -> NextInstructionState {
        let reg_list = state.registers.push_pop_register_asc(self.reg_list.clone());
        let mut sp = state.registers.read_by_id(SP);
        for r in &reg_list {
            let read_from_stack = state.memory.read().unwrap().read_u32(sp);
            changes.register_change(*r, read_from_stack);
            sp = sp + 4;
        }
        changes.register_change(SP, sp);
        None
    }

    fn is_branch(&self) -> bool {
        self.reg_list.iter().find(|r| **r == PC).is_some()
    }
}
