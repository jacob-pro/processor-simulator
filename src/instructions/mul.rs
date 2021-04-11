use super::Instruction;
use crate::instructions::util::ArmOperandExt;
use crate::instructions::PollResult;
use crate::registers::ids::CPSR;
use crate::registers::ConditionFlag;
use crate::station::ReservationStation;
use capstone::arch::arm::ArmOperand;
use capstone::prelude::*;

// Make the Multiply require extra cycles to complete
const EXTRA_CYCLES: u8 = 4;

#[derive(Clone)]
pub struct MUL {
    dest: RegId,
    val: RegId,
    cycles: u8,
}

impl MUL {
    pub fn new(operands: Vec<ArmOperand>) -> Self {
        let dest = operands[0].reg_id().unwrap();
        let val = operands[1].reg_id().unwrap();
        return Self {
            dest,
            val,
            cycles: 0,
        };
    }
}

impl Instruction for MUL {
    fn poll(&self, station: &ReservationStation) -> PollResult {
        if self.cycles < EXTRA_CYCLES {
            let mut cloned = self.clone();
            cloned.cycles = cloned.cycles + 1;
            return PollResult::Again(Box::new(cloned));
        }
        let dest_val = station.read_by_id(self.dest);
        let sec_val = station.read_by_id(self.val);
        let (result, unsigned_overflow) = dest_val.overflowing_mul(sec_val);
        let (_, signed_overflow) = (dest_val as i32).overflowing_mul(sec_val as i32);
        let mut changes = vec![(self.dest, result)];
        let mut cpsr = station.read_by_id(CPSR);
        ConditionFlag::N.write_flag(&mut cpsr, (result as i32).is_negative());
        ConditionFlag::Z.write_flag(&mut cpsr, result == 0);
        ConditionFlag::C.write_flag(&mut cpsr, unsigned_overflow);
        ConditionFlag::V.write_flag(&mut cpsr, signed_overflow);
        changes.push((CPSR, cpsr));
        PollResult::Complete(changes)
    }

    fn source_registers(&self) -> Vec<RegId> {
        vec![self.val, self.dest]
    }

    fn dest_registers(&self) -> Vec<RegId> {
        vec![self.dest, CPSR]
    }
}
