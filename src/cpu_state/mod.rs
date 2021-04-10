pub mod decode;
pub mod execute;
pub mod fetch;

use crate::cpu_state::decode::DecodeChanges;
use crate::cpu_state::execute::ExecuteChanges;
use crate::cpu_state::fetch::FetchChanges;
use crate::instructions::Instruction;
use crate::memory::Memory;
use crate::registers::ids::PC;
use crate::registers::RegisterFile;
use std::sync::{Arc, RwLock};
use crate::decoded::DecodedInstruction;
use crate::station::ReservationStation;

pub struct CpuState {
    pub memory: Arc<RwLock<Memory>>,
    pub registers: RegisterFile,
    pub next_instr_addr: u32, // Address of instruction waiting to be fetched
    pub fetched_instruction: Option<FetchedInstruction>, // Instruction waiting to be decoded
    pub decoded_instructions: Vec<DecodedInstruction>, // Instruction queued to be executed
    pub reservation_stations: Vec<ReservationStation>,
    pub should_terminate: bool,
}

impl CpuState {
    pub fn new(memory: Arc<RwLock<Memory>>, entry: u32, stations: usize) -> Self {
        let registers = RegisterFile::new();
        let stations = (0..stations).map(|i| {
            ReservationStation::new(i, memory.clone())
        }).collect();
        Self {
            memory,
            registers,
            fetched_instruction: None,
            decoded_instructions: vec![],
            should_terminate: false,
            next_instr_addr: entry,
            reservation_stations: stations
        }
    }

    pub fn flush_pipeline(&mut self) {
        self.fetched_instruction = None;
        self.decoded_instructions.clear();
    }

    // If there will be space for another decoded instruction
    // Depends on if the current instruction will complete this cycle or not
    pub fn decoded_space(&self) -> bool {
        // match &self.decoded_instruction {
        //     None => {}
        //     Some(i) => {
        //         // if !i.imp.will_complete_this_cycle() {
        //         //     return false;
        //         // }
        //     }
        // }
        true
    }

    // Transition the state to the new state
    pub fn update(
        &mut self,
        fetch: Option<FetchChanges>,
        decode: Option<DecodeChanges>,
        execute: Option<ExecuteChanges>,
    ) -> UpdateResult {
        let mut result = UpdateResult::default();
        // If we finished an instruction remove it from decoded
        // match &execute {
        //     None => {}
        //     Some(execute) => {
        //         // if execute.next_state.is_none() {
        //         //     self.decoded_instruction = None;
        //         // }
        //     }
        // }
        //
        // // If we decoded an instruction remove it from fetched
        // match &decode {
        //     None => {}
        //     Some(_) => {
        //         self.fetched_instruction = None;
        //     }
        // }
        //
        // match fetch {
        //     None => {}
        //     Some(fetch) => {
        //         assert!(self.fetched_instruction.is_none());
        //         self.fetched_instruction = Some(FetchedInstruction {
        //             bytes: fetch.instruction,
        //             address: self.next_instr_addr,
        //         });
        //         self.next_instr_addr = fetch.next_addr;
        //     }
        // }
        //
        // match decode {
        //     None => {}
        //     Some(decode) => {
        //         assert!(self.decoded_instruction.is_none());
        //         self.registers.write_by_id(PC, decode.instr.address);
        //         self.decoded_instruction = Some(decode.instr);
        //     }
        // }
        //
        // match execute {
        //     None => {}
        //     Some(execute) => {
        //         for (reg_id, value) in execute.register_changes {
        //             self.registers.write_by_id(reg_id, value);
        //             if reg_id == PC {
        //                 // If the PC is changed we must ensure the next fetch uses the updated PC
        //                 self.next_instr_addr = value;
        //                 result.pc_changed = true;
        //             }
        //         }
        //         for (flag, value) in execute.flag_changes {
        //             self.registers.cond_flags.write_flag(flag, value);
        //         }
        //         // match execute.next_state {
        //         //     None => {}
        //         //     Some(c) => self.decoded_instruction.as_mut().unwrap().imp = c,
        //         // }
        //         self.should_terminate = execute.should_terminate;
        //         if execute.did_execute_instruction {
        //             result.instructions_executed = result.instructions_executed + 1;
        //         }
        //         if execute.did_skip_instruction {
        //             result.instructions_skipped = result.instructions_skipped + 1;
        //         }
        //         if execute.did_execute_instruction && execute.instruction_is_branch {
        //             result.branches_taken = result.branches_taken + 1;
        //         }
        //         if execute.did_skip_instruction && execute.instruction_is_branch {
        //             result.branches_not_taken = result.branches_not_taken + 1;
        //         }
        //     }
        // }
        //
        // if result.pc_changed {
        //     assert!(result.branches_taken >= 1);
        // }

        result
    }
}

pub struct FetchedInstruction {
    pub bytes: Vec<u8>,
    pub address: u32,
}

#[derive(Default)]
pub struct UpdateResult {
    pub pc_changed: bool,
    pub instructions_executed: u8,
    pub instructions_skipped: u8,
    pub branches_taken: u8,
    pub branches_not_taken: u8,
}
