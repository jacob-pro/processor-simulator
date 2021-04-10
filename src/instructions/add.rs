use super::Instruction;
use crate::cpu_state::execute::ExecuteChanges;
use crate::cpu_state::CpuState;
use crate::instructions::util::ArmOperandExt;
use crate::instructions::NextInstructionState;
use crate::registers::ids::{PC, CPSR};
use crate::registers::ConditionFlag;
use capstone::arch::arm::{ArmOperand, ArmOperandType};
use capstone::prelude::*;
use crate::station::ReservationStation;
use std::collections::{HashSet, HashMap};

#[allow(unused)]
#[derive(Clone)]
pub enum Mode {
    ADC,
    ADD,
    RSB,
    SBC,
    SUB,
}

#[derive(Clone)]
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
            return Self {
                update_flags,
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
                update_flags,
                mode,
                dest,
                first,
                second,
            };
        }
    }
}

impl Instruction for ADD {
    fn poll(&self, station: &ReservationStation) -> NextInstructionState {
        let mut reg_changes = HashMap::new();
        let first_val = station.read_by_id(self.first);
        let sec_val = station
            .value_of_flexible_second_operand(&self.second);
        let mut cpsr = station.read_by_id(CPSR);

        // NOTE! ARM uses an inverted carry flag for borrow (i.e. subtraction)

        let (result, carry, overflow) = match self.mode {
            Mode::ADC => {
                let (result_u, carry1) = first_val.overflowing_add(sec_val);
                let (result_s, overflow1) = (first_val as i32).overflowing_add(sec_val as i32);

                let carry = ConditionFlag::C.read_flag(cpsr) as u8;
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
                let carry = !ConditionFlag::C.read_flag(cpsr) as u8;
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

        reg_changes.insert(self.dest, result);
        if self.update_flags {
            cpsr = ConditionFlag::N.write_flag (cpsr, (result as i32).is_negative());
            cpsr = ConditionFlag::Z.write_flag(cpsr, result == 0);
            cpsr = ConditionFlag::C.write_flag(cpsr, carry);
            cpsr = ConditionFlag::V.write_flag(cpsr, overflow);
            reg_changes.insert(CPSR, cpsr);
        }
        (None, reg_changes)
    }

    fn source_registers(&self) -> HashSet<RegId> {
        let mut set = HashSet::new();
        set.insert(self.first);
        if let ArmOperandType::Reg(reg_id) = self.second.op_type {
            set.insert(reg_id);
        }
        set
    }

    fn dest_registers(&self) -> HashSet<RegId> {
        let mut set = HashSet::new();
        set.insert(self.dest);
        if self.update_flags {
            set.insert(CPSR);
        }
        set
    }

}
