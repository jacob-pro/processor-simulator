use super::Instruction;
use crate::cpu_state::execute::ExecuteChanges;
use crate::cpu_state::CpuState;
use crate::instructions::util::ArmOperandExt;
use crate::registers::ConditionFlag;
use capstone::arch::arm::ArmOperand;
use capstone::prelude::*;

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
    fn execute(&self, sim: &CpuState, changes: &mut ExecuteChanges) {
        let first_val = sim.registers.read_by_id(self.first);
        let sec_val = sim
            .registers
            .value_of_flexible_second_operand(&self.second, true);
        let result = first_val & sec_val;
        changes.flag_change(ConditionFlag::N, (result as i32).is_negative());
        changes.flag_change(ConditionFlag::Z, result == 0);
    }
}
