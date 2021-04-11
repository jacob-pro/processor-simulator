use capstone::arch::arm::{ArmOpMem, ArmOperand, ArmOperandType};
use capstone::prelude::*;
use std::collections::HashSet;

pub trait ArmOperandExt {
    fn reg_id(&self) -> Option<RegId>;

    fn imm_value(&self) -> Option<i32>;

    fn op_mem_value(&self) -> Option<ArmOpMem>;
}

impl ArmOperandExt for ArmOperand {
    fn reg_id(&self) -> Option<RegId> {
        if let ArmOperandType::Reg(id) = self.op_type {
            return Some(id);
        }
        None
    }

    fn imm_value(&self) -> Option<i32> {
        if let ArmOperandType::Imm(value) = self.op_type {
            return Some(value);
        }
        None
    }

    fn op_mem_value(&self) -> Option<ArmOpMem> {
        if let ArmOperandType::Mem(x) = self.op_type {
            return Some(x);
        }
        None
    }
}

pub trait RegisterSet {
    fn registers(&self) -> HashSet<RegId>;
}

impl RegisterSet for ArmOpMem {
    fn registers(&self) -> HashSet<RegId> {
        let mut set = hashset![self.base()];
        if self.index().0 != 0 {
            set.insert(self.index());
        }
        set
    }
}

impl RegisterSet for ArmOperand {
    fn registers(&self) -> HashSet<RegId> {
        if let ArmOperandType::Reg(reg_id) = self.op_type {
            return hashset![reg_id];
        }
        hashset![]
    }
}
