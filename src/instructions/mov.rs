use super::{Instruction, ShouldTerminate};
use crate::simulator::Simulator;
use capstone::prelude::*;
use capstone::arch::arm::ArmOperand;
use crate::instructions::util::ArmOperandExt;

pub struct MOV {
    update_flags: bool,
    dest: RegId,
    src: ArmOperand,
}

impl MOV {
    pub fn new(operands: Vec<ArmOperand>, update_flags: bool) -> Self {
        let dest = operands[0].reg_id().unwrap();
        Self { update_flags, dest, src: operands[1].clone() }
    }
}

impl Instruction for MOV {
    fn execute(&self, sim: &mut Simulator) -> ShouldTerminate {
        let val= sim.registers.value_of_flexible_second_operand(&self.src);
        *sim.registers.get_by_id(self.dest) = val;
        false
    }
}
