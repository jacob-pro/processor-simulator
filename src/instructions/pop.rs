use super::Instruction;
use crate::cpu_state::station::ReservationStation;
use crate::instructions::util::ArmOperandExt;
use crate::instructions::PollResult;
use crate::registers::ids::SP;
use crate::registers::RegisterFile;
use capstone::arch::arm::ArmOperand;
use capstone::prelude::*;
use std::collections::{HashSet, VecDeque};
use std::iter::FromIterator;

#[derive(Clone, Debug)]
pub struct POP {
    reg_list: VecDeque<RegId>,
}

impl POP {
    pub fn new(operands: Vec<ArmOperand>) -> Self {
        let mut reg_list: Vec<RegId> = operands
            .into_iter()
            .map(|x: ArmOperand| x.reg_id().unwrap())
            .collect();
        RegisterFile::push_pop_register_asc(&mut reg_list);
        Self {
            reg_list: VecDeque::from(reg_list),
        }
    }
}

impl Instruction for POP {
    fn poll(&self, station: &ReservationStation) -> PollResult {
        let mut changes = vec![];
        let mut sp = station.read_by_id(SP);
        for r in &self.reg_list {
            let read_from_stack = station.memory.read().unwrap().read_u32(sp).unwrap();
            changes.push((*r, read_from_stack));
            sp = sp + 4;
        }
        changes.push((SP, sp));
        PollResult::Complete(changes)
    }

    fn source_registers(&self) -> HashSet<RegId> {
        hashset![SP]
    }

    fn dest_registers(&self) -> HashSet<RegId> {
        let mut list = HashSet::from_iter(self.reg_list.clone());
        list.insert(SP);
        list
    }
}
