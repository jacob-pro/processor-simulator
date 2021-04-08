use super::Instruction;
use crate::cpu_state::execute::ExecuteChanges;
use crate::cpu_state::CpuState;
use crate::instructions::util::ArmOperandExt;
use crate::instructions::ExecutionComplete;
use crate::registers::ConditionFlag;
use capstone::arch::arm::ArmOperand;
use capstone::prelude::*;

pub enum Mode {
    AND,
    ORR,
    EOR,
    BIC,
}

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
    fn poll(&self, state: &CpuState, changes: &mut ExecuteChanges) -> ExecutionComplete {
        let first_val = state.registers.read_by_id(self.dest);
        let sec_val = state.registers.read_by_id(self.second);
        let result = match self.mode {
            Mode::AND => first_val & sec_val,
            Mode::ORR => first_val | sec_val,
            Mode::EOR => first_val ^ sec_val,
            Mode::BIC => first_val & (!sec_val),
        };
        changes.register_change(self.dest, result);
        changes.flag_change(ConditionFlag::N, (result as i32).is_negative());
        changes.flag_change(ConditionFlag::Z, result == 0);
        true
    }
}
