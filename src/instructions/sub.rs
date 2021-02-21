use super::{Instruction, ShouldTerminate};
use crate::simulator::Simulator;
use capstone::prelude::*;
use capstone::arch::arm::ArmOperand;
use crate::instructions::util::ArmOperandExt;

pub struct SUB {
    update_flags: bool,
    dest: RegId,
    first: RegId,
    second: ArmOperand,
}

impl SUB {
    pub fn new(operands: Vec<ArmOperand>, update_flags: bool) -> Self {
        if operands.len() == 2 {
            let dest = operands[0].reg_id().unwrap();
            let first = operands[0].reg_id().unwrap();
            let second = operands[1].clone();
            return Self { update_flags, dest, first, second };
        } else {
            let dest = operands[0].reg_id().unwrap();
            let first = operands[1].reg_id().unwrap();
            let second = operands[2].clone();
            return Self { update_flags, dest, first, second };
        }
    }
}

impl Instruction for SUB {
    fn execute(&self, sim: &mut Simulator) -> ShouldTerminate {
        let first_val = *sim.registers.get_by_id(self.first);
        let sec_val = sim.registers.value_of_flexible_second_operand(&self.second, self.update_flags);
        let (result, unsigned_overflow) = first_val.overflowing_sub(sec_val);
        let (_, signed_overflow) = (first_val as i32).overflowing_sub(sec_val as i32);
        *sim.registers.get_by_id(self.dest) = result;
        if self.update_flags {
            sim.registers.cond_flags.n = (result as i32).is_negative();
            sim.registers.cond_flags.z = result == 0;
            sim.registers.cond_flags.c = unsigned_overflow;
            sim.registers.cond_flags.v = signed_overflow;
        }
        false
    }
}
