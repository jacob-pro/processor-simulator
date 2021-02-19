use super::{Instruction, ShouldTerminate};
use capstone::Capstone;
use crate::simulator::Simulator;
use capstone::arch::arm::{ArmOperand, ArmOperandType};

pub struct PUSH {
    reg_list: Vec<String>
}

impl PUSH {
    pub fn new(operands: Vec<ArmOperand>, capstone: &Capstone) -> Self {
        let reg_list = operands.into_iter()
            .map(|x: ArmOperand| {
                if let ArmOperandType::Reg(id) = x.op_type {
                    return capstone.reg_name(id).unwrap()
                }
                panic!("Unexpected operand type")
            }).collect();
        Self {
            reg_list
        }
    }
}

impl Instruction for PUSH {
    fn execute(&self, sim: &mut Simulator) -> ShouldTerminate {
        for r in &self.reg_list {
            sim.registers.sp = sim.registers.sp - 4;
            let register_value = sim.registers.get(r).to_le_bytes();
            sim.memory.write_bytes(sim.registers.sp, &register_value);
        }
        return false
    }
}
