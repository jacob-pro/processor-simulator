use super::{Instruction, ShouldTerminate};
use crate::simulator::Simulator;
use capstone::arch::arm::{ArmOperand, ArmOperandType};
use crate::instructions::util::ArmOperandExt;

pub struct SVC {
    id: i32
}

impl SVC {
    pub fn new(operands: Vec<ArmOperand>) -> Self {
        let id = operands.first().unwrap().imm_value().unwrap();
        return Self {
            id
        }
    }
}

impl Instruction for SVC {
    fn execute(&self, sim: &mut Simulator) -> ShouldTerminate {
        match self.id {
            1 => {
                println!("Program exited with code: {}", sim.registers.get_by_name("R0"));
                return true
            }
            _ => {
                println!("Unknown SVC ID: {}", self.id);
                return true
            }
        }
    }
}
