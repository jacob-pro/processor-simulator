mod nop;
mod push;
mod pop;
mod svc;
mod util;
mod mov;
mod b;
mod bx;
mod add;
mod cmp;
mod ldm;
mod shift;
mod ldr;
mod stm;
mod str;
mod extends;
mod tst;
mod logical;
mod mul;
mod adr;

use capstone::arch::arm::{ArmOperand, ArmInsnDetail};
use crate::simulator::Simulator;

pub type ShouldTerminate = bool;

pub trait Instruction {
    fn execute(&self, sim: &mut Simulator) -> ShouldTerminate;
}

/*
https://en.wikipedia.org/wiki/ARM_Cortex-M#Instruction_sets
https://developer.arm.com/documentation/dui0497/a/the-cortex-m0-instruction-set/instruction-set-summary?lang=en
 */
pub fn decode_instruction(name: &str,
                          detail: &ArmInsnDetail,
                          operands: Vec<ArmOperand>) -> Box<dyn Instruction> {
    let update_flags = detail.update_flags();
    let writeback = detail.writeback();
    return match name.to_ascii_uppercase().as_str() {
        "ADC" => Box::new(add::ADD::new(operands, update_flags, add::Mode::ADC)),
        "ADD" => Box::new(add::ADD::new(operands, update_flags, add::Mode::ADD)),
        "ADR" => Box::new(adr::ADR::new(operands)),
        "AND" => Box::new(logical::LOGICAL::new(operands, logical::Mode::AND)),
        "ASR" => Box::new(shift::SHIFT::new(operands, shift::Mode::ASR)),
        "B" => Box::new(b::B::new(operands, false)),
        "BIC" => Box::new(logical::LOGICAL::new(operands, logical::Mode::BIC)),
        "BKPT" => panic!("{} not yet implemented", name),
        "BL" => Box::new(b::B::new(operands, true)),
        "BLX" => Box::new(bx::BX::new(operands, true)),
        "BX" => Box::new(bx::BX::new(operands, false)),
        "CMN" => Box::new(cmp::CMP::new(operands, cmp::Mode::CMN)),
        "CMP" => Box::new(cmp::CMP::new(operands, cmp::Mode::CMP)),
        "CPS" => panic!("{} not yet implemented", name),
        "DMB" => panic!("{} not yet implemented", name),
        "DSB" => panic!("{} not yet implemented", name),
        "EOR" => Box::new(logical::LOGICAL::new(operands,  logical::Mode::EOR)),
        "ISB" => panic!("{} not yet implemented", name),
        "LDM" => Box::new(ldm::LDM::new(operands, writeback)),
        "LDR" => Box::new(ldr::LDR::new(operands, ldr::Mode::Word)),
        "LDRB" => Box::new(ldr::LDR::new(operands, ldr::Mode::Byte)),
        "LDRH" => Box::new(ldr::LDR::new(operands, ldr::Mode::HalfWord)),
        "LDRSB" => Box::new(ldr::LDR::new(operands, ldr::Mode::SignedByte)),
        "LDRSH" => Box::new(ldr::LDR::new(operands, ldr::Mode::SignedHalfWord)),
        "LSL" => Box::new(shift::SHIFT::new(operands, shift::Mode::LSL)),
        "LSR" => Box::new(shift::SHIFT::new(operands, shift::Mode::LSR)),
        "MOV" => Box::new(mov::MOV::new(operands, mov::Mode::MOV, update_flags)),
        "MRS" => panic!("{} not yet implemented", name),
        "MSR" => panic!("{} not yet implemented", name),
        "MUL" => Box::new(mul::MUL::new(operands)),
        "MVN" => Box::new(mov::MOV::new(operands, mov::Mode::MVN, update_flags)),
        "NOP" => Box::new(nop::NOP::new()),
        "ORR" => Box::new(logical::LOGICAL::new(operands,  logical::Mode::ORR)),
        "POP" => Box::new(pop::POP::new(operands)),
        "PUSH" => Box::new(push::PUSH::new(operands)),
        "REV" => panic!("{} not yet implemented", name),
        "REV16" => panic!("{} not yet implemented", name),
        "REVSH" => panic!("{} not yet implemented", name),
        "ROR" => Box::new(shift::SHIFT::new(operands, shift::Mode::ROR)),
        "RSB" => Box::new(add::ADD::new(operands, update_flags, add::Mode::RSB)),
        "SBC" => Box::new(add::ADD::new(operands, update_flags, add::Mode::SBC)),
        "SEV" => panic!("{} not yet implemented", name),
        "STM" => Box::new(stm::STM::new(operands, writeback)),
        "STR" => Box::new(str::STR::new(operands, str::Mode::Word)),
        "STRB" => Box::new(str::STR::new(operands, str::Mode::Byte)),
        "STRH" => Box::new(str::STR::new(operands, str::Mode::HalfWord)),
        "SUB" => Box::new(add::ADD::new(operands, update_flags, add::Mode::SUB)),
        "SVC" => Box::new(svc::SVC::new(operands)),
        "SXTB" => Box::new(extends::EXTENDS::new(operands, extends::Mode::SXTB)),
        "SXTH" => Box::new(extends::EXTENDS::new(operands, extends::Mode::SXTH)),
        "TST" => Box::new(tst::TST::new(operands)),
        "UXTB" => Box::new(extends::EXTENDS::new(operands, extends::Mode::UXTB)),
        "UXTH" => Box::new(extends::EXTENDS::new(operands, extends::Mode::UXTH)),
        "WFE" => panic!("{} not yet implemented", name),
        "WFI" => panic!("{} not yet implemented", name),
        "YIELD" => panic!("{} not yet implemented", name),
        _ => panic!("Unrecognised Cortex-M0 instruction: {}", name),
    }
}
