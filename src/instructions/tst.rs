use super::Instruction;
use crate::instructions::util::ArmOperandExt;
use crate::instructions::PollResult;
use crate::registers::ids::CPSR;
use crate::registers::ConditionFlag;
use crate::station::ReservationStation;
use capstone::arch::arm::{ArmOperand, ArmOperandType};
use capstone::prelude::*;

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

    fn source_registers(&self) -> Vec<RegId> {
        let mut set = vec![self.first];
        if let ArmOperandType::Reg(reg_id) = self.second.op_type {
            set.push(reg_id);
        }
        set
    }

    fn dest_registers(&self) -> Vec<RegId> {
        vec![CPSR]
    }
}
