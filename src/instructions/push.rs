use super::{Instruction, ShouldTerminate};
use crate::instructions::util::ArmOperandExt;
use crate::simulator::{Simulator, ExecuteChanges};
use capstone::arch::arm::ArmOperand;
use capstone::prelude::*;
use crate::registers::SP;

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
    fn execute(&self, sim: &Simulator, changes: &mut ExecuteChanges) -> ShouldTerminate {
        let mut reg_list = sim.registers.push_pop_register_asc(self.reg_list.clone());
        reg_list.reverse();
        let mut sp = sim.registers.sp;
        for r in &reg_list {
            sp = sp - 4;
            let register_value = sim.registers.read_by_id(*r).to_le_bytes();
            sim.memory
                .write()
                .unwrap()
                .write_bytes(sim.registers.sp, &register_value);
        }
        changes.register_change(SP, sp);
        return false;
    }
}
