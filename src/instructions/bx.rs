use super::Instruction;
use crate::instructions::util::ArmOperandExt;
use crate::instructions::PollResult;
use crate::registers::ids::{LR, PC};
use crate::station::ReservationStation;
use capstone::arch::arm::ArmOperand;
use capstone::prelude::*;

#[derive(Clone)]
pub struct BX {
    register: RegId,
    with_link: bool,
}

impl BX {
    pub fn new(operands: Vec<ArmOperand>, with_link: bool) -> Self {
        let register = operands[0].reg_id().unwrap();
        Self {
            register,
            with_link,
        }
    }
}

impl Instruction for BX {
    fn poll(&self, station: &ReservationStation) -> PollResult {
        let mut changes = vec![];
        if self.with_link {
            // copy the address of the next instruction into LR
            // BL and BLX instructions also set bit[0] of the LR to 1
            // so that the value is suitable for use by a subsequent POP {PC}
            let cur = station.instruction.as_ref().unwrap();
            changes.push((LR, cur.address + cur.length));
        }
        let new_addr = station.read_by_id(self.register);
        changes.push((PC, new_addr));
        PollResult::Complete(changes)
    }

    fn source_registers(&self) -> Vec<RegId> {
        vec![self.register]
    }

    fn dest_registers(&self) -> Vec<RegId> {
        let mut dest = vec![PC];
        if self.with_link {
            dest.push(LR);
        }
        dest
    }
}
