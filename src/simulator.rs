use crate::memory::Memory;
use crate::registers::RegisterFile;
use capstone::prelude::*;
use capstone::Insn;

pub struct Simulator {
    memory: Memory,
    registers: RegisterFile,
    disassembler: Capstone,
}

impl Simulator {

    pub fn new(memory: Memory) -> Self {
        let cs = Capstone::new()
            .arm()
            .mode(arch::arm::ArchMode::Thumb)
            .endian(capstone::Endian::Little)
            .detail(true)
            .build()
            .unwrap();
        Self {
            memory,
            registers: Default::default(),
            disassembler: cs,
        }
    }

    pub fn run(&mut self) {
        let mut cycle_counter = 0;
        loop {
            cycle_counter = cycle_counter + 1;
            let ins = self.fetch();
            println!("{} {}", ins.0, ins.1);
        }
    }

    fn fetch(&mut self) -> (String, String) {
        let pc = self.registers.pc;
        let code = self.memory.read_bytes(pc, 4);
        let list = self.disassembler.disasm_count(code, 0x0, 1).expect("Invalid instruction");
        let instr = list.iter().next().unwrap();
        let instr_len = instr.bytes().len() as u32;
        assert!(instr_len == 2 || instr_len == 4);
        self.registers.pc = self.registers.pc + instr_len;
        let mut mnemonic = instr.mnemonic().unwrap().to_owned();
        let operand = instr.op_str().unwrap_or("").to_owned();
        (mnemonic, operand)
    }

}