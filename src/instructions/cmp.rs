use super::{Instruction, ShouldTerminate};
use crate::instructions::util::ArmOperandExt;
use crate::simulator::Simulator;
use capstone::arch::arm::ArmOperand;
use capstone::prelude::*;

pub enum Mode {
    CMP,
    CMN,
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
        let first_val = sim.registers.read_by_id(self.first);
        let sec_val = sim
            .registers
            .value_of_flexible_second_operand(&self.second, false);

        let (result, carry, overflow) = match self.mode {
            Mode::CMN => {
                // Same as ADD
                let (result, carry) = first_val.overflowing_add(sec_val);
                let (_, overflow) = (first_val as i32).overflowing_add(sec_val as i32);
                (result, carry, overflow)
            }
            Mode::CMP => {
                // Same as SUB
                let (result, carry) = first_val.overflowing_sub(sec_val);
                let (_, overflow) = (first_val as i32).overflowing_sub(sec_val as i32);
                (result, !carry, overflow)
            }
        };

        sim.registers.cond_flags.n = (result as i32).is_negative();
        sim.registers.cond_flags.z = result == 0;
        sim.registers.cond_flags.c = carry;
        sim.registers.cond_flags.v = overflow;
        false
    }
}
