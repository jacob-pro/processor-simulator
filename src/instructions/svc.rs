use super::{Instruction, ShouldTerminate};
use crate::simulator::Simulator;
use capstone::arch::arm::{ArmOperand, ArmOperandType};

pub struct SVC {
    id: i32
}

impl SVC {
    pub fn new(operands: Vec<ArmOperand>) -> Self {
        let o = operands.first().unwrap();
        if let ArmOperandType::Imm(imm) = o.op_type {
            return Self {
                id: imm
            }
        }
        panic!("Unexpected SVC operands");
    }
}

impl Instruction for SVC {
    fn execute(&self, sim: &mut Simulator) -> ShouldTerminate {
        match self.id {
            1 => {
                println!("Program exited with code: {}", sim.registers.get("R0"));
                return true
            }
            _ => {
                println!("Unknown SVC ID: {}", self.id);
                return true
            }
        }
    }
}
