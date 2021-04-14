use super::Instruction;
use crate::cpu_state::station::ReservationStation;
use crate::instructions::util::ArmOperandExt;
use crate::instructions::PollResult;
use crate::registers::ids::{LR, PC};
use capstone::arch::arm::ArmOperand;
use capstone::prelude::*;
use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct BX {
    register: RegId,
    with_link: bool,
    changes: Vec<(RegId, u32)>,
}

impl BX {
    pub fn new(operands: Vec<ArmOperand>, with_link: bool) -> Self {
        let register = operands[0].reg_id().unwrap();
        Self {
            register,
            with_link,
            changes: vec![],
        }
    }
}

impl Instruction for BX {
    fn poll(&self, station: &ReservationStation) -> PollResult {
        let mut clone = self.clone();
        if clone.with_link {
            // copy the address of the next instruction into LR
            // BL and BLX instructions also set bit[0] of the LR to 1
            // so that the value is suitable for use by a subsequent POP {PC}
            let cur = station.instruction.as_ref().unwrap();
            clone.changes.push((LR, cur.address + cur.length));
            clone.with_link = false;
            return PollResult::Again(Box::new(clone));
        }
        let new_addr = station.read_by_id(self.register);
        clone.changes.push((PC, new_addr));
        PollResult::Complete(clone.changes)
    }

    fn source_registers(&self) -> HashSet<RegId> {
        hashset![self.register]
    }

    fn dest_registers(&self) -> HashSet<RegId> {
        let mut dest = hashset![PC];
        if self.with_link {
            dest.insert(LR);
        }
        dest
    }
}
