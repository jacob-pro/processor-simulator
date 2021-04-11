use super::Instruction;
use crate::cpu_state::CpuState;
use crate::instructions::util::ArmOperandExt;
use crate::instructions::PollResult;
use crate::registers::ids::{LR, PC};
use crate::station::ReservationStation;
use capstone::arch::arm::ArmOperand;
use capstone::RegId;

#[derive(Clone)]
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

    fn source_registers(&self) -> Vec<RegId> {
        vec![]
    }

    fn dest_registers(&self) -> Vec<RegId> {
        let mut dest = vec![PC];
        if self.with_link {
            dest.push(LR);
        }
        dest
    }
}
