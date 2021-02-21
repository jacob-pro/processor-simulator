use capstone::prelude::*;
use std::rc::Rc;
use capstone::arch::arm::{ArmOperand, ArmOperandType, ArmShift, ArmOpMem};

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
    pub future_pc : u32,
}

impl RegisterFile {
    
    pub fn new(capstone: Rc<Capstone>, pc: u32) -> Self {
        Self {
            gprs: Default::default(),
            sp: std::u32::MAX,
            lr: 0,
            pc,
            cond_flags: Default::default(),
            capstone,
            future_pc: 0,
        }
    }

    pub fn get_by_name(&mut self, name: &str) -> &mut u32 {
        let name = name.to_ascii_uppercase();
        if name.starts_with("R") {
            let number = name[1..].parse::<usize>().expect("Invalid register");
            return &mut self.gprs[number]
        }
        return match name.as_str() {
            "IP" => &mut self.gprs[12], // Synonym
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

    pub fn eval_op_mem(&mut self, op_mem: &ArmOpMem) -> u32 {

        /* PC appears WORD aligned to LDR/STR PC relative instructions
        * https://community.arm.com/developer/ip-products/processors/f/cortex-m-forum/4541/real-value-of-pc-register/11430#11430
         */
        let base_reg_val = if self.capstone.reg_name(op_mem.base()).unwrap() == "pc" {
            let pc_val = *self.get_by_id(op_mem.base()) as i64;
            pc_val & 0xFFFFFFFC
        } else {
            *self.get_by_id(op_mem.base()) as i64
        };

        // Immediate offset
        let displacement: i32 = if op_mem.index().0 == 0 {
            op_mem.disp()
        } else {
            // Register offset
            let index_val = *self.get_by_id(op_mem.index());
            (index_val as i32) * op_mem.scale()     // Scale for index register (can be 1, or -1)
        };
        let result = base_reg_val + displacement as i64;
        result as u32
    }

}
