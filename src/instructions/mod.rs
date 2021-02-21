mod nop;
mod push;
mod svc;
mod util;
mod mov;
mod b;
mod add;
mod sub;
mod cmp;
mod ldm;
mod lsl;
mod ldr;
mod stm;
mod str;

use capstone::arch::arm::{ArmOperand, ArmInsnDetail};
use crate::simulator::Simulator;

pub type ShouldTerminate = bool;

pub trait Instruction {
    fn execute(&self, sim: &mut Simulator) -> ShouldTerminate;
}

/*
https://en.wikipedia.org/wiki/ARM_Cortex-M#Instruction_sets
https://developer.arm.com/documentation/dui0497/a/the-cortex-m0-instruction-set/instruction-set-summary?lang=en
https://www.keil.com/support/man/docs/armasm/armasm_dom1361289850039.htm
 */
pub fn decode_instruction(name: &str,
                          detail: &ArmInsnDetail,
                          operands: Vec<ArmOperand>) -> Box<dyn Instruction> {
    let update_flags = detail.update_flags();
    let writeback = detail.writeback();
    return match name.to_ascii_uppercase().as_str() {
        "ADC" => panic!("{} not yet implemented", name),
        "ADD" => Box::new(add::ADD::new(operands, update_flags)),
        "ADR" => panic!("{} not yet implemented", name),
        "AND" => panic!("{} not yet implemented", name),
        "ASR" => panic!("{} not yet implemented", name),
        "B" => Box::new(b::B::new(operands, false)),
        "BIC" => panic!("{} not yet implemented", name),
        "BKPT" => panic!("{} not yet implemented", name),
        "BL" => Box::new(b::B::new(operands, true)),
        "BLX" => panic!("{} not yet implemented", name),
        "BX" => panic!("{} not yet implemented", name),
        "CMN" => Box::new(cmp::CMP::new(operands, true)),
        "CMP" => Box::new(cmp::CMP::new(operands, false)),
        "CPS" => panic!("{} not yet implemented", name),
        "DMB" => panic!("{} not yet implemented", name),
        "DSB" => panic!("{} not yet implemented", name),
        "EOR" => panic!("{} not yet implemented", name),
        "ISB" => panic!("{} not yet implemented", name),
        "LDM" => Box::new(ldm::LDM::new(operands, writeback)),
        "LDR" => Box::new(ldr::LDR::new(operands)),
        "LDRB" => panic!("{} not yet implemented", name),
        "LDRH" => panic!("{} not yet implemented", name),
        "LDRSB" => panic!("{} not yet implemented", name),
        "LDRSH" => panic!("{} not yet implemented", name),
        "LSL" => Box::new(lsl::LSL::new(operands)),
        "LSR" => panic!("{} not yet implemented", name),
        "MOV" => Box::new(mov::MOV::new(operands, update_flags)),
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
        "STM" => Box::new(stm::STM::new(operands, writeback)),
        "STR" => Box::new(str::STR::new(operands)),
        "STRB" => panic!("{} not yet implemented", name),
        "STRH" => panic!("{} not yet implemented", name),
        "SUB" => Box::new(sub::SUB::new(operands, update_flags)),
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
