use super::Instruction;
use crate::instructions::util::ArmOperandExt;
use crate::instructions::PollResult;
use crate::station::ReservationStation;
use capstone::arch::arm::ArmOperand;
use capstone::prelude::*;

#[derive(Clone)]
pub enum Mode {
    SXTB,
    UXTB,
    SXTH,
    UXTH,
}

#[derive(Clone)]
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
    fn poll(&self, station: &ReservationStation) -> PollResult {
        let mut changes = vec![];
        let value = station.read_by_id(self.src);
        match self.mode {
            Mode::SXTB => {
                // extracts bits[7:0] and sign extends to 32 bits
                let smol = value as i8;
                let big = smol as i32;
                changes.push((self.dest, big as u32));
            }
            Mode::UXTB => {
                // extracts bits[7:0] and zero extends to 32 bits
                let smol = value as u8;
                changes.push((self.dest, smol as u32));
            }
            Mode::SXTH => {
                // extracts bits[15:0] and sign extends to 32 bits
                let smol = value as i16;
                let big = smol as i32;
                changes.push((self.dest, big as u32));
            }
            Mode::UXTH => {
                // extracts bits[15:0] and zero extends to 32 bits.
                let smol = value as u16;
                changes.push((self.dest, smol as u32));
            }
        }
        PollResult::Complete(changes)
    }

    fn source_registers(&self) -> Vec<RegId> {
        vec![self.src]
    }

    fn dest_registers(&self) -> Vec<RegId> {
        vec![self.dest]
    }
}
