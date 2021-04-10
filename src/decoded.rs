use crate::instructions::Instruction;
use capstone::arch::arm::ArmCC;

pub struct DecodedInstruction {
    pub imp: Box<dyn Instruction>,
    pub cc: ArmCC,
    pub string: String,
    pub length: u32,
    pub address: u32,
}
