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
    sp: Option<u32>,
    changes: Vec<(RegId, u32)>,
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
            sp: None,
            changes: vec![],
        }
    }
}

impl Instruction for POP {
    fn poll(&self, station: &ReservationStation) -> PollResult {
        let mut clone = self.clone();
        if let None = clone.sp {
            clone.sp = Some(station.read_by_id(SP));
        }
        if let Some(r) = clone.reg_list.pop_front() {
            let read_from_stack = station
                .memory
                .read()
                .unwrap()
                .read_u32(clone.sp.unwrap())
                .unwrap();
            clone.changes.push((r, read_from_stack));
            clone.sp = Some(clone.sp.unwrap() + 4);
        }
        if clone.reg_list.is_empty() {
            clone.changes.push((SP, clone.sp.unwrap()));
            PollResult::Complete(clone.changes)
        } else {
            PollResult::Again(Box::new(clone))
        }
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
