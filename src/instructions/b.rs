use super::Instruction;
use crate::cpu_state::station::ReservationStation;
use crate::instructions::util::ArmOperandExt;
use crate::instructions::PollResult;
use crate::registers::ids::{LR, PC};
use capstone::arch::arm::ArmOperand;
use capstone::RegId;
use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct B {
    jump: i32,
    with_link: bool,
}

impl B {
    pub fn new(operands: Vec<ArmOperand>, with_link: bool) -> Self {
        let jump = operands[0].imm_value().unwrap();
        Self { jump, with_link }
    }
}

impl Instruction for B {
    fn poll(&self, station: &ReservationStation) -> PollResult {
        let mut changes = vec![];
        let cur = station.instruction.as_ref().unwrap();
        if self.with_link {
            // copy the address of the next instruction into LR
            // BL and BLX instructions also set bit[0] of the LR to 1
            // so that the value is suitable for use by a subsequent POP {PC}
            changes.push((LR, cur.address + cur.length));
        }
        changes.push((PC, (cur.address as i64 + self.jump as i64) as u32));
        PollResult::Complete(changes)
    }

    fn source_registers(&self) -> HashSet<RegId> {
        hashset![]
    }

    fn dest_registers(&self) -> HashSet<RegId> {
        let mut dest = hashset![PC];
        if self.with_link {
            dest.insert(LR);
        }
        dest
    }
}
