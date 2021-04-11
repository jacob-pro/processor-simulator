use crate::CAPSTONE;
use capstone::arch::arm::ArmCC;
use capstone::arch::arm::{ArmOpMem, ArmOperand, ArmOperandType, ArmShift};
use capstone::prelude::*;

#[allow(unused)]
pub mod ids {
    use capstone::RegId;
    pub const LR: RegId = RegId(10);
    pub const PC: RegId = RegId(11);
    pub const SP: RegId = RegId(12);
    pub const R0: RegId = RegId(66);
    pub const R1: RegId = RegId(67);
    pub const R2: RegId = RegId(68);
    pub const R3: RegId = RegId(69);
    pub const R4: RegId = RegId(70);
    pub const R5: RegId = RegId(71);
    pub const R6: RegId = RegId(72);
    pub const R7: RegId = RegId(73);
    pub const R8: RegId = RegId(74);
    pub const SB: RegId = RegId(75);
    pub const SL: RegId = RegId(76);
    pub const FP: RegId = RegId(77);
    pub const IP: RegId = RegId(78);
    pub const CPSR: RegId = RegId(3);
}

pub enum ConditionFlag {
    N,
    Z,
    C,
    V,
}

impl ConditionFlag {
    fn pos(&self) -> u32 {
        match self {
            ConditionFlag::N => 31,
            ConditionFlag::Z => 30,
            ConditionFlag::C => 29,
            ConditionFlag::V => 28,
        }
    }

    pub fn write_flag(&self, cpsr: u32, value: bool) -> u32 {
        if value {
            let mask = 1 << self.pos();
            cpsr | mask
        } else {
            let mask = !(1 << self.pos());
            cpsr & mask
        }
    }

    pub fn read_flag(&self, cpsr: u32) -> bool {
        cpsr & (1 << self.pos()) > 0
    }
}

#[derive(Default, Debug, Clone)]
pub struct ConditionFlags {
    n: bool, // Negative
    z: bool, // Zero
    c: bool, // Carry
    v: bool, // Overflow
}

#[derive(Debug, Clone)]
pub struct RegisterFile {
    gprs: [u32; 13],
    sp: u32,
    lr: u32,
    pc: u32,
    cpsr: u32,
}

impl RegisterFile {
    pub fn new() -> Self {
        Self {
            gprs: Default::default(),
            sp: crate::_STACK,
            lr: 0,
            pc: 0,
            cpsr: 0,
        }
    }

    pub fn read_by_id(&self, id: RegId) -> u32 {
        let name = Self::reg_name(id);
        if name.starts_with("R") {
            let number = name[1..].parse::<usize>().expect("Invalid register");
            return self.gprs[number];
        }
        return match name.as_str() {
            "SB" => self.gprs[9],  // Synonym
            "SL" => self.gprs[10], // Synonym
            "FP" => self.gprs[11], // Synonym
            "IP" => self.gprs[12], // Synonym
            "SP" => self.sp,
            "LR" => self.lr,
            "CPSR" => self.cpsr,
            _ => panic!("Unknown register {}", name),
        };
    }

    pub fn write_by_id(&mut self, id: RegId, value: u32) {
        let name = Self::reg_name(id);
        if name.starts_with("R") {
            let number = name[1..].parse::<usize>().expect("Invalid register");
            self.gprs[number] = value;
            return;
        }
        match name.as_str() {
            "SB" => self.gprs[9] = value,  // Synonym
            "SL" => self.gprs[10] = value, // Synonym
            "FP" => self.gprs[11] = value, // Synonym
            "IP" => self.gprs[12] = value, // Synonym
            "SP" => self.sp = value,
            "LR" => self.lr = value,
            "PC" => self.pc = value,
            "CPSR" => self.cpsr = value,
            _ => panic!("Unknown register {}", name),
        }
    }

    /*
     *  reglist must use only R0-R7. The exception is LR for a PUSH and PC for a POP.
     * lowest numbered register using the lowest memory address
     */
    pub fn push_pop_register_asc(mut reg_list: Vec<RegId>) -> Vec<RegId> {
        reg_list.sort_by_key(|r| {
            let name = Self::reg_name(*r);
            if name.starts_with("R") {
                let number = name[1..].parse::<usize>().expect("Invalid register");
                if number <= 7 {
                    return number;
                }
            }
            return match name.as_str() {
                "LR" => 14,
                "PC" => 15,
                _ => panic!("Unknown register {}", name),
            };
        });
        reg_list
    }

    pub fn debug_string(&self) -> String {
        let mut output = String::new();
        for i in 0..8 {
            output.push_str(&format!("R{} {:08X} ", i, self.gprs[i]));
        }
        output.push_str(&format!("LR {:08X} ", self.lr));
        output.push_str(&format!("PC {:08X} ", self.pc & 0xFFFFFFFE));
        output.push_str(&format!("SP {:08X} ", self.sp));
        output.push_str(&format!("N{}", ConditionFlag::N.read_flag(self.cpsr) as u8));
        output.push_str(&format!("Z{}", ConditionFlag::Z.read_flag(self.cpsr) as u8));
        output.push_str(&format!("C{}", ConditionFlag::C.read_flag(self.cpsr) as u8));
        output.push_str(&format!("V{}", ConditionFlag::V.read_flag(self.cpsr) as u8));
        output
    }

    #[inline]
    fn reg_name(reg_id: RegId) -> String {
        CAPSTONE.with(|capstone| {
            capstone
                .reg_name(reg_id)
                .expect("Couldn't get reg_name")
                .to_ascii_uppercase()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn reg_names() {
        assert_eq!(RegisterFile::reg_name(R0), "R0");
        assert_eq!(RegisterFile::reg_name(R1), "R1");
        assert_eq!(RegisterFile::reg_name(R2), "R2");
        assert_eq!(RegisterFile::reg_name(R3), "R3");
        assert_eq!(RegisterFile::reg_name(R4), "R4");
        assert_eq!(RegisterFile::reg_name(R5), "R5");
        assert_eq!(RegisterFile::reg_name(R6), "R6");
        assert_eq!(RegisterFile::reg_name(R7), "R7");
        assert_eq!(RegisterFile::reg_name(R8), "R8");
        assert_eq!(RegisterFile::reg_name(SB), "SB");
        assert_eq!(RegisterFile::reg_name(SL), "SL");
        assert_eq!(RegisterFile::reg_name(FP), "FP");
        assert_eq!(RegisterFile::reg_name(IP), "IP");
        assert_eq!(RegisterFile::reg_name(LR), "LR");
        assert_eq!(RegisterFile::reg_name(PC), "PC");
        assert_eq!(RegisterFile::reg_name(SP), "SP");
        assert_eq!(RegisterFile::reg_name(CPSR), "CPSR");
    }

    #[test]
    fn flags() {
        let cpsr = 0b1010_0000 << 24;
        assert!(ConditionFlag::N.read_flag(cpsr));
        assert!(!ConditionFlag::Z.read_flag(cpsr));
        assert!(ConditionFlag::C.read_flag(cpsr));
        assert!(!ConditionFlag::V.read_flag(cpsr));

        let mut cpsr = 0;
        cpsr = ConditionFlag::N.write_flag(cpsr, false);
        assert!(!ConditionFlag::N.read_flag(cpsr));
        cpsr = ConditionFlag::N.write_flag(cpsr, true);
        assert!(ConditionFlag::N.read_flag(cpsr));
        cpsr = ConditionFlag::N.write_flag(cpsr, false);
        assert!(!ConditionFlag::N.read_flag(cpsr));
    }
}
