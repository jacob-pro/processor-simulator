use super::Instruction;
use crate::cpu_state::station::ReservationStation;
use crate::instructions::util::ArmOperandExt;
use crate::instructions::PollResult;
use capstone::arch::arm::ArmOperand;
use capstone::prelude::*;
use std::collections::{HashSet, VecDeque};
use std::iter::FromIterator;

#[derive(Clone, Debug)]
pub struct STM {
    base_register: RegId,
    reg_list: VecDeque<RegId>,
    writeback: bool,
    address: Option<u32>,
    changes: Vec<(RegId, u32)>,
}

impl STM {
    pub fn new(operands: Vec<ArmOperand>, writeback: bool) -> Self {
        let reg_list: Vec<RegId> = operands
            .into_iter()
            .map(|x: ArmOperand| x.reg_id().unwrap())
            .collect();
        Self {
            base_register: reg_list[0],
            reg_list: reg_list[1..].to_vec().into(),
            writeback,
            address: None,
            changes: vec![],
        }
    }
}

impl Instruction for STM {
    fn poll(&self, station: &ReservationStation) -> PollResult {
        let mut clone = self.clone();
        if let None = clone.address {
            clone.address = Some(station.read_by_id(self.base_register));
        }

        if let Some(reg) = clone.reg_list.pop_front() {
            let reg_val = station.read_by_id(reg);
            station
                .memory
                .write()
                .unwrap()
                .write_bytes(clone.address.unwrap(), &reg_val.to_le_bytes())
                .unwrap();
            clone.address = Some(clone.address.unwrap() + 4);
            return PollResult::Again(Box::new(clone));
        }

        if self.writeback {
            clone
                .changes
                .push((self.base_register, self.address.unwrap()));
            clone.writeback = false;
            return PollResult::Again(Box::new(clone));
        }

        PollResult::Complete(clone.changes)
    }

    fn source_registers(&self) -> HashSet<RegId> {
        let mut set = HashSet::from_iter(self.reg_list.clone());
        set.insert(self.base_register);
        set
    }

    fn dest_registers(&self) -> HashSet<RegId> {
        if self.writeback {
            return hashset![self.base_register];
        }
        hashset![]
    }
}
