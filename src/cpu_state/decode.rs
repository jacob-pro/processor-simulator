use crate::cpu_state::station::ReservationStation;
use crate::cpu_state::CpuState;
use crate::instructions::{decode_instruction, Instruction, PollResult};
use crate::CAPSTONE;
use capstone::arch::arm::{ArmCC, ArmOperand};
use capstone::arch::ArchOperand;
use capstone::{InsnDetail, RegId};
use std::collections::HashSet;

pub struct DecodedInstruction {
    pub imp: Box<dyn Instruction>,
    pub cc: ArmCC,
    pub string: String,
    pub length: u32,
    pub address: u32,
}

pub struct DecodeResults {
    pub instr: DecodedInstruction,
}

impl CpuState {
    pub fn decode(&self) -> Option<DecodeResults> {
        // Only if we have space to decode into
        if !self.decoded_space() {
            return None;
        }
        let instr = self
            .fetched_instruction
            .as_ref()
            .map(|fetched_instruction| {
                CAPSTONE.with(|capstone| {
                    let list = capstone
                        .disasm_all(&fetched_instruction.bytes, 0x0)
                        .expect("Invalid instruction");
                    match list.iter().next() {
                        // We may not get a valid instruction when speculating
                        // An InvalidInstruction is used as a placeholder
                        None => DecodedInstruction {
                            imp: Box::new(InvalidInstruction {}),
                            cc: ArmCC::ARM_CC_INVALID,
                            string: "Invalid".to_string(),
                            length: fetched_instruction.bytes.len() as u32,
                            address: fetched_instruction.address,
                        },
                        Some(instr) => {
                            let insn_detail: InsnDetail = capstone
                                .insn_detail(&instr)
                                .expect("Failed to get insn detail");
                            let arch_detail = insn_detail.arch_detail();
                            let operands: Vec<ArmOperand> = arch_detail
                                .operands()
                                .into_iter()
                                .map(|x| {
                                    if let ArchOperand::ArmOperand(inner) = x {
                                        return inner;
                                    }
                                    panic!("Unexpected ArchOperand");
                                })
                                .collect();

                            let ins_name =
                                CAPSTONE.with(|capstone| capstone.insn_name(instr.id()).unwrap());
                            let arm_detail = arch_detail.arm().unwrap();

                            let decoded = decode_instruction(&ins_name, &arm_detail, operands);
                            DecodedInstruction {
                                imp: decoded.into(),
                                cc: arm_detail.cc(),
                                string: format!(
                                    "{} {}",
                                    instr.mnemonic().unwrap(),
                                    instr.op_str().unwrap_or("")
                                ),
                                length: instr.bytes().len() as u32,
                                address: fetched_instruction.address,
                            }
                        }
                    }
                })
            });
        instr.map(|i| DecodeResults { instr: i })
    }
}

#[derive(Clone)]
struct InvalidInstruction {}

impl Instruction for InvalidInstruction {
    fn poll(&self, _: &ReservationStation) -> PollResult {
        panic!()
    }

    fn source_registers(&self) -> HashSet<RegId> {
        hashset!()
    }

    fn dest_registers(&self) -> HashSet<RegId> {
        hashset!()
    }
}
