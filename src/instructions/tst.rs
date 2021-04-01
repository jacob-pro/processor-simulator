use super::{Instruction, ShouldTerminate};
use crate::instructions::util::ArmOperandExt;
use crate::simulator::{Simulator, ExecuteChanges};
use capstone::arch::arm::ArmOperand;
use capstone::prelude::*;
use crate::registers::ConditionFlag;

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
    fn execute(&self, sim: &Simulator, changes: &mut ExecuteChanges) -> ShouldTerminate {
        let first_val = sim.registers.read_by_id(self.first);
        let sec_val = sim
            .registers
            .value_of_flexible_second_operand(&self.second, true);
        let result = first_val & sec_val;
        changes.flag_change(ConditionFlag::N, (result as i32).is_negative());
        changes.flag_change(ConditionFlag::Z, result == 0);
        false
    }
}
