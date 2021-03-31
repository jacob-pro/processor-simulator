use crate::instructions::{decode_instruction, Instruction};
use crate::memory::Memory;
use crate::registers::RegisterFile;
use crate::{DebugLevel, CAPSTONE};
use capstone::arch::arm::{ArmCC, ArmOperand};
use capstone::arch::ArchOperand;
use capstone::prelude::*;
use std::time::Instant;
use std::sync::{RwLock, Arc};

pub struct Simulator {
    pub memory: Arc<RwLock<Memory>>,
    pub registers: RegisterFile,
}

impl Simulator {
    pub fn new(memory: Arc<RwLock<Memory>>, entry: u32) -> Self {
        let registers = RegisterFile::new(entry);
        Self { memory, registers }
    }

    pub fn run(&mut self, debug: DebugLevel) {
        let start_time = Instant::now();
        let mut cycle_counter = 0;
        loop {
            cycle_counter = cycle_counter + 1;
            let instr_bytes = self.fetch();
            let dec = self.decode(instr_bytes.as_slice());
            let ex = self.should_execute(&dec.cc);
            if debug >= DebugLevel::Minimal {
                let mut output = String::new();
                if ex {
                    output.push_str(&dec.string);
                } else {
                    output.push_str(&format!("{} (omitted)", dec.string));
                }
                if debug >= DebugLevel::Full {
                    let padding: String = vec![' '; 30 as usize - output.len()].iter().collect();
                    output.push_str(&format!("{} [{}]", padding, self.registers.debug_string()));
                }
                println!("{}", output);
            }
            if ex && dec.imp.execute(self) {
                break;
            }
            self.registers.pc = self.registers.real_pc;
        }
        let elapsed = start_time.elapsed();
        println!(
            "Simulator run {} cycles in {} seconds",
            cycle_counter,
            elapsed.as_millis() as f64 / 1000.0
        );
    }

    fn fetch(&mut self) -> Vec<u8> {
        /*  The Thumb instruction stream is a sequence of halfword-aligned halfwords.
           Each Thumb instruction is either a single 16-bit halfword in that stream,
           or a 32-bit instruction consisting of two consecutive halfwords in that stream.
           If bits [15:11] of the halfword being decoded take any of the following values,
           the halfword is the first halfword of a 32-bit instruction:
           0b11101 0b11110 0b11111 Otherwise, the halfword is a 16-bit instruction.
        */
        assert_eq!(
            self.registers.real_pc & 1,
            1,
            "LSB of PC must be 1 for thumb mode"
        );
        let addr = self.registers.real_pc & 0xFFFFFFFE; // Ignore the last bit for actual address
        let code = self.memory.read().unwrap().read_bytes(addr, 4);
        let bits_15_11 = code[1] >> 3;
        let instr_len = match bits_15_11 {
            0b11101 | 0b11110 | 0b11111 => 4,
            _ => 2,
        };
        assert!(instr_len == 2 || instr_len == 4);
        self.registers.real_pc = self.registers.pc + instr_len;
        // The PC is a liar (sometimes)!
        // The PC offset is always 4 bytes in Thumb state
        self.registers.pc = self.registers.pc + 4;
        code[0..instr_len as usize].to_vec()
    }

    fn decode(&self, instr: &[u8]) -> DecodedInstruction {
        CAPSTONE.with(|capstone| {
            let list = capstone
                .disasm_all(instr, 0x0)
                .expect("Invalid instruction");
            let instr = list.iter().next().unwrap();
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

            let ins_name = CAPSTONE.with(|capstone| capstone.insn_name(instr.id()).unwrap());
            let arm_detail = arch_detail.arm().unwrap();

            let decoded = decode_instruction(&ins_name, &arm_detail, operands);
            DecodedInstruction {
                imp: decoded,
                cc: arm_detail.cc(),
                string: format!(
                    "{} {}",
                    instr.mnemonic().unwrap(),
                    instr.op_str().unwrap_or("")
                ),
            }
        })
    }

    // https://community.arm.com/developer/ip-products/processors/b/processors-ip-blog/posts/condition-codes-1-condition-flags-and-codes
    fn should_execute(&self, cc: &ArmCC) -> bool {
        let flags = &self.registers.cond_flags;
        return match cc {
            ArmCC::ARM_CC_INVALID => panic!("CC Invalid"),
            ArmCC::ARM_CC_EQ => flags.z == true,
            ArmCC::ARM_CC_NE => flags.z == false,
            ArmCC::ARM_CC_HS => flags.c == true,
            ArmCC::ARM_CC_LO => flags.c == false,
            ArmCC::ARM_CC_MI => flags.n == true,
            ArmCC::ARM_CC_PL => flags.n == false,
            ArmCC::ARM_CC_VS => flags.v == true,
            ArmCC::ARM_CC_VC => flags.v == false,
            ArmCC::ARM_CC_HI => flags.c == true && flags.z == false,
            ArmCC::ARM_CC_LS => flags.c == false || flags.z == true,
            ArmCC::ARM_CC_GE => flags.n == flags.v,
            ArmCC::ARM_CC_LT => flags.n != flags.v,
            ArmCC::ARM_CC_GT => flags.z == false && flags.n == flags.v,
            ArmCC::ARM_CC_LE => flags.z == true || flags.n != flags.v,
            ArmCC::ARM_CC_AL => true,
        };
    }
}

struct DecodedInstruction {
    imp: Box<dyn Instruction>,
    cc: ArmCC,
    string: String,
}
