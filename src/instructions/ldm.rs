use super::{Instruction, ShouldTerminate};
use crate::simulator::Simulator;
use capstone::arch::arm::ArmOperand;
use crate::instructions::util::ArmOperandExt;
use capstone::prelude::*;

pub struct LDM {
    base_register: RegId,
    reg_list: Vec<RegId>,
    writeback: bool,
}

impl LDM {
    pub fn new(operands: Vec<ArmOperand>, writeback: bool) -> Self {
        let reg_list: Vec<RegId> = operands.into_iter()
            .map(|x: ArmOperand| x.reg_id().unwrap()).collect();
        Self { base_register: reg_list[0], reg_list: reg_list[1..].to_vec(), writeback }
    }
}

impl Instruction for LDM {
    // https://keleshev.com/ldm-my-favorite-arm-instruction/
    fn execute(&self, sim: &mut Simulator) -> ShouldTerminate {
        let base_addr = *sim.registers.get_by_id(self.base_register);
        for (idx, reg) in self.reg_list.iter().enumerate() {
            let adj_addr = base_addr + (idx as u32 * 4);
            let val = sim.memory.read_u32(adj_addr);
            *sim.registers.get_by_id(*reg) = val;
        }
        if self.writeback {
            let final_address = base_addr + ((self.reg_list.len() as u32 - 1) * 4);
            *sim.registers.get_by_id(self.base_register) = final_address;
        }
        false
    }
}
