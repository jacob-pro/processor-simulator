use crate::instructions::Instruction;
use capstone::arch::arm::ArmCC;
use std::collections::HashSet;
use capstone::RegId;
use crate::registers::ids::CPSR;

pub struct DecodedInstruction {
    pub imp: Box<dyn Instruction>,
    pub cc: ArmCC,
    pub string: String,
    pub length: u32,
    pub address: u32,
}

impl DecodedInstruction {

    pub fn source_registers(&self) -> HashSet<RegId> {
        // Every instruction needs the CPSR register to be ready
        // to know if to execute or not
        let mut base = self.imp.source_registers();
        base.insert(CPSR);
        base
    }

}
