use crate::CAPSTONE;
use capstone::arch::arm::{ArmOpMem, ArmOperand, ArmOperandType, ArmShift};
use capstone::prelude::*;

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

pub enum ConditionFlag {
    N,
    Z,
    C,
    V,
}

#[derive(Default, Debug, Clone)]
pub struct ConditionFlags {
    pub n: bool, // Negative
    pub z: bool, // Zero
    pub c: bool, // Carry
    pub v: bool, // Overflow
}

impl ConditionFlags {
    pub fn write_flag(&mut self, flag: ConditionFlag, value: bool) {
        match flag {
            ConditionFlag::N => self.n = value,
            ConditionFlag::Z => self.z = value,
            ConditionFlag::C => self.c = value,
            ConditionFlag::V => self.v = value,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RegisterFile {
    pub gprs: [u32; 13],
    pub sp: u32,
    pub lr: u32,
    pub pc: u32,
    pub cond_flags: ConditionFlags,
    pub next_instr_len: Option<u32>,
    pub cur_instr_len: Option<u32>,
    pub changed_pc: bool,
}

impl RegisterFile {
    pub fn new(pc: u32) -> Self {
        Self {
            gprs: Default::default(),
            sp: crate::_STACK,
            lr: 0,
            pc: pc,
            cond_flags: Default::default(),
            next_instr_len: None,
            cur_instr_len: None,
            changed_pc: false,
        }
    }

    // The PC is a liar (sometimes)!
    // the PC offset is always 4 bytes even in Thumb state
    pub fn arm_adjusted_pc(&self) -> u32 {
        self.pc - self.cur_instr_len.unwrap() + 4 - self.next_instr_len.unwrap()
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
            "PC" => self.pc = value, // When an instruction updates the PC - write to the real PC!
            _ => panic!("Unknown register {}", name),
        }
    }

    // Can potentially update flags during computation of shift
    // https://www.keil.com/support/man/docs/armasm/armasm_dom1361289851539.htm
    pub fn value_of_flexible_second_operand(&self, op: &ArmOperand, _update_c_flag: bool) -> u32 {
        match op.op_type {
            ArmOperandType::Reg(reg_id) => {
                assert!(op.shift == ArmShift::Invalid, "Shift not supported");
                self.read_by_id(reg_id)
            }
            ArmOperandType::Imm(value) => value as u32,
            _ => panic!("Unsupported type"),
        }
    }

    pub fn eval_ldr_str_op_mem(&self, op_mem: &ArmOpMem) -> u32 {
        /* PC appears WORD aligned to LDR/STR PC relative instructions
         * https://community.arm.com/developer/ip-products/processors/f/cortex-m-forum/4541/real-value-of-pc-register/11430#11430
         */
        let base_reg_val = if op_mem.base() == PC {
            let pc_val = self.arm_adjusted_pc() as i64;
            pc_val & 0xFFFFFFFC
        } else {
            self.read_by_id(op_mem.base()) as i64
        };

        // Immediate offset
        let displacement: i32 = if op_mem.index().0 == 0 {
            op_mem.disp()
        } else {
            // Register offset
            let index_val = self.read_by_id(op_mem.index());
            (index_val as i32) * op_mem.scale() // Scale for index register (can be 1, or -1)
        };
        let result = base_reg_val + displacement as i64;
        result as u32
    }

    /*
     *  reglist must use only R0-R7. The exception is LR for a PUSH and PC for a POP.
     * lowest numbered register using the lowest memory address
     */
    pub fn push_pop_register_asc(&self, mut reg_list: Vec<RegId>) -> Vec<RegId> {
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
        output.push_str(&format!("PC {:08X} ", self.arm_adjusted_pc() - 5));
        output.push_str(&format!("SP {:08X} ", self.sp));
        output.push_str(&format!("N{}", self.cond_flags.n as u8));
        output.push_str(&format!("Z{}", self.cond_flags.z as u8));
        output.push_str(&format!("C{}", self.cond_flags.c as u8));
        output.push_str(&format!("V{}", self.cond_flags.v as u8));
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
    }
}
