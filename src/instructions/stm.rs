use super::Instruction;
use crate::instructions::util::ArmOperandExt;
use crate::instructions::PollResult;
use crate::station::ReservationStation;
use capstone::arch::arm::ArmOperand;
use capstone::prelude::*;

#[derive(Clone)]
pub struct STM {
    base_register: RegId,
    reg_list: Vec<RegId>,
    writeback: bool,
}

impl STM {
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

impl Instruction for STM {
    fn poll(&self, station: &ReservationStation) -> PollResult {
        let base_addr = station.read_by_id(self.base_register);
        for (idx, reg) in self.reg_list.iter().enumerate() {
            let adj_addr = base_addr + (idx as u32 * 4);
            let reg_val = station.read_by_id(*reg);
            station
                .memory
                .write()
                .unwrap()
                .write_bytes(adj_addr, &reg_val.to_le_bytes());
        }
        let mut changes = vec![];
        if self.writeback {
            let final_address = base_addr + (self.reg_list.len() as u32 * 4);
            changes.push((self.base_register, final_address));
        }
        PollResult::Complete(changes)
    }

    fn source_registers(&self) -> Vec<RegId> {
        let mut list = self.reg_list.clone();
        list.push(self.base_register);
        list
    }

    fn dest_registers(&self) -> Vec<RegId> {
        if self.writeback {
            return vec![self.base_register];
        }
        vec![]
    }
}
