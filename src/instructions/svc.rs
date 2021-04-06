use super::Instruction;
use crate::cpu_state::execute::ExecuteChanges;
use crate::cpu_state::CpuState;
use crate::instructions::util::ArmOperandExt;
use crate::registers::ids::{R0, R1};
use capstone::arch::arm::ArmOperand;
use std::io::Write;

pub struct SVC {
    id: i32,
}

impl SVC {
    pub fn new(operands: Vec<ArmOperand>) -> Self {
        let id = operands[0].imm_value().unwrap();
        Self { id }
    }
}

impl Instruction for SVC {
    fn execute(&self, sim: &CpuState, changes: &mut ExecuteChanges) {
        match self.id {
            1 => {
                println!(
                    "\nProgram exited with code: {}\n",
                    sim.registers.read_by_id(R0) as i32
                );
                changes.should_terminate = true;
            }
            2 => {
                let buffer_addr = sim.registers.read_by_id(R0);
                let buffer_len = sim.registers.read_by_id(R1);
                let data = sim
                    .memory
                    .read()
                    .unwrap()
                    .read_bytes(buffer_addr, buffer_len);
                std::io::stdout().write_all(&data).expect("Failed to write");
            }
            _ => {
                println!("\nUnknown SVC ID: {}", self.id);
                changes.should_terminate = true;
            }
        }
    }
}
