use super::Instruction;
use crate::cpu_state::station::ReservationStation;
use crate::instructions::util::ArmOperandExt;
use crate::instructions::PollResult;
use crate::registers::ids::CPSR;
use crate::registers::ConditionFlag;
use capstone::arch::arm::ArmOperand;
use capstone::prelude::*;
use std::collections::HashSet;

#[derive(Clone, Debug)]
pub enum Mode {
    AND,
    ORR,
    EOR,
    BIC,
}

#[derive(Clone, Debug)]
pub struct LOGICAL {
    dest: RegId,
    second: RegId,
    mode: Mode,
}

impl LOGICAL {
    pub fn new(operands: Vec<ArmOperand>, mode: Mode) -> Self {
        let dest = operands[0].reg_id().unwrap();
        let second = operands[1].reg_id().unwrap();
        return Self { dest, second, mode };
    }
}

impl Instruction for LOGICAL {
    fn poll(&self, station: &ReservationStation) -> PollResult {
        let first_val = station.read_by_id(self.dest);
        let sec_val = station.read_by_id(self.second);
        let result = match self.mode {
            Mode::AND => first_val & sec_val,
            Mode::ORR => first_val | sec_val,
            Mode::EOR => first_val ^ sec_val,
            Mode::BIC => first_val & (!sec_val),
        };
        let mut changes = vec![(self.dest, result)];
        let mut cpsr = station.read_by_id(CPSR);
        ConditionFlag::N.write_flag(&mut cpsr, (result as i32).is_negative());
        ConditionFlag::Z.write_flag(&mut cpsr, result == 0);
        changes.push((CPSR, cpsr));
        PollResult::Complete(changes)
    }

    fn source_registers(&self) -> HashSet<RegId> {
        hashset![self.dest, self.second, CPSR]
    }

    fn dest_registers(&self) -> HashSet<RegId> {
        hashset![self.dest, CPSR]
    }
}
