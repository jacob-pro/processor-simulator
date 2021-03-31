use super::{Instruction, ShouldTerminate};
use crate::instructions::util::ArmOperandExt;
use crate::simulator::Simulator;
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
    fn execute(&self, sim: &mut Simulator) -> ShouldTerminate {
        let first_val = sim.registers.read_by_id(self.dest);
        let sec_val = sim.registers.read_by_id(self.second);
        let result = match self.mode {
            Mode::AND => first_val & sec_val,
            Mode::ORR => first_val | sec_val,
            Mode::EOR => first_val ^ sec_val,
            Mode::BIC => first_val & (!sec_val),
        };
        sim.registers.write_by_id(self.dest, result);
        sim.registers.cond_flags.n = (result as i32).is_negative();
        sim.registers.cond_flags.z = result == 0;
        false
    }
}
