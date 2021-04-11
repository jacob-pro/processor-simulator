use super::Instruction;
use crate::cpu_state::CpuState;
use crate::instructions::util::ArmOperandExt;
use crate::instructions::PollResult;
use crate::registers::ids::PC;
use crate::station::ReservationStation;
use capstone::arch::arm::ArmOperand;
use capstone::prelude::*;

#[derive(Clone)]
pub struct ADR {
    dest: RegId,
    pc_rel: i32,
}

impl ADR {
    pub fn new(operands: Vec<ArmOperand>) -> Self {
        let dest = operands[0].reg_id().unwrap();
        return Self {
            dest,
            pc_rel: operands[1].imm_value().unwrap(),
        };
    }
}

impl Instruction for ADR {
    fn poll(&self, station: &ReservationStation) -> PollResult {
        // PC always appears as the current instruction address + 4 bytes - even in Thumb state
        let pc = ((station.read_by_id(PC) + 4) & 0xFFFFFFFC) as i64;
        let relative = pc + self.pc_rel as i64;
        PollResult::Complete(vec![(self.dest, relative as u32)])
    }

    fn source_registers(&self) -> Vec<RegId> {
        vec![]
    }

    fn dest_registers(&self) -> Vec<RegId> {
        vec![self.dest]
    }
}
