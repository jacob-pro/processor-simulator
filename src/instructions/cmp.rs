use super::{Instruction, ShouldTerminate};
use crate::simulator::Simulator;
use capstone::prelude::*;
use capstone::arch::arm::ArmOperand;
use crate::instructions::util::ArmOperandExt;

pub enum Mode {
    Positive,
    Negative,
}

pub struct CMP {
    mode: Mode,
    first: RegId,
    second: ArmOperand,
}

impl CMP {
    pub fn new(operands: Vec<ArmOperand>, mode: Mode) -> Self {
        Self {
            mode,
            first: operands[0].reg_id().unwrap(),
            second: operands[1].clone(),
        }
    }
}

impl Instruction for CMP {
    fn execute(&self, sim: &mut Simulator) -> ShouldTerminate {
        let first_value = sim.registers.read_by_id(self.first);
        let second_value = sim.registers.value_of_flexible_second_operand(&self.second, false);
        // The CMP instruction subtracts either the value in the register specified by Rm, or the immediate imm from the value in Rn and updates the flags.
        let (pos_res, pos_ovf) = second_value.overflowing_sub(first_value);
        // The CMN instruction adds the value of Rm to the value in Rn and updates the flags.
        let (neg_res, neg_ovf) = (second_value).overflowing_add(first_value);

        let res = match self.mode {
            Mode::Positive => { pos_res }
            Mode::Negative => { neg_res }
        };
        sim.registers.cond_flags.n = (res as i32).is_negative();
        sim.registers.cond_flags.z = res == 0;
        sim.registers.cond_flags.c = pos_ovf;
        sim.registers.cond_flags.v = neg_ovf;
        false
    }
}
