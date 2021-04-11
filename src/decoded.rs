use crate::instructions::Instruction;
use crate::registers::ids::CPSR;
use capstone::arch::arm::ArmCC;
use capstone::RegId;
use std::collections::HashSet;

pub struct DecodedInstruction {
    pub imp: Box<dyn Instruction>,
    pub cc: ArmCC,
    pub string: String,
    pub length: u32,
    pub address: u32,
}

impl DecodedInstruction {
    pub fn source_registers(&self) -> Vec<RegId> {
        // Every instruction needs the CPSR register to be ready
        // to know if to execute or not
        let mut base = self.imp.source_registers();
        base.push(CPSR);
        base
    }
}
