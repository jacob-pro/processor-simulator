use super::Instruction;
use crate::cpu_state::station::ReservationStation;
use crate::instructions::util::ArmOperandExt;
use crate::instructions::PollResult;
use crate::registers::ids::{R0, R1};
use capstone::arch::arm::ArmOperand;
use capstone::RegId;
use std::collections::HashSet;
use std::io::Write;

#[derive(Clone, Debug)]
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
    fn poll(&self, station: &ReservationStation) -> PollResult {
        match self.id {
            1 => {
                println!(
                    "\nProgram exited with code: {}\n",
                    station.read_by_id(R0) as i32
                );
                return PollResult::Exception;
            }
            2 => {
                let buffer_addr = station.read_by_id(R0);
                let buffer_len = station.read_by_id(R1);
                let data = station
                    .memory
                    .read()
                    .unwrap()
                    .read_bytes(buffer_addr, buffer_len)
                    .expect("SVC Tried to read from invalid memory address");
                std::io::stdout().write_all(&data).expect("Failed to write");
            }
            _ => {
                panic!("\nUnknown SVC ID: {}", self.id);
            }
        }
        PollResult::Complete(vec![])
    }

    fn source_registers(&self) -> HashSet<RegId> {
        hashset![R0, R1]
    }

    fn dest_registers(&self) -> HashSet<RegId> {
        hashset![]
    }

    fn hazardous(&self) -> bool {
        true
    }
}
