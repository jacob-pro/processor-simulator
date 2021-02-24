use super::{Instruction, ShouldTerminate};
use crate::simulator::Simulator;
use capstone::prelude::*;
use capstone::arch::arm::ArmOperand;
use crate::instructions::util::ArmOperandExt;

#[derive(PartialEq)]
pub enum Mode {
    MOV,
    MVN,
}

pub struct MOV {
    update_flags: bool,
    mode: Mode,
    dest: RegId,
    src: ArmOperand,
}

impl MOV {
    pub fn new(operands: Vec<ArmOperand>, mode: Mode, update_flags: bool) -> Self {
        let dest = operands[0].reg_id().unwrap();
        Self { update_flags, mode, dest, src: operands[1].clone() }
    }
}

impl Instruction for MOV {
    fn execute(&self, sim: &mut Simulator) -> ShouldTerminate {
        let mut val= sim.registers.value_of_flexible_second_operand(&self.src, self.update_flags);
        if sim.registers.reg_name(self.dest) == "PC" {
            val = val | 1;  // When Rd is the PC in a MOV instruction: Bit[0] of the result is discarded.
        }
        if self.mode == Mode::MVN {
            val = !val;
        }
        sim.registers.write_by_id(self.dest, val);
        if self.update_flags {
            sim.registers.cond_flags.n = (val as i32).is_negative();
            sim.registers.cond_flags.z = val == 0;
        }
        false
    }
}
