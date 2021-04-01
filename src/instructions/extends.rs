use super::{Instruction, ShouldTerminate};
use crate::instructions::util::ArmOperandExt;
use crate::simulator::{ExecuteChanges, Simulator};
use capstone::arch::arm::ArmOperand;
use capstone::prelude::*;

pub enum Mode {
    SXTB,
    UXTB,
    SXTH,
    UXTH,
}

pub struct EXTENDS {
    dest: RegId,
    src: RegId,
    mode: Mode,
}

impl EXTENDS {
    pub fn new(operands: Vec<ArmOperand>, mode: Mode) -> Self {
        let dest = operands[0].reg_id().unwrap();
        let src = operands[1].reg_id().unwrap();
        Self { dest, src, mode }
    }
}

impl Instruction for EXTENDS {
    fn execute(&self, sim: &Simulator, changes: &mut ExecuteChanges) -> ShouldTerminate {
        let value = sim.registers.read_by_id(self.src);
        match self.mode {
            Mode::SXTB => {
                // extracts bits[7:0] and sign extends to 32 bits
                let smol = value as i8;
                let big = smol as i32;
                changes.register_change(self.dest, big as u32);
            }
            Mode::UXTB => {
                // extracts bits[7:0] and zero extends to 32 bits
                let smol = value as u8;
                changes.register_change(self.dest, smol as u32);
            }
            Mode::SXTH => {
                // extracts bits[15:0] and sign extends to 32 bits
                let smol = value as i16;
                let big = smol as i32;
                changes.register_change(self.dest, big as u32);
            }
            Mode::UXTH => {
                // extracts bits[15:0] and zero extends to 32 bits.
                let smol = value as u16;
                changes.register_change(self.dest, smol as u32);
            }
        }
        false
    }
}
