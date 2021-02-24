use super::{Instruction, ShouldTerminate};
use crate::simulator::Simulator;
use capstone::arch::arm::ArmOperand;
use crate::instructions::util::ArmOperandExt;
use capstone::prelude::*;

pub enum Mode {
    ASR,
    LSL,
    LSR,
    ROR,
}

pub struct SHIFT {
    mode: Mode,
    dest: RegId,
    first: RegId,
    second: ArmOperand,
}

impl SHIFT {
    pub fn new(operands: Vec<ArmOperand>, mode: Mode) -> Self {
        if operands.len() == 2 {
            let dest = operands[0].reg_id().unwrap();
            let first = operands[0].reg_id().unwrap();
            let second = operands[1].clone();
            return Self { mode, dest, first, second };
        } else {
            let dest = operands[0].reg_id().unwrap();
            let first = operands[1].reg_id().unwrap();
            let second = operands[2].clone();
            return Self { mode, dest, first, second };
        }
    }
}

// https://developer.arm.com/documentation/dui0497/a/the-cortex-m0-instruction-set/about-the-instruction-descriptions/shift-operations?lang=en
impl Instruction for SHIFT {
    fn execute(&self, sim: &mut Simulator) -> ShouldTerminate {
        let value = sim.registers.read_by_id(self.first);
        let n = sim.registers.value_of_flexible_second_operand(&self.second, true) as u8;

        // The C flag is unaffected if the shift value is 0. Otherwise, the C flag is updated to the last bit shifted out.
        let result = match self.mode {
            Mode::ASR => {
                assert!(n >= 1 && n <= 32);
                sim.registers.cond_flags.c = get_bit_at(value, n - 1);
                (value as i32 >> n) as u32
            }
            Mode::LSL => {
                assert!(n <= 31);
                if n > 0 {
                    sim.registers.cond_flags.c = get_bit_at(value, 32 - n);
                }
                value << n
            }
            Mode::LSR => {
                sim.registers.cond_flags.c = if n <= 32 {
                     get_bit_at(value, n - 1)
                } else {
                    false   // If n is 33 or more and the carry flag is updated, it is updated to 0.
                };
                value.checked_shr(n as u32).unwrap_or(0)    // If n is 32 or more, then all the bits in the result are cleared to 0.
            }
            Mode::ROR => {
                assert!(n >= 1 && n <= 31);
                sim.registers.cond_flags.c = get_bit_at(value, n - 1);
                value.rotate_right(n as u32)
            }
        };
        sim.registers.write_by_id(self.dest, result);
        sim.registers.cond_flags.n = (result as i32).is_negative();
        sim.registers.cond_flags.z = result == 0;
        false
    }
}

/// gets the bit at position `n`. Bits are numbered from 0 (least significant) to 31 (most significant).
fn get_bit_at(input: u32, n: u8) -> bool {
    input & (1 << n) != 0
}
