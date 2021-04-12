use super::Instruction;
use crate::cpu_state::station::ReservationStation;
use crate::instructions::util::ArmOperandExt;
use crate::instructions::PollResult;
use crate::registers::ids::{CPSR, PC};
use crate::registers::ConditionFlag;
use capstone::arch::arm::{ArmOperand, ArmOperandType};
use capstone::prelude::*;
use std::collections::HashSet;

#[derive(PartialEq, Debug)]
pub enum Mode {
    MOV,
    MVN,
}

#[derive(Debug)]
pub struct MOV {
    update_flags: bool,
    mode: Mode,
    dest: RegId,
    src: ArmOperand,
}

impl MOV {
    pub fn new(operands: Vec<ArmOperand>, mode: Mode, update_flags: bool) -> Self {
        let dest = operands[0].reg_id().unwrap();
        Self {
            update_flags,
            mode,
            dest,
            src: operands[1].clone(),
        }
    }
}

impl Instruction for MOV {
    fn poll(&self, station: &ReservationStation) -> PollResult {
        let mut val = station.value_of_flexible_second_operand(&self.src);
        if self.dest == PC {
            val = val | 1; // When Rd is the PC in a MOV instruction: Bit[0] of the result is discarded.
        }
        if self.mode == Mode::MVN {
            val = !val;
        }
        let mut changes = vec![(self.dest, val)];
        if self.update_flags {
            let mut cpsr = station.read_by_id(CPSR);
            ConditionFlag::N.write_flag(&mut cpsr, (val as i32).is_negative());
            ConditionFlag::Z.write_flag(&mut cpsr, val == 0);
            changes.push((CPSR, cpsr));
        }
        PollResult::Complete(changes)
    }

    fn source_registers(&self) -> HashSet<RegId> {
        let mut set = hashset![];
        if let ArmOperandType::Reg(reg_id) = self.src.op_type {
            set.insert(reg_id);
        }
        if self.update_flags {
            set.insert(CPSR);
        }
        set
    }

    fn dest_registers(&self) -> HashSet<RegId> {
        let mut dest = hashset![self.dest];
        if self.update_flags {
            dest.insert(CPSR);
        }
        dest
    }
}
