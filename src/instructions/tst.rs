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
pub struct TST {
    first: RegId,
    second: ArmOperand,
}

impl TST {
    pub fn new(operands: Vec<ArmOperand>) -> Self {
        let first = operands[0].reg_id().unwrap();
        let second = operands[1].clone();
        return Self { first, second };
    }
}

impl Instruction for TST {
    fn poll(&self, station: &ReservationStation) -> PollResult {
        let first_val = station.read_by_id(self.first);
        let sec_val = station.value_of_flexible_second_operand(&self.second);
        let result = first_val & sec_val;
        let mut cpsr = station.read_by_id(CPSR);
        ConditionFlag::N.write_flag(&mut cpsr, (result as i32).is_negative());
        ConditionFlag::Z.write_flag(&mut cpsr, result == 0);
        PollResult::Complete(vec![(CPSR, cpsr)])
    }

    fn source_registers(&self) -> HashSet<RegId> {
        let mut set = hashset![self.first, CPSR];
        if let ArmOperandType::Reg(reg_id) = self.second.op_type {
            set.insert(reg_id);
        }
        set
    }

    fn dest_registers(&self) -> HashSet<RegId> {
        hashset![CPSR]
    }
}
