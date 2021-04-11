pub mod decode;
pub mod execute;
pub mod fetch;

use crate::cpu_state::decode::DecodeChanges;
use crate::cpu_state::execute::StationChanges;
use crate::cpu_state::fetch::FetchChanges;
use crate::decoded::DecodedInstruction;
use crate::instructions::Instruction;
use crate::memory::Memory;
use crate::registers::ids::PC;
use crate::registers::RegisterFile;
use crate::station::{Register, ReservationStation};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub struct CpuState {
    pub memory: Arc<RwLock<Memory>>,
    pub registers: RegisterFile,
    pub next_instr_addr: u32, // Address of instruction waiting to be fetched
    pub fetched_instruction: Option<FetchedInstruction>, // Instruction waiting to be decoded
    pub reservation_stations: Vec<ReservationStation>,
    pub should_terminate: bool,
}

impl CpuState {
    pub fn new(memory: Arc<RwLock<Memory>>, entry: u32, stations: usize) -> Self {
        let registers = RegisterFile::new();
        let stations = (0..stations)
            .map(|i| ReservationStation::new(i, memory.clone()))
            .collect();
        Self {
            memory,
            registers,
            fetched_instruction: None,
            should_terminate: false,
            next_instr_addr: entry,
            reservation_stations: stations,
        }
    }

    pub fn flush_pipeline(&mut self) {
        self.fetched_instruction = None;
        for i in &mut self.reservation_stations {
            i.clear();
        }
    }

    // If there will be space for another decoded instruction
    // Depends on if the current instruction will complete this cycle or not
    pub fn decoded_space(&self) -> bool {
        self.reservation_stations
            .iter()
            .find(|r| r.instruction.is_none())
            .is_some()
    }

    // Transition the state to the new state
    pub fn update(
        &mut self,
        fetch: Option<FetchChanges>,
        decode: Option<DecodeChanges>,
        mut stations: Vec<Option<StationChanges>>,
    ) -> UpdateResult {
        assert_eq!(stations.len(), self.reservation_stations.len());
        let mut result = UpdateResult::default();

        // If we finished executing an instruction remove it from reservation station
        for (i, s) in stations.iter_mut().enumerate() {
            match s {
                None => {}
                Some(s) => {
                    let next_state = std::mem::take(&mut s.next_state);
                    match next_state {
                        None => self.reservation_stations[i].clear(),
                        Some(n) => {
                            self.reservation_stations[i]
                                .instruction
                                .as_mut()
                                .unwrap()
                                .imp = n
                        }
                    }
                }
            }
        }

        // If we decoded an instruction remove it from fetched
        match &decode {
            None => {}
            Some(_) => {
                self.fetched_instruction = None;
            }
        }

        match fetch {
            None => {}
            Some(fetch) => {
                assert!(self.fetched_instruction.is_none());
                self.fetched_instruction = Some(FetchedInstruction {
                    bytes: fetch.instruction,
                    address: self.next_instr_addr,
                });
                self.next_instr_addr = fetch.next_addr;
            }
        }

        match decode {
            None => {}
            Some(decode) => {
                // Find an empty reservation station
                let station = self
                    .reservation_stations
                    .iter_mut()
                    .find(|r| r.instruction.is_none())
                    .unwrap();
                let mut source_registers = HashMap::new();
                source_registers.insert(PC, Register::Ready(decode.instr.address));
                for r in decode.instr.source_registers() {
                    if r != PC {
                        source_registers.insert(r, Register::Ready(self.registers.read_by_id(r)));
                    }
                }
                station.issue(decode.instr, source_registers);
            }
        }

        for (i, execute) in stations.iter().enumerate() {
            match execute {
                None => {}
                Some(execute) => {
                    for (reg_id, value) in &execute.register_changes {
                        self.registers.write_by_id(*reg_id, *value);
                        if *reg_id == PC {
                            // If the PC is changed we must ensure the next fetch uses the updated PC
                            self.next_instr_addr = *value;
                            result.pc_changed = true;
                        }
                    }
                    for s in &mut self.reservation_stations {
                        s.receive_broadcast(i, &execute.register_changes);
                    }
                    self.should_terminate = execute.should_terminate;
                    if execute.did_execute_instruction {
                        result.instructions_executed = result.instructions_executed + 1;
                    }
                    if execute.did_skip_instruction {
                        result.instructions_skipped = result.instructions_skipped + 1;
                    }
                    if execute.did_execute_instruction && execute.instruction_is_branch {
                        result.branches_taken = result.branches_taken + 1;
                    }
                    if execute.did_skip_instruction && execute.instruction_is_branch {
                        result.branches_not_taken = result.branches_not_taken + 1;
                    }
                }
            }
        }

        // if result.pc_changed {
        //     assert!(result.branches_taken == 1);
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
