use crate::cpu_state::execute::ExecuteChanges;
use crate::cpu_state::{CpuState, DecodedInstruction};
use crate::instructions::{decode_instruction, Instruction};
use crate::CAPSTONE;
use capstone::arch::arm::{ArmCC, ArmOperand};
use capstone::arch::ArchOperand;
use capstone::InsnDetail;
use std::rc::Rc;

pub struct DecodeChanges(Option<DecodedInstruction>);

impl DecodeChanges {
    pub fn apply(self, sim: &mut CpuState) {
        match self.0.as_ref() {
            None => {}
            Some(x) => {
                sim.registers.cur_instr_len = Some(x.length);
            }
        }
        sim.decoded_instruction = self.0;
    }
}

impl CpuState {
    pub fn decode(&self) -> DecodeChanges {
        DecodeChanges(
            self.fetched_instruction
                .as_ref()
                .map(|fetched_instruction| {
                    CAPSTONE.with(|capstone| {
                        let list = capstone
                            .disasm_all(fetched_instruction, 0x0)
                            .expect("Invalid instruction");
                        match list.iter().next() {
                            None => DecodedInstruction {
                                imp: Rc::new(InvalidInstruction {}),
                                cc: ArmCC::ARM_CC_INVALID,
                                string: "Invalid".to_string(),
                                length: fetched_instruction.len() as u32,
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

                                let ins_name = CAPSTONE
                                    .with(|capstone| capstone.insn_name(instr.id()).unwrap());
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
                                }
                            }
                        }
                    })
                }),
        )
    }
}

struct InvalidInstruction {}

impl Instruction for InvalidInstruction {
    fn execute(&self, _: &CpuState, _: &mut ExecuteChanges) -> bool {
        panic!()
    }
}
