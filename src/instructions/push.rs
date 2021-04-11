use super::Instruction;
use crate::cpu_state::execute::StationChanges;
use crate::instructions::util::ArmOperandExt;
use crate::instructions::PollResult;
use crate::registers::ids::SP;
use crate::registers::RegisterFile;
use crate::station::ReservationStation;
use capstone::arch::arm::ArmOperand;
use capstone::prelude::*;

#[derive(Clone)]
pub struct PUSH {
    reg_list: Vec<RegId>,
}

impl PUSH {
    pub fn new(operands: Vec<ArmOperand>) -> Self {
        let reg_list: Vec<RegId> = operands
            .into_iter()
            .map(|x: ArmOperand| x.reg_id().unwrap())
            .collect();
        Self { reg_list }
    }
}

impl Instruction for PUSH {
    fn poll(&self, station: &ReservationStation) -> PollResult {
        let mut reg_list = RegisterFile::push_pop_register_asc(self.reg_list.clone());
        reg_list.reverse();
        let mut sp = station.read_by_id(SP);
        for r in &reg_list {
            sp = sp - 4;
            let register_value = station.read_by_id(*r).to_le_bytes();
            station
                .memory
                .write()
                .unwrap()
                .write_bytes(sp, &register_value);
        }
        PollResult::Complete(vec![(SP, sp)])
    }

    fn source_registers(&self) -> Vec<RegId> {
        let mut set = vec![SP];
        for i in &self.reg_list {
            set.push(*i);
        }
        set
    }

    fn dest_registers(&self) -> Vec<RegId> {
        vec![SP]
    }
}
