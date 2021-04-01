use crate::cpu_state::CpuState;

pub struct FetchChanges {
    next_addr: u32,
    instruction: Vec<u8>,
}

impl FetchChanges {
    pub fn apply(self, sim: &mut CpuState) {
        sim.fetched_instruction = Some(self.instruction);
        sim.fetched_instr_addr = Some(sim.next_instr_addr);
        sim.next_instr_addr = self.next_addr;
    }
}

impl CpuState {
    pub fn fetch(&self) -> FetchChanges {
        /*  The Thumb instruction stream is a sequence of halfword-aligned halfwords.
           Each Thumb instruction is either a single 16-bit halfword in that stream,
           or a 32-bit instruction consisting of two consecutive halfwords in that stream.
           If bits [15:11] of the halfword being decoded take any of the following values,
           the halfword is the first halfword of a 32-bit instruction:
           0b11101 0b11110 0b11111 Otherwise, the halfword is a 16-bit instruction.
        */
        assert_eq!(
            self.next_instr_addr & 1,
            1,
            "LSB of PC must be 1 for thumb mode"
        );
        let addr = self.next_instr_addr & 0xFFFFFFFE; // Ignore the last bit for actual address
        let code = self.memory.read().unwrap().read_bytes(addr, 4);
        let bits_15_11 = code[1] >> 3;
        let instr_len = match bits_15_11 {
            0b11101 | 0b11110 | 0b11111 => 4,
            _ => 2,
        };
        assert!(instr_len == 2 || instr_len == 4);
        FetchChanges {
            next_addr: self.next_instr_addr + instr_len,
            instruction: code[0..instr_len as usize].to_vec(),
        }
    }
}
