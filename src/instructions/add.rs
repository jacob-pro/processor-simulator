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

        // NOTE! ARM uses an inverted carry flag for borrow (i.e. subtraction)

        let (result, carry, overflow) = match self.mode {
            Mode::ADC => {
                let (result_u, carry1) = first_val.overflowing_add(sec_val);
                let (result_s, overflow1) = (first_val as i32).overflowing_add(sec_val as i32);

                let carry = sim.registers.cond_flags.c as u8;
                let (result, carry2) = result_u.overflowing_add(carry as u32);
                let (_, overflow2) = result_s.overflowing_add(carry as i32);

                (result, carry1 || carry2, overflow1 | overflow2)
            }
            Mode::ADD => {
                let (result, carry) = first_val.overflowing_add(sec_val);
                let (_, overflow) = (first_val as i32).overflowing_add(sec_val as i32);
                (result, carry, overflow)
            }
            Mode::RSB => {
                let (result, carry) = sec_val.overflowing_sub(first_val);
                let (_, overflow) = (sec_val as i32).overflowing_sub(first_val as i32);
                (result, !carry, overflow)
            }
            Mode::SBC => {
                let (result_u, carry1) = first_val.overflowing_sub(sec_val);
                let (result_s, overflow1) = (first_val as i32).overflowing_sub(sec_val as i32);

                // If the carry flag is clear, the result is reduced by one.
                let carry = !sim.registers.cond_flags.c as u8;
                let (result, carry2) = result_u.overflowing_sub(carry as u32);
                let (_, overflow2) = result_s.overflowing_sub(carry as i32);

                (result, !(carry1 || carry2), overflow1 || overflow2)
            }
            Mode::SUB => {
                let (result, carry) = first_val.overflowing_sub(sec_val);
                let (_, overflow) = (first_val as i32).overflowing_sub(sec_val as i32);
                (result, !carry, overflow)
            }
        };

        sim.registers.write_by_id(self.dest, result);
        if self.update_flags {
            sim.registers.cond_flags.n = (result as i32).is_negative();
            sim.registers.cond_flags.z = result == 0;
            sim.registers.cond_flags.c = carry;
            sim.registers.cond_flags.v = overflow;
        }
        false
    }
}
