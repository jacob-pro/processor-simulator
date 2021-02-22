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
        let dest = operands[0].reg_id().unwrap();
        let first = operands[1].reg_id().unwrap();
        let second = operands[2].clone();
        return Self { mode, dest, first, second };
    }
}

impl Instruction for SHIFT {
    fn execute(&self, sim: &mut Simulator) -> ShouldTerminate {
        let first_val = sim.registers.read_by_id(self.first);
        let second = sim.registers.value_of_flexible_second_operand(&self.second, true) as u8;

        // The C flag is unaffected if the shift value is 0. Otherwise, the C flag is updated to the last bit shifted out.
        let result = match self.mode {
            Mode::ASR => {
                if second > 0 {
                    sim.registers.cond_flags.c = get_bit_at(first_val, second - 1);
                }
                (first_val as i32 >> second) as u32
            }
            Mode::LSL => {
                if second > 0 {
                    sim.registers.cond_flags.c = get_bit_at(first_val, 31 - second);
                }
                first_val << second
            }
            Mode::LSR => {
                if second > 0 {
                    sim.registers.cond_flags.c = get_bit_at(first_val, second - 1);
                }
                first_val >> second
            }
            Mode::ROR => {
                if second > 0 {
                    sim.registers.cond_flags.c = get_bit_at(first_val, second - 1);
                }
                first_val.rotate_right(second as u32)
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
