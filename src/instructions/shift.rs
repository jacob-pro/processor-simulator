use super::Instruction;
use crate::cpu_state::station::ReservationStation;
use crate::instructions::util::ArmOperandExt;
use crate::instructions::PollResult;
use crate::registers::ids::CPSR;
use crate::registers::ConditionFlag;
use capstone::arch::arm::{ArmOperand, ArmOperandType};
use capstone::prelude::*;
use std::collections::HashSet;

#[derive(Clone)]
pub enum Mode {
    ASR,
    LSL,
    LSR,
    ROR,
}

#[derive(Clone)]
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
            return Self {
                mode,
                dest,
                first,
                second,
            };
        } else {
            let dest = operands[0].reg_id().unwrap();
            let first = operands[1].reg_id().unwrap();
            let second = operands[2].clone();
            return Self {
                mode,
                dest,
                first,
                second,
            };
        }
    }
}

// https://developer.arm.com/documentation/dui0497/a/the-cortex-m0-instruction-set/about-the-instruction-descriptions/shift-operations?lang=en
impl Instruction for SHIFT {
    fn poll(&self, station: &ReservationStation) -> PollResult {
        let value = station.read_by_id(self.first);
        let n = station.value_of_flexible_second_operand(&self.second) as u8;
        let mut cpsr = station.read_by_id(CPSR);

        // The C flag is unaffected if the shift value is 0. Otherwise, the C flag is updated to the last bit shifted out.
        let result = match self.mode {
            Mode::ASR => {
                assert!(n >= 1 && n <= 32);
                ConditionFlag::C.write_flag(&mut cpsr, get_bit_at(value, n - 1));
                (value as i32 >> n) as u32
            }
            Mode::LSL => {
                if n > 0 {
                    // These instructions do not affect the carry flag when used with LSL #0
                    if n >= 33 {
                        ConditionFlag::C.write_flag(&mut cpsr, false); // If n is 33 it is updated to 0.
                    } else {
                        ConditionFlag::C.write_flag(&mut cpsr, get_bit_at(value, 32 - n));
                        // carry flag is updated to the last bit shifted out, bit[32-n]
                    }
                }
                value.checked_shl(n as u32).unwrap_or(0) // If n is 32 or more, then all the bits in the result are cleared to 0.
            }
            Mode::LSR => {
                ConditionFlag::C.write_flag(
                    &mut cpsr,
                    if n <= 32 {
                        get_bit_at(value, n - 1)
                    } else {
                        false // If n is 33 or more and the carry flag is updated, it is updated to 0.
                    },
                );
                value.checked_shr(n as u32).unwrap_or(0) // If n is 32 or more, then all the bits in the result are cleared to 0.
            }
            Mode::ROR => {
                assert!(n >= 1 && n <= 31);
                ConditionFlag::C.write_flag(&mut cpsr, get_bit_at(value, n - 1));
                value.rotate_right(n as u32)
            }
        };
        let mut changes = vec![(self.dest, result)];
        ConditionFlag::N.write_flag(&mut cpsr, (result as i32).is_negative());
        ConditionFlag::Z.write_flag(&mut cpsr, result == 0);
        changes.push((CPSR, cpsr));
        PollResult::Complete(changes)
    }

    fn source_registers(&self) -> HashSet<RegId> {
        let mut set = hashset![self.first, CPSR];
        if let ArmOperandType::Reg(reg_id) = self.second.op_type {
            set.insert(reg_id);
        }
        set
    }

    fn dest_registers(&self) -> HashSet<RegId> {
        hashset![self.dest, CPSR]
    }
}

/// gets the bit at position `n`. Bits are numbered from 0 (least significant) to 31 (most significant).
fn get_bit_at(input: u32, n: u8) -> bool {
    assert!(n <= 31);
    input & (1 << n) != 0
}
