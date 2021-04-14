use super::Instruction;
use crate::cpu_state::station::ReservationStation;
use crate::instructions::util::ArmOperandExt;
use crate::instructions::PollResult;
use crate::registers::ids::SP;
use crate::registers::RegisterFile;
use capstone::arch::arm::ArmOperand;
use capstone::prelude::*;
use std::collections::{HashSet, VecDeque};

#[derive(Clone, Debug)]
pub struct PUSH {
    reg_list: VecDeque<RegId>,
    sp: Option<u32>,
}

impl PUSH {
    pub fn new(operands: Vec<ArmOperand>) -> Self {
        let mut reg_list: Vec<RegId> = operands
            .into_iter()
            .map(|x: ArmOperand| x.reg_id().unwrap())
            .collect();
        RegisterFile::push_pop_register_asc(&mut reg_list);
        reg_list.reverse();
        Self {
            reg_list: VecDeque::from(reg_list),
            sp: None,
        }
    }
}

impl Instruction for PUSH {
    fn poll(&self, station: &ReservationStation) -> PollResult {
        let mut clone = self.clone();
        if let None = clone.sp {
            clone.sp = Some(station.read_by_id(SP));
        }
        if let Some(r) = clone.reg_list.pop_front() {
            clone.sp = Some(clone.sp.unwrap() - 4);
            let register_value = station.read_by_id(r).to_le_bytes();
            station
                .memory
                .write()
                .unwrap()
                .write_bytes(clone.sp.unwrap(), &register_value)
                .unwrap();
        }
        if clone.reg_list.is_empty() {
            PollResult::Complete(vec![(SP, clone.sp.unwrap())])
        } else {
            PollResult::Again(Box::new(clone))
        }
    }

    fn source_registers(&self) -> HashSet<RegId> {
        let mut set = hashset![SP];
        for i in &self.reg_list {
            set.insert(*i);
        }
        set
    }

    fn dest_registers(&self) -> HashSet<RegId> {
        hashset![SP]
    }

    fn hazardous(&self) -> bool {
        true // Not quite sure why push causes issues with svc?
    }
}
