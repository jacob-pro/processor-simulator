use super::Instruction;
use crate::cpu_state::execute::ExecuteChanges;
use crate::cpu_state::CpuState;
use crate::instructions::util::ArmOperandExt;
use crate::instructions::ExecutionComplete;
use crate::registers::ids::SP;
use capstone::arch::arm::ArmOperand;
use capstone::prelude::*;

pub struct PUSH {
    reg_list: Vec<RegId>,
}

impl PUSH {
    pub fn new(operands: Vec<ArmOperand>) -> Self {
        let reg_list: Vec<RegId> = operands
            .into_iter()
            .map(|x: ArmOperand| x.reg_id().unwrap())
            .collect();
        Self { reg_list }
    }
}

impl Instruction for PUSH {
    fn poll(&self, state: &CpuState, changes: &mut ExecuteChanges) -> ExecutionComplete {
        let mut reg_list = state.registers.push_pop_register_asc(self.reg_list.clone());
        reg_list.reverse();
        let mut sp = state.registers.read_by_id(SP);
        for r in &reg_list {
            sp = sp - 4;
            let register_value = state.registers.read_by_id(*r).to_le_bytes();
            state
                .memory
                .write()
                .unwrap()
                .write_bytes(sp, &register_value);
        }
        changes.register_change(SP, sp);
        true
    }
}
