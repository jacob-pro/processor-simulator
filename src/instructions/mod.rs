mod nop;
mod push;
mod svc;
mod util;

use capstone::arch::arm::ArmOperand;
use crate::simulator::Simulator;

pub type ShouldTerminate = bool;

pub trait Instruction {
    fn execute(&self, sim: &mut Simulator) -> ShouldTerminate;
}

pub fn decode_instruction(name: &str,
                          update_flags: bool,
                          operands: Vec<ArmOperand>) -> Box<dyn Instruction> {
    return match name.to_ascii_uppercase().as_str() {
        "ADC" => panic!("{} not yet implemented", name),
        "ADD" => Box::new(nop::NOP::new()),
        "ADR" => panic!("{} not yet implemented", name),
        "AND" => panic!("{} not yet implemented", name),
        "ASR" => panic!("{} not yet implemented", name),
        "B" => Box::new(nop::NOP::new()),
        "BIC" => panic!("{} not yet implemented", name),
        "BKPT" => panic!("{} not yet implemented", name),
        "BL" => panic!("{} not yet implemented", name),
        "BLX" => panic!("{} not yet implemented", name),
        "BX" => panic!("{} not yet implemented", name),
        "CMN" => panic!("{} not yet implemented", name),
        "CMP" => Box::new(nop::NOP::new()),
        "CPS" => panic!("{} not yet implemented", name),
        "DMB" => panic!("{} not yet implemented", name),
        "DSB" => panic!("{} not yet implemented", name),
        "EOR" => panic!("{} not yet implemented", name),
        "ISB" => panic!("{} not yet implemented", name),
        "LDM" => Box::new(nop::NOP::new()),
        "LDR" => Box::new(nop::NOP::new()),
        "LDRB" => panic!("{} not yet implemented", name),
        "LDRH" => panic!("{} not yet implemented", name),
        "LDRSB" => panic!("{} not yet implemented", name),
        "LDRSH" => panic!("{} not yet implemented", name),
        "LSL" => Box::new(nop::NOP::new()),
        "LSR" => panic!("{} not yet implemented", name),
        "MOV" => Box::new(nop::NOP::new()),
        "MRS" => panic!("{} not yet implemented", name),
        "MSR" => panic!("{} not yet implemented", name),
        "MUL" => panic!("{} not yet implemented", name),
        "MVN" => panic!("{} not yet implemented", name),
        "NOP" => Box::new(nop::NOP::new()),
        "ORR" => panic!("{} not yet implemented", name),
        "POP" => panic!("{} not yet implemented", name),
        "PUSH" => Box::new(push::PUSH::new(operands)),
        "REV" => panic!("{} not yet implemented", name),
        "REV16" => panic!("{} not yet implemented", name),
        "REVSH" => panic!("{} not yet implemented", name),
        "ROR" => panic!("{} not yet implemented", name),
        "RSB" => panic!("{} not yet implemented", name),
        "SBC" => panic!("{} not yet implemented", name),
        "SEV" => panic!("{} not yet implemented", name),
        "STM" => Box::new(nop::NOP::new()),
        "STR" => Box::new(nop::NOP::new()),
        "STRB" => panic!("{} not yet implemented", name),
        "STRH" => panic!("{} not yet implemented", name),
        "SUB" => Box::new(nop::NOP::new()),
        "SVC" => Box::new(svc::SVC::new(operands)),
        "SXTB" => panic!("{} not yet implemented", name),
        "SXTH" => panic!("{} not yet implemented", name),
        "TST" => panic!("{} not yet implemented", name),
        "UXTB" => panic!("{} not yet implemented", name),
        "UXTH" => panic!("{} not yet implemented", name),
        "WFE" => panic!("{} not yet implemented", name),
        "WFI" => panic!("{} not yet implemented", name),
        "YIELD" => panic!("{} not yet implemented", name),
        _ => panic!("Unrecognised Cortex-M0 instruction: {}", name),
    }
}


