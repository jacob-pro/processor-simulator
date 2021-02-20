use super::{Instruction, ShouldTerminate};
use crate::simulator::Simulator;
use capstone::arch::arm::ArmOperand;
use crate::instructions::util::ArmOperandExt;
use capstone::prelude::*;

pub struct LSL {
    dest: RegId,
    first: RegId,
    second: ArmOperand,
}

impl LSL {
    pub fn new(operands: Vec<ArmOperand>) -> Self {
        let dest = operands[0].reg_id().unwrap();
        let first = operands[1].reg_id().unwrap();
        let second = operands[2].clone();
        return Self { dest, first, second };
    }
}

impl Instruction for LSL {
    fn execute(&self, sim: &mut Simulator) -> ShouldTerminate {
        let first_val = *sim.registers.get_by_id(self.first);
        let shift = sim.registers.value_of_flexible_second_operand(&self.second, true) as u8;
        let result = first_val << shift;
        *sim.registers.get_by_id(self.dest) = result;
        sim.registers.cond_flags.n = (result as i32).is_negative();
        sim.registers.cond_flags.z = result == 0;
        //TODO: update c flag?
        false
    }
}
