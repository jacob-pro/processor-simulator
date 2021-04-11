use super::Instruction;
use crate::cpu_state::CpuState;
use crate::instructions::util::ArmOperandExt;
use crate::instructions::PollResult;
use crate::registers::ids::CPSR;
use crate::registers::ConditionFlag;
use crate::station::ReservationStation;
use capstone::arch::arm::{ArmOperand, ArmOperandType};
use capstone::prelude::*;

#[derive(Clone)]
pub enum Mode {
    CMP,
    CMN,
}

#[derive(Clone)]
pub struct CMP {
    mode: Mode,
    first: RegId,
    second: ArmOperand,
}

impl CMP {
    pub fn new(operands: Vec<ArmOperand>, mode: Mode) -> Self {
        Self {
            mode,
            first: operands[0].reg_id().unwrap(),
            second: operands[1].clone(),
        }
    }
}

impl Instruction for CMP {
    fn poll(&self, station: &ReservationStation) -> PollResult {
        let first_val = station.read_by_id(self.first);
        let sec_val = station.value_of_flexible_second_operand(&self.second);

        let (result, carry, overflow) = match self.mode {
            Mode::CMN => {
                // Same as ADD
                let (result, carry) = first_val.overflowing_add(sec_val);
                let (_, overflow) = (first_val as i32).overflowing_add(sec_val as i32);
                (result, carry, overflow)
            }
            Mode::CMP => {
                // Same as SUB
                let (result, carry) = first_val.overflowing_sub(sec_val);
                let (_, overflow) = (first_val as i32).overflowing_sub(sec_val as i32);
                (result, !carry, overflow)
            }
        };

        let mut cpsr = station.read_by_id(CPSR);
        ConditionFlag::N.write_flag(cpsr, (result as i32).is_negative());
        ConditionFlag::Z.write_flag(cpsr, result == 0);
        ConditionFlag::C.write_flag(cpsr, carry);
        ConditionFlag::V.write_flag(cpsr, overflow);
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
