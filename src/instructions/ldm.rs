use super::Instruction;
use crate::instructions::util::ArmOperandExt;
use crate::instructions::PollResult;
use crate::station::ReservationStation;
use capstone::arch::arm::ArmOperand;
use capstone::prelude::*;

#[derive(Clone)]
pub struct LDM {
    base_register: RegId,
    reg_list: Vec<RegId>,
    writeback: bool,
}

impl LDM {
    pub fn new(operands: Vec<ArmOperand>, writeback: bool) -> Self {
        let reg_list: Vec<RegId> = operands
            .into_iter()
            .map(|x: ArmOperand| x.reg_id().unwrap())
            .collect();
        Self {
            base_register: reg_list[0],
            reg_list: reg_list[1..].to_vec(),
            writeback,
        }
    }
}

impl Instruction for LDM {
    // https://keleshev.com/ldm-my-favorite-arm-instruction/
    fn poll(&self, station: &ReservationStation) -> PollResult {
        let mut changes = vec![];
        let base_addr = station.read_by_id(self.base_register);
        for (idx, reg) in self.reg_list.iter().enumerate() {
            let adj_addr = base_addr + (idx as u32 * 4);
            let val = station.memory.read().unwrap().read_u32(adj_addr);
            changes.push((*reg, val));
        }
        if self.writeback {
            let final_address = base_addr + (self.reg_list.len() as u32 * 4);
            changes.push((self.base_register, final_address));
        }
        PollResult::Complete(changes)
    }

    fn source_registers(&self) -> Vec<RegId> {
        vec![self.base_register]
    }

    fn dest_registers(&self) -> Vec<RegId> {
        let mut list = self.reg_list.clone();
        if self.writeback {
            list.push(self.base_register);
        }
        list
    }
}
