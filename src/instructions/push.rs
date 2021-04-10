use super::Instruction;
use crate::cpu_state::execute::ExecuteChanges;
use crate::cpu_state::CpuState;
use crate::instructions::util::ArmOperandExt;
use crate::instructions::{PollResult};
use crate::registers::ids::SP;
use capstone::arch::arm::ArmOperand;
use capstone::prelude::*;
use std::collections::hash_map::RandomState;
use std::collections::{HashSet, HashMap};
use crate::station::ReservationStation;
use crate::registers::RegisterFile;

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
        let mut reg_changes = HashMap::new();
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
        reg_changes.insert(SP, sp);
        PollResult::Complete(reg_changes)
    }

    fn source_registers(&self) -> HashSet<RegId, RandomState> {
        let mut set = HashSet::new();
        set.insert(SP);
        for i in &self.reg_list {
            set.insert(*i);
        }
        set
    }

    fn dest_registers(&self) -> HashSet<RegId, RandomState> {
        let mut set = HashSet::new();
        set.insert(SP);
        set
    }
}
