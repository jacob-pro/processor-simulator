pub mod decode;
pub mod execute;
pub mod fetch;
pub mod station;

use crate::cpu_state::decode::{DecodeResults, DecodedInstruction};
use crate::cpu_state::execute::StationResults;
use crate::cpu_state::fetch::{FetchResults, FetchedInstruction};
use crate::memory::Memory;
use crate::registers::ids::{CPSR, PC};
use crate::registers::RegisterFile;
use capstone::arch::arm::ArmCC;
use capstone::RegId;
use station::{Register, ReservationStation};
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, RwLock};

pub struct CpuState {
    pub memory: Arc<RwLock<Memory>>,
    pub registers: RegisterFile,
    pub next_instr_addr: u32, // Address of instruction waiting to be fetched
    pub fetched_instruction: Option<FetchedInstruction>, // Instruction waiting to be decoded
    pub decoded_instructions: VecDeque<DecodedInstruction>,
    pub reservation_stations: Vec<ReservationStation>,
    pub should_terminate: bool,
}

#[derive(Default)]
pub struct UpdateResult {
    pub pc_changed: bool,
    pub instructions_executed: u8,
    pub instructions_skipped: u8,
    pub branches_taken: u8,
    pub branches_not_taken: u8,
}

const DECODED_QUEUE_CAPACITY: usize = 4;

impl CpuState {
    pub fn new(memory: Memory, entry: u32, stations: usize) -> Self {
        let memory = Arc::new(RwLock::new(memory));
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
            decoded_instructions: Default::default(),
        }
    }

    pub fn flush_pipeline(&mut self) {
        self.fetched_instruction = None;
        self.decoded_instructions.clear();
        for i in &mut self.reservation_stations {
            i.clear();
        }
    }

    // If there will be space for another decoded instruction
    pub fn decoded_space(&self) -> bool {
        if self.decoded_instructions.len() < DECODED_QUEUE_CAPACITY {
            return true;
        }
        self.reservation_stations
            .iter()
            .find(|r| r.instruction.is_none())
            .is_some()
    }

    // Transition the state to the new state
    pub fn apply_stages(
        &mut self,
        fetch_results: Option<FetchResults>,
        decode_results: Option<DecodeResults>,
        mut station_results: Vec<Option<StationResults>>,
    ) -> UpdateResult {
        assert_eq!(station_results.len(), self.reservation_stations.len());
        let mut result = UpdateResult::default();

        // If we finished executing an instruction remove it from reservation station
        for (i, s) in station_results.iter_mut().enumerate() {
            if let Some(s) = s {
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

        // If we decoded an instruction remove it from fetched
        if let Some(_) = &decode_results {
            self.fetched_instruction = None;
        }

        if let Some(fetch) = fetch_results {
            assert!(self.fetched_instruction.is_none());
            self.fetched_instruction = Some(fetch.instr);
            self.next_instr_addr = fetch.next_addr;
        }

        if let Some(decode_results) = decode_results {
            assert!(self.decoded_instructions.len() <= DECODED_QUEUE_CAPACITY);
            self.decoded_instructions.push_back(decode_results.instr);
        }

        for (i, execute) in station_results.iter().enumerate() {
            if let Some(execute) = execute {
                for (reg_id, value) in &execute.register_changes {
                    // Write results to architectural registers
                    self.registers.write_by_id(*reg_id, *value);
                    if *reg_id == PC {
                        // If the PC is changed we must ensure the next fetch uses the updated PC
                        self.next_instr_addr = *value;
                        result.pc_changed = true;
                    }
                }
                // Write results to waiting stations
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

        // Don't issue if we have control hazards pending
        let no_control_hazards = self
            .reservation_stations
            .iter()
            .flat_map(|s| &s.instruction)
            .find(|d| d.imp.control_hazard())
            .is_none();
        let available_station = self
            .reservation_stations
            .iter()
            .find(|r| r.instruction.is_none())
            .is_some();

        if no_control_hazards && available_station {
            // Issue an instruction
            if let Some(instr) = self.decoded_instructions.pop_front() {
                let mut source_registers = HashMap::new();
                let mut required_registers = instr.imp.source_registers();
                required_registers.insert(PC);
                if let ArmCC::ARM_CC_AL = instr.cc {
                } else {
                    required_registers.insert(CPSR);
                }
                for r in required_registers {
                    if r == PC {
                        source_registers.insert(PC, Register::Ready(instr.address));
                    } else {
                        source_registers.insert(r, self.register_value(r));
                    }
                }
                self.reservation_stations
                    .iter_mut()
                    .find(|r| r.instruction.is_none())
                    .unwrap()
                    .issue(instr, source_registers);
            }
        }

        if result.pc_changed {
            assert_eq!(result.branches_taken, 1);
        }

        result
    }

    fn register_value(&self, id: RegId) -> Register {
        for s in &self.reservation_stations {
            if let Some(instr) = &s.instruction {
                if instr.imp.dest_registers().contains(&id) {
                    return Register::Pending(s.id, id);
                }
            }
        }
        Register::Ready(self.registers.read_by_id(id))
    }
}
