use capstone::prelude::*;
use std::rc::Rc;
use capstone::arch::arm::{ArmOperand, ArmOperandType, ArmShift};

#[derive(Default)]
pub struct ConditionFlags {
    pub n: bool,    // Negative
    pub z: bool,    // Zero
    pub c: bool,    // Carry
    pub v: bool,    // Signed overflow
}

pub struct RegisterFile {
    pub gprs: [u32; 13],
    pub sp: u32,
    pub lr: u32,
    pub pc: u32,
    pub cond_flags: ConditionFlags,
    capstone: Rc<Capstone>,
}

impl RegisterFile {
    
    pub fn new(capstone: Rc<Capstone>, pc: u32) -> Self {
        Self {
            gprs: Default::default(),
            sp: std::u32::MAX,
            lr: 0,
            pc,
            cond_flags: Default::default(),
            capstone
        }
    }

    pub fn get_by_name(&mut self, name: &str) -> &mut u32 {
        let name = name.to_ascii_uppercase();
        if name.starts_with("R") {
            let number = name[1..].parse::<usize>().expect("Invalid register");
            return &mut self.gprs[number]
        }
        return match name.as_str() {
            "SP" => &mut self.sp,
            "LR" => &mut self.lr,
            "PC" => &mut self.pc,
            _ => panic!("Unknown register {}", name)
        }
    }

    pub fn get_by_id(&mut self, id: RegId) -> &mut u32 {
        let n = self.capstone.reg_name(id).expect("Couldn't get reg_name");
        self.get_by_name(&n)
    }

    // https://www.keil.com/support/man/docs/armasm/armasm_dom1361289851539.htm
    pub fn value_of_flexible_second_operand(&mut self, op: &ArmOperand, _update_c_flag: bool) -> u32 {
        match op.op_type {
            ArmOperandType::Reg(reg_id) => {
                assert!(op.shift == ArmShift::Invalid, "Shift not yet implemented");
                *self.get_by_id(reg_id)
            },
            ArmOperandType::Imm(value) => { value as u32 }
            _ => panic!("Unsupported type")
        }
    }

}
