use super::Instruction;
use crate::cpu_state::execute::ExecuteChanges;
use crate::cpu_state::CpuState;
use crate::instructions::util::ArmOperandExt;
use crate::instructions::NextInstructionState;
use crate::registers::ConditionFlag;
use capstone::arch::arm::ArmOperand;
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
    fn poll(&self, state: &CpuState, changes: &mut ExecuteChanges) -> NextInstructionState {
        let first_val = state.registers.read_by_id(self.first);
        let sec_val = state
            .registers
            .value_of_flexible_second_operand(&self.second, false);

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

        changes.flag_change(ConditionFlag::N, (result as i32).is_negative());
        changes.flag_change(ConditionFlag::Z, result == 0);
        changes.flag_change(ConditionFlag::C, carry);
        changes.flag_change(ConditionFlag::V, overflow);
        None
    }
}
