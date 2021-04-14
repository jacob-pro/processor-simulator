use super::Instruction;
use crate::cpu_state::station::ReservationStation;
use crate::instructions::util::ArmOperandExt;
use crate::instructions::PollResult;
use capstone::arch::arm::ArmOperand;
use capstone::prelude::*;
use std::collections::{HashSet, VecDeque};
use std::iter::FromIterator;

#[derive(Clone, Debug)]
pub struct LDM {
    base_register: RegId,
    reg_list: VecDeque<RegId>,
    writeback: bool,
    address: Option<u32>,
    changes: Vec<(RegId, u32)>,
}

impl LDM {
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

impl Instruction for LDM {
    // https://keleshev.com/ldm-my-favorite-arm-instruction/
    fn poll(&self, station: &ReservationStation) -> PollResult {
        let mut clone = self.clone();
        if let None = clone.address {
            clone.address = Some(station.read_by_id(self.base_register));
        }

        if let Some(reg) = clone.reg_list.pop_front() {
            let val = station
                .memory
                .read()
                .unwrap()
                .read_u32(clone.address.unwrap())
                .unwrap();
            clone.changes.push((reg, val));
            clone.address = Some(clone.address.unwrap() + 4);
            return PollResult::Again(Box::new(clone));
        }

        if clone.writeback {
            clone
                .changes
                .push((self.base_register, self.address.unwrap()));
            clone.writeback = false;
            return PollResult::Again(Box::new(clone));
        }

        PollResult::Complete(clone.changes)
    }

    fn source_registers(&self) -> HashSet<RegId> {
        hashset![self.base_register]
    }

    fn dest_registers(&self) -> HashSet<RegId> {
        let mut list = HashSet::from_iter(self.reg_list.clone());
        if self.writeback {
            list.insert(self.base_register);
        }
        list
    }
}
