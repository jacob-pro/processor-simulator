use capstone::arch::arm::{ArmOperand, ArmOperandType};
use capstone::prelude::*;

pub trait ArmOperandExt {

    fn reg_id(&self) -> Option<RegId>;

    fn imm_value(&self) -> Option<i32>;
}

impl ArmOperandExt for ArmOperand {

    fn reg_id(&self) -> Option<RegId> {
        if let ArmOperandType::Reg(id) = self.op_type {
            return Some(id)
        }
        None
    }

    fn imm_value(&self) -> Option<i32> {
        if let ArmOperandType::Imm(value) = self.op_type {
            return Some(value)
        }
        None
    }
}
