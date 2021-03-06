use super::{Instruction, ShouldTerminate};
use crate::simulator::Simulator;
use capstone::arch::arm::{ArmOperand};
use crate::instructions::util::ArmOperandExt;
use std::io::Write;

pub struct SVC {
    id: i32
}

impl SVC {
    pub fn new(operands: Vec<ArmOperand>) -> Self {
        let id = operands[0].imm_value().unwrap();
        Self { id }
    }
}

impl Instruction for SVC {
    fn execute(&self, sim: &mut Simulator) -> ShouldTerminate {
        match self.id {
            1 => {
                println!("\nProgram exited with code: {}", sim.registers.gprs[0] as i32);
                return true;
            }
            2 => {
                let buffer_addr = sim.registers.gprs[0];
                let buffer_len = sim.registers.gprs[1];
                let data = sim.memory.read_bytes(buffer_addr, buffer_len);
                std::io::stdout().write_all(&data).expect("Failed to write");
            }
            _ => {
                println!("\nUnknown SVC ID: {}", self.id);
                return true;
            }
        }
        false
    }
}
