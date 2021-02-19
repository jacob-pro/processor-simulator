use capstone::arch::arm::{ArmOperand, ArmOperandType};
use capstone::prelude::*;

pub fn decode_instruction(mnemonic: &str, operands: Vec<ArmOperand>, capstone: &Capstone) -> impl Instruction {
    let upper = mnemonic.to_ascii_uppercase();
    if upper.starts_with("ADC") {
        panic!("{} not yet implemented", upper);
    }
    if upper.starts_with("ADD") {
        return NOP::new()
    }
    if upper.starts_with("ADR") {
        panic!("{} not yet implemented", upper);
    }
    if upper.starts_with("AND") {
        return NOP::new()
    }
    if upper.starts_with("ASR") {
        panic!("{} not yet implemented", upper);
    }
    if upper.starts_with("BIC") {
        panic!("{} not yet implemented", upper);
    }
    if upper.starts_with("BKPT") {
        panic!("{} not yet implemented", upper);
    }
    if upper.starts_with("BLX") {
        panic!("{} not yet implemented", upper);
    }
    if upper.starts_with("BL") {
        return NOP::new()
    }
    if upper.starts_with("BX") {
        panic!("{} not yet implemented", upper);
    }
    if upper.starts_with("B") {
        return NOP::new()
    }
    if upper.starts_with("CMN") {
        panic!("{} not yet implemented", upper);
    }
    if upper.starts_with("CMP") {
        return NOP::new()
    }
    if upper.starts_with("CPS") {
        panic!("{} not yet implemented", upper);
    }
    if upper.starts_with("DMB") {
        panic!("{} not yet implemented", upper);
    }
    if upper.starts_with("DSB") {
        panic!("{} not yet implemented", upper);
    }
    if upper.starts_with("EOR") {
        panic!("{} not yet implemented", upper);
    }
    if upper.starts_with("ISB") {
        panic!("{} not yet implemented", upper);
    }
    if upper.starts_with("LDM") {
        return NOP::new()
    }
    if upper.starts_with("LDR") {
        return NOP::new()
    }
    if upper.starts_with("LSL") {
        return NOP::new()
    }
    if upper.starts_with("LSR") {
        panic!("{} not yet implemented", upper);
    }
    if upper.starts_with("MOV") {
        return NOP::new()
    }
    if upper.starts_with("MRS") {
        panic!("{} not yet implemented", upper);
    }
    if upper.starts_with("MSR") {
        panic!("{} not yet implemented", upper);
    }
    if upper.starts_with("MUL") {
        return NOP::new()
    }
    if upper.starts_with("MVN") {
        panic!("{} not yet implemented", upper);
    }
    if upper.starts_with("NOP") {
        return NOP::new()
    }
    if upper.starts_with("ORR") {
        panic!("{} not yet implemented", upper);
    }
    if upper.starts_with("POP") {
        return NOP::new()
    }
    if upper.starts_with("PUSH") {
        return NOP::new()
    }
    if upper.starts_with("REV") {
        panic!("{} not yet implemented", upper);
    }
    if upper.starts_with("ROR") {
        panic!("{} not yet implemented", upper);
    }
    if upper.starts_with("RSB") {
        panic!("{} not yet implemented", upper);
    }
    if upper.starts_with("SBC") {
        panic!("{} not yet implemented", upper);
    }
    if upper.starts_with("SEV") {
        panic!("{} not yet implemented", upper);
    }
    if upper.starts_with("ST") {
        return NOP::new()
    }
    if upper.starts_with("SUB") {
        return NOP::new()
    }
    if upper.starts_with("SVC") {
        return NOP::new()
    }
    if upper.starts_with("SXT") {
        panic!("{} not yet implemented", upper);
    }
    if upper.starts_with("TST") {
        panic!("{} not yet implemented", upper);
    }
    if upper.starts_with("UXT") {
        panic!("{} not yet implemented", upper);
    }
    if upper.starts_with("WF") {
        panic!("{} not yet implemented", upper);
    }
    if upper.starts_with("YIELD") {
        panic!("{} not yet implemented", upper);
    }
    panic!("Unrecognised Cortex-M0 mnemonic: {}", upper);
}

pub trait Instruction {
    fn execute(&self);
}

pub struct PUSH {
    reg_list: Vec<String>
}

impl PUSH {
    fn new(operands: Vec<ArmOperand>, capstone: Capstone) -> Self {
        let reg_list = operands.into_iter()
            .map(|x: ArmOperand| {
                if let ArmOperandType::Reg(id) = x.op_type {
                    return capstone.reg_name(id).unwrap()
                }
                panic!("Unexpected operand type")
            }).collect();
        Self {
            reg_list
        }
    }
}

impl Instruction for PUSH {
    fn execute(&self) -> bool {
        unimplemented!()
    }
}

pub struct NOP {
}

impl NOP {
    fn new() -> Self {
        Self {}
    }
}

impl Instruction for NOP {
    fn execute(&self) -> bool {
        println!("NOP!!!");
        false
    }
}
