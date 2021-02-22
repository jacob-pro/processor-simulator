use super::{Instruction, ShouldTerminate};
use crate::simulator::Simulator;
use capstone::prelude::*;
use capstone::arch::arm::ArmOperand;
use crate::instructions::util::ArmOperandExt;

#[allow(unused)]
pub enum Mode {
    ADC,
    ADD,
    RSB,
    SBC,
    SUB,
}

pub struct ADD {
    update_flags: bool,
    mode: Mode,
    dest: RegId,
    first: RegId,
    second: ArmOperand,
}

impl ADD {
    pub fn new(operands: Vec<ArmOperand>, update_flags: bool, mode: Mode) -> Self {
        // https://stackoverflow.com/a/25577464/7547647
        if operands.len() == 2 {
            let dest = operands[0].reg_id().unwrap();
            let first = operands[0].reg_id().unwrap();
            let second = operands[1].clone();
            return Self { update_flags, mode, dest, first, second };
        } else {
            let dest = operands[0].reg_id().unwrap();
            let first = operands[1].reg_id().unwrap();
            let second = operands[2].clone();
            return Self { update_flags, mode, dest, first, second };
        }
    }
}

impl Instruction for ADD {
    fn execute(&self, sim: &mut Simulator) -> ShouldTerminate {
        let first_val = sim.registers.read_by_id(self.first);
        let sec_val = sim.registers.value_of_flexible_second_operand(&self.second, self.update_flags);

        let (result, unsigned_overflow, signed_overflow) = match self.mode {
            Mode::ADC => {
                let (result_u, unsigned_overflow_1) = first_val.overflowing_add(sec_val);
                let (result_s, signed_overflow_1) = (first_val as i32).overflowing_add(sec_val as i32);
                let carry = if sim.registers.cond_flags.c {
                    1
                } else {
                    0
                } as u8;
                let (result, unsigned_overflow_2) = result_u.overflowing_add(carry as u32);
                let (_, signed_overflow_2) = result_s.overflowing_add(carry as i32);
                let unsigned_overflow = unsigned_overflow_1 || unsigned_overflow_2;
                let signed_overflow = signed_overflow_1 || signed_overflow_2;
                (result, unsigned_overflow, signed_overflow)
            }
            Mode::ADD => {
                let (result, unsigned_overflow) = first_val.overflowing_add(sec_val);
                let (_, signed_overflow) = (first_val as i32).overflowing_add(sec_val as i32);
                (result, unsigned_overflow, signed_overflow)
            }
            Mode::RSB => {
                let (result, unsigned_overflow) = sec_val.overflowing_sub(first_val);
                let (_, signed_overflow) = (sec_val as i32).overflowing_sub(first_val as i32);
                (result, unsigned_overflow, signed_overflow)
            }
            Mode::SBC => {panic!()}
            Mode::SUB => {
                let (result, unsigned_overflow) = first_val.overflowing_sub(sec_val);
                let (_, signed_overflow) = (first_val as i32).overflowing_sub(sec_val as i32);
                (result, unsigned_overflow, signed_overflow)
            }
        };

        sim.registers.write_by_id(self.dest, result);
        if self.update_flags {
            sim.registers.cond_flags.n = (result as i32).is_negative();
            sim.registers.cond_flags.z = result == 0;
            sim.registers.cond_flags.c = unsigned_overflow;
            sim.registers.cond_flags.v = signed_overflow;
        }
        false
    }
}
