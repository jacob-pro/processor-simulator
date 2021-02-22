use super::{Instruction, ShouldTerminate};
use crate::simulator::Simulator;
use capstone::prelude::*;
use capstone::arch::arm::ArmOperand;
use crate::instructions::util::ArmOperandExt;

pub struct CMP {
    negative: bool,
    first: RegId,
    second: ArmOperand,
}

impl CMP {
    pub fn new(operands: Vec<ArmOperand>, negative: bool) -> Self {
        Self {
            negative,
            first: operands[0].reg_id().unwrap(),
            second: operands[1].clone(),
        }
    }
}

impl Instruction for CMP {
    fn execute(&self, sim: &mut Simulator) -> ShouldTerminate {
        let first_value = sim.registers.read_by_id(self.first);
        let second_value = sim.registers.value_of_flexible_second_operand(&self.second, false);
        let res = if !self.negative {
            let (res, ovf) = first_value.overflowing_sub(second_value);
            sim.registers.cond_flags.c = ovf;
            res as i32
        } else {
            let (res, ovf) = (first_value as i32).overflowing_sub(second_value as i32);
            sim.registers.cond_flags.v = ovf;
            res
        };
        sim.registers.cond_flags.n = res.is_negative();
        sim.registers.cond_flags.z = res == 0;
        false
    }
}
