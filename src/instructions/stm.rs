use super::{Instruction, ShouldTerminate};
use crate::simulator::Simulator;
use capstone::arch::arm::ArmOperand;
use crate::instructions::util::ArmOperandExt;
use capstone::prelude::*;

pub struct STM {
    base_register: RegId,
    reg_list: Vec<RegId>,
    writeback: bool,
}

impl STM {
    pub fn new(operands: Vec<ArmOperand>, writeback: bool) -> Self {
        let reg_list: Vec<RegId> = operands.into_iter()
            .map(|x: ArmOperand| x.reg_id().unwrap()).collect();
        Self { base_register: reg_list[0], reg_list: reg_list[1..].to_vec(), writeback }
    }
}

impl Instruction for STM {
    fn execute(&self, sim: &mut Simulator) -> ShouldTerminate {
        let base_addr = sim.registers.read_by_id(self.base_register);
        for (idx, reg) in self.reg_list.iter().enumerate() {
            let adj_addr = base_addr + (idx as u32 * 4);
            let reg_val = sim.registers.read_by_id(*reg);
            sim.memory.write_bytes(adj_addr, &reg_val.to_le_bytes());
        }
        if self.writeback {
            let final_address = base_addr + (self.reg_list.len() as u32 * 4);
            sim.registers.write_by_id(self.base_register, final_address);
        }
        false
    }
}
