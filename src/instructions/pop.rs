use super::{Instruction, ShouldTerminate};
use crate::instructions::util::ArmOperandExt;
use crate::registers::SP;
use crate::simulator::{ExecuteChanges, Simulator};
use capstone::arch::arm::ArmOperand;
use capstone::prelude::*;

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
    fn execute(&self, sim: &Simulator, changes: &mut ExecuteChanges) -> ShouldTerminate {
        let reg_list = sim.registers.push_pop_register_asc(self.reg_list.clone());
        let mut sp = sim.registers.sp;
        for r in &reg_list {
            let read_from_stack = sim.memory.read().unwrap().read_u32(sp);
            changes.register_change(*r, read_from_stack);
            sp = sp + 4;
        }
        changes.register_change(SP, sp);
        return false;
    }
}
