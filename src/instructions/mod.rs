mod add;
mod adr;
mod b;
mod bx;
mod cmp;
mod extends;
mod ldm;
mod ldr;
mod logical;
mod mov;
mod mul;
mod nop;
mod pop;
mod push;
mod shift;
mod stm;
mod str;
mod svc;
mod tst;
mod util;

use crate::cpu_state::station::ReservationStation;
use crate::registers::ids::PC;
use capstone::arch::arm::{ArmInsnDetail, ArmOperand};
use capstone::RegId;
use std::collections::HashSet;
use std::fmt::Debug;

#[derive(Debug)]
pub enum PollResult {
    Complete(Vec<(RegId, u32)>),
    Again(Box<dyn Instruction>),
    Exception,
}

pub trait Instruction: Send + Sync + Debug {
    fn poll(&self, station: &ReservationStation) -> PollResult;

    fn source_registers(&self) -> HashSet<RegId>;

    fn dest_registers(&self) -> HashSet<RegId>;

    fn hazardous(&self) -> bool {
        self.dest_registers().contains(&PC)
    }
}

#[derive(Debug, Clone)]
pub enum DecodeError {
    Unimplemented(String),
    UnsupportedInCortexM0(String),
}

/*
https://en.wikipedia.org/wiki/ARM_Cortex-M#Instruction_sets
https://developer.arm.com/documentation/dui0497/a/the-cortex-m0-instruction-set/instruction-set-summary?lang=en
 */
pub fn decode_instruction(
    name: &str,
    detail: &ArmInsnDetail,
    operands: Vec<ArmOperand>,
) -> Result<Box<dyn Instruction>, DecodeError> {
    let update_flags = detail.update_flags();
    let writeback = detail.writeback();
    return Ok(match name.to_ascii_uppercase().as_str() {
        "ADC" => Box::new(add::ADD::new(operands, update_flags, add::Mode::ADC)),
        "ADD" => Box::new(add::ADD::new(operands, update_flags, add::Mode::ADD)),
        "ADR" => Box::new(adr::ADR::new(operands)),
        "AND" => Box::new(logical::LOGICAL::new(operands, logical::Mode::AND)),
        "ASR" => Box::new(shift::SHIFT::new(operands, shift::Mode::ASR)),
        "B" => Box::new(b::B::new(operands, false)),
        "BIC" => Box::new(logical::LOGICAL::new(operands, logical::Mode::BIC)),
        "BKPT" => return Err(DecodeError::Unimplemented(name.to_owned())),
        "BL" => Box::new(b::B::new(operands, true)),
        "BLX" => Box::new(bx::BX::new(operands, true)),
        "BX" => Box::new(bx::BX::new(operands, false)),
        "CMN" => Box::new(cmp::CMP::new(operands, cmp::Mode::CMN)),
        "CMP" => Box::new(cmp::CMP::new(operands, cmp::Mode::CMP)),
        "CPS" => return Err(DecodeError::Unimplemented(name.to_owned())),
        "DMB" => return Err(DecodeError::Unimplemented(name.to_owned())),
        "DSB" => return Err(DecodeError::Unimplemented(name.to_owned())),
        "EOR" => Box::new(logical::LOGICAL::new(operands, logical::Mode::EOR)),
        "ISB" => return Err(DecodeError::Unimplemented(name.to_owned())),
        "LDM" => Box::new(ldm::LDM::new(operands, writeback)),
        "LDR" => Box::new(ldr::LDR::new(operands, ldr::Mode::Word)),
        "LDRB" => Box::new(ldr::LDR::new(operands, ldr::Mode::Byte)),
        "LDRH" => Box::new(ldr::LDR::new(operands, ldr::Mode::HalfWord)),
        "LDRSB" => Box::new(ldr::LDR::new(operands, ldr::Mode::SignedByte)),
        "LDRSH" => Box::new(ldr::LDR::new(operands, ldr::Mode::SignedHalfWord)),
        "LSL" => Box::new(shift::SHIFT::new(operands, shift::Mode::LSL)),
        "LSR" => Box::new(shift::SHIFT::new(operands, shift::Mode::LSR)),
        "MOV" => Box::new(mov::MOV::new(operands, mov::Mode::MOV, update_flags)),
        "MRS" => return Err(DecodeError::Unimplemented(name.to_owned())),
        "MSR" => return Err(DecodeError::Unimplemented(name.to_owned())),
        "MUL" => Box::new(mul::MUL::new(operands)),
        "MVN" => Box::new(mov::MOV::new(operands, mov::Mode::MVN, update_flags)),
        "NOP" => Box::new(nop::NOP::new()),
        "ORR" => Box::new(logical::LOGICAL::new(operands, logical::Mode::ORR)),
        "POP" => Box::new(pop::POP::new(operands)),
        "PUSH" => Box::new(push::PUSH::new(operands)),
        "REV" => return Err(DecodeError::Unimplemented(name.to_owned())),
        "REV16" => return Err(DecodeError::Unimplemented(name.to_owned())),
        "REVSH" => return Err(DecodeError::Unimplemented(name.to_owned())),
        "ROR" => Box::new(shift::SHIFT::new(operands, shift::Mode::ROR)),
        "RSB" => Box::new(add::ADD::new(operands, update_flags, add::Mode::RSB)),
        "SBC" => Box::new(add::ADD::new(operands, update_flags, add::Mode::SBC)),
        "SEV" => return Err(DecodeError::Unimplemented(name.to_owned())),
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
        "WFE" => return Err(DecodeError::Unimplemented(name.to_owned())),
        "WFI" => return Err(DecodeError::Unimplemented(name.to_owned())),
        "YIELD" => return Err(DecodeError::Unimplemented(name.to_owned())),
        _ => return Err(DecodeError::UnsupportedInCortexM0(name.to_owned())),
    });
}
