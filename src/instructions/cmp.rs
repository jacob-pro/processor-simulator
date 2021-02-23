use super::{Instruction, ShouldTerminate};
use crate::simulator::Simulator;
use capstone::prelude::*;
use capstone::arch::arm::ArmOperand;
use crate::instructions::util::ArmOperandExt;

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
        let sec_val = sim.registers.value_of_flexible_second_operand(&self.second, false);

        let (result, unsigned_overflow, signed_overflow) = match self.mode {
            Mode::CMN => {
                let (result, unsigned_overflow) = first_val.overflowing_add(sec_val);
                let (_, signed_overflow) = (first_val as i32).overflowing_add(sec_val as i32);
                (result, unsigned_overflow, signed_overflow)
            }
            Mode::CMP => {
                let (result, unsigned_overflow) = first_val.overflowing_sub(sec_val);
                let (_, signed_overflow) = (first_val as i32).overflowing_sub(sec_val as i32);
                (result, unsigned_overflow, signed_overflow)
            }
        };

        sim.registers.cond_flags.n = (result as i32).is_negative();
        sim.registers.cond_flags.z = result == 0;
        sim.registers.cond_flags.c = unsigned_overflow;
        sim.registers.cond_flags.v = signed_overflow;
        false
    }
}
