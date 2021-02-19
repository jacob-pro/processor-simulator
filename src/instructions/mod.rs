mod nop;
mod push;
mod svc;

use capstone::arch::arm::ArmOperand;
use capstone::prelude::*;
use crate::simulator::Simulator;

pub type ShouldTerminate = bool;

pub trait Instruction {
    fn execute(&self, sim: &mut Simulator) -> ShouldTerminate;
}

pub fn decode_instruction(mnemonic: &str,
                          operands: Vec<ArmOperand>,
                          capstone: &Capstone) -> Box<dyn Instruction> {
    let mnemonic = mnemonic.to_ascii_uppercase();
    if mnemonic.starts_with("ADC") {
        panic!("{} not yet implemented", mnemonic);
    }
    if mnemonic.starts_with("ADD") {
        return Box::new(nop::NOP::new())
    }
    if mnemonic.starts_with("ADR") {
        panic!("{} not yet implemented", mnemonic);
    }
    if mnemonic.starts_with("AND") {
        return Box::new(nop::NOP::new())
    }
    if mnemonic.starts_with("ASR") {
        panic!("{} not yet implemented", mnemonic);
    }
    if mnemonic.starts_with("BIC") {
        panic!("{} not yet implemented", mnemonic);
    }
    if mnemonic.starts_with("BKPT") {
        panic!("{} not yet implemented", mnemonic);
    }
    if mnemonic.starts_with("BLX") {
        panic!("{} not yet implemented", mnemonic);
    }
    if mnemonic.starts_with("BL") {
        return Box::new(nop::NOP::new())
    }
    if mnemonic.starts_with("BX") {
        panic!("{} not yet implemented", mnemonic);
    }
    if mnemonic.starts_with("B") {
        return Box::new(nop::NOP::new())
    }
    if mnemonic.starts_with("CMN") {
        panic!("{} not yet implemented", mnemonic);
    }
    if mnemonic.starts_with("CMP") {
        return Box::new(nop::NOP::new())
    }
    if mnemonic.starts_with("CPS") {
        panic!("{} not yet implemented", mnemonic);
    }
    if mnemonic.starts_with("DMB") {
        panic!("{} not yet implemented", mnemonic);
    }
    if mnemonic.starts_with("DSB") {
        panic!("{} not yet implemented", mnemonic);
    }
    if mnemonic.starts_with("EOR") {
        panic!("{} not yet implemented", mnemonic);
    }
    if mnemonic.starts_with("ISB") {
        panic!("{} not yet implemented", mnemonic);
    }
    if mnemonic.starts_with("LDM") {
        return Box::new(nop::NOP::new())
    }
    if mnemonic.starts_with("LDR") {
        return Box::new(nop::NOP::new())
    }
    if mnemonic.starts_with("LSL") {
        return Box::new(nop::NOP::new())
    }
    if mnemonic.starts_with("LSR") {
        panic!("{} not yet implemented", mnemonic);
    }
    if mnemonic.starts_with("MOV") {
        return Box::new(nop::NOP::new())
    }
    if mnemonic.starts_with("MRS") {
        panic!("{} not yet implemented", mnemonic);
    }
    if mnemonic.starts_with("MSR") {
        panic!("{} not yet implemented", mnemonic);
    }
    if mnemonic.starts_with("MUL") {
        return Box::new(nop::NOP::new())
    }
    if mnemonic.starts_with("MVN") {
        panic!("{} not yet implemented", mnemonic);
    }
    if mnemonic.starts_with("NOP") {
        return Box::new(nop::NOP::new())
    }
    if mnemonic.starts_with("ORR") {
        panic!("{} not yet implemented", mnemonic);
    }
    if mnemonic.starts_with("POP") {
        return Box::new(nop::NOP::new())
    }
    if mnemonic.starts_with("PUSH") {
        return Box::new(push::PUSH::new(operands, capstone));
    }
    if mnemonic.starts_with("REV") {
        panic!("{} not yet implemented", mnemonic);
    }
    if mnemonic.starts_with("ROR") {
        panic!("{} not yet implemented", mnemonic);
    }
    if mnemonic.starts_with("RSB") {
        panic!("{} not yet implemented", mnemonic);
    }
    if mnemonic.starts_with("SBC") {
        panic!("{} not yet implemented", mnemonic);
    }
    if mnemonic.starts_with("SEV") {
        panic!("{} not yet implemented", mnemonic);
    }
    if mnemonic.starts_with("ST") {
        return Box::new(nop::NOP::new())
    }
    if mnemonic.starts_with("SUB") {
        return Box::new(nop::NOP::new())
    }
    if mnemonic.starts_with("SVC") {
        return Box::new(svc::SVC::new(operands))
    }
    if mnemonic.starts_with("SXT") {
        panic!("{} not yet implemented", mnemonic);
    }
    if mnemonic.starts_with("TST") {
        panic!("{} not yet implemented", mnemonic);
    }
    if mnemonic.starts_with("UXT") {
        panic!("{} not yet implemented", mnemonic);
    }
    if mnemonic.starts_with("WF") {
        panic!("{} not yet implemented", mnemonic);
    }
    if mnemonic.starts_with("YIELD") {
        panic!("{} not yet implemented", mnemonic);
    }
    panic!("Unrecognised Cortex-M0 mnemonic: {}", mnemonic);
}


