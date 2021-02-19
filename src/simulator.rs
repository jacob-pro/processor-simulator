use crate::memory::Memory;
use crate::registers::RegisterFile;
use capstone::prelude::*;
use capstone::Insn;
use std::process::exit;
use crate::instructions::decode_instruction;

pub struct Simulator {
    memory: Memory,
    registers: RegisterFile,
    disassembler: Capstone,
}

impl Simulator {

    pub fn new(memory: Memory, pc: u32) -> Self {
        let cs = Capstone::new()
            .arm()
            .mode(arch::arm::ArchMode::Thumb)
            .endian(capstone::Endian::Little)
            .detail(true)
            .build()
            .unwrap();
        let mut registers=  RegisterFile::default();
        registers.pc = pc;
        registers.sp = std::u32::MAX;
        Self {
            memory,
            registers,
            disassembler: cs,
        }
    }

    pub fn run(&mut self) {
        let mut cycle_counter = 0;
        loop {
            cycle_counter = cycle_counter + 1;
            let instr_bytes = self.fetch();
            let instr_decoded = self.decode(instr_bytes.as_slice());
        }
    }

    fn fetch(&mut self) -> Vec<u8> {
        /*  The Thumb instruction stream is a sequence of halfword-aligned halfwords.
            Each Thumb instruction is either a single 16-bit halfword in that stream,
            or a 32-bit instruction consisting of two consecutive halfwords in that stream.
            If bits [15:11] of the halfword being decoded take any of the following values,
            the halfword is the first halfword of a 32-bit instruction:
            0b11101 0b11110 0b11111 Otherwise, the halfword is a 16-bit instruction.
         */
        let pc = self.registers.pc;
        let code = self.memory.read_bytes(pc, 4);
        let bits_15_11 = code[1] >> 3;
        let instr_len = match bits_15_11 {
            0b11101 | 0b11110 | 0b11111 => 4,
            _ => 2
        };
        assert!(instr_len == 2 || instr_len == 4);
        self.registers.pc = self.registers.pc + instr_len;
        code[0..instr_len as usize].to_vec()
    }

    fn decode(&self, instr: &[u8]) {
        let list = self.disassembler.disasm_all(instr, 0x0).expect("Invalid instruction");
        let instr = list.iter().next().unwrap();
        let opcode = instr.mnemonic().unwrap();
        println!("{} {}", opcode, instr.op_str().unwrap_or(""));
        decode_instruction(opcode);
    }

}
