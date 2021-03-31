use super::{Instruction, ShouldTerminate};
use crate::instructions::util::ArmOperandExt;
use crate::simulator::Simulator;
use capstone::arch::arm::ArmOperand;
use capstone::prelude::*;

pub struct MUL {
    dest: RegId,
    val: RegId,
}

impl MUL {
    pub fn new(operands: Vec<ArmOperand>) -> Self {
        let dest = operands[0].reg_id().unwrap();
        let val = operands[1].reg_id().unwrap();
        return Self { dest, val };
    }
}

impl Instruction for MUL {
    fn execute(&self, sim: &mut Simulator) -> ShouldTerminate {
        let dest_val = sim.registers.read_by_id(self.dest);
        let sec_val = sim.registers.read_by_id(self.val);
        let (result, unsigned_overflow) = dest_val.overflowing_mul(sec_val);
        let (_, signed_overflow) = (dest_val as i32).overflowing_mul(sec_val as i32);
        sim.registers.write_by_id(self.dest, result);
        sim.registers.cond_flags.n = (result as i32).is_negative();
        sim.registers.cond_flags.z = result == 0;
        sim.registers.cond_flags.c = unsigned_overflow;
        sim.registers.cond_flags.v = signed_overflow;
        false
    }
}
