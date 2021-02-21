use super::{Instruction, ShouldTerminate};
use crate::simulator::Simulator;
use capstone::prelude::*;
use capstone::arch::arm::ArmOperand;
use crate::instructions::util::ArmOperandExt;

pub struct TST {
    first: RegId,
    second: ArmOperand,
}

impl TST {
    pub fn new(operands: Vec<ArmOperand>) -> Self {
        let first = operands[0].reg_id().unwrap();
        let second = operands[1].clone();
        return Self { first, second };
    }
}

impl Instruction for TST {
    fn execute(&self, sim: &mut Simulator) -> ShouldTerminate {
        let first_val = *sim.registers.get_by_id(self.first);
        let sec_val = sim.registers.value_of_flexible_second_operand(&self.second, true);
        let result = first_val & sec_val;
        sim.registers.cond_flags.n = (result as i32).is_negative();
        sim.registers.cond_flags.z = result == 0;
        false
    }
}
