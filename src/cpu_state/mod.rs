pub mod decode;
pub mod execute;
pub mod fetch;
pub mod station;

use crate::cpu_state::decode::{DecodeResults, DecodedInstruction};
use crate::cpu_state::execute::StationResults;
use crate::cpu_state::fetch::{FetchResults, FetchedInstruction};
use crate::cpu_state::station::StationId;
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
    pub decoded_instructions: VecDeque<DecodedInstruction>, // Instructions waiting to be executed
    pub reservation_stations: Vec<ReservationStation>,
    pub should_terminate: bool,
    pub pending_registers: HashMap<RegId, StationId>, // Stations that will produce a register value
}

#[derive(Default)]
pub struct UpdateResult {
    pub pc_changed: bool,
    pub instructions_executed: u8,
    pub instructions_skipped: u8,
    pub branches_taken: u8,
    pub branches_not_taken: u8,
}

const DECODED_QUEUE_CAPACITY: usize = 6;

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
            pending_registers: Default::default(),
        }
    }

    // Instructions that are already executing are allowed to continue
    pub fn flush_pipeline(&mut self) {
        self.fetched_instruction = None;
        self.decoded_instructions.clear();
    }

    // If there will be space for another decoded instruction
    pub fn decoded_space(&self) -> bool {
        self.decoded_instructions.len() < DECODED_QUEUE_CAPACITY
    }

    // Transition the state to the new state
    pub fn apply_stages(
        &mut self,
        fetch_results: Option<FetchResults>,
        decode_results: Option<DecodeResults>,
        mut station_results: Vec<Option<StationResults>>,
    ) -> UpdateResult {
        let mut result = UpdateResult::default();

        // If we finished executing an instruction remove it from reservation stations
        assert_eq!(station_results.len(), self.reservation_stations.len());
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

        // Store the fetched instruction
        if let Some(fetch) = fetch_results {
            assert!(self.fetched_instruction.is_none());
            self.fetched_instruction = Some(fetch.instr);
            self.next_instr_addr = fetch.next_addr;
        }

        // Store the decoded instruction
        if let Some(decode_results) = decode_results {
            assert!(self.decoded_instructions.len() < DECODED_QUEUE_CAPACITY);
            self.decoded_instructions.push_back(decode_results.instr);
        }

        // Deal with completed instructions
        for (i, execute) in station_results.iter().enumerate() {
            if let Some(execute) = execute {
                if let Some(register_changes) = &execute.register_changes {
                    // Write results to architectural registers
                    for (reg_id, value) in register_changes {
                        self.registers.write_by_id(*reg_id, *value);
                        if *reg_id == PC {
                            // If the PC is changed we must ensure the next fetch uses the updated PC
                            self.next_instr_addr = *value;
                            result.pc_changed = true;
                        }
                        // Indicate that we are no longer waiting on this register to compute
                        if let Some(station_id) = self.pending_registers.get(reg_id) {
                            if *station_id == i {
                                self.pending_registers.remove(reg_id);
                            }
                        }
                    }
                    // Write results to waiting stations
                    for s in &mut self.reservation_stations {
                        s.receive_broadcast(i, &register_changes);
                    }
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

        // If any stations hold an instruction that may either branch
        // Or write to memory
        // Or has conditional execution (may not actually produce its output values)
        let pending_control_hazards = self
            .reservation_stations
            .iter()
            .flat_map(|s| &s.instruction)
            .find(|d| {
                let changes_pc = d.imp.hazardous();
                let conditional = if let ArmCC::ARM_CC_AL = d.cc {
                    false
                } else {
                    true
                };
                changes_pc || conditional
            })
            .is_some();

        let available_station = self
            .reservation_stations
            .iter()
            .find(|r| r.instruction.is_none())
            .is_some();

        if !pending_control_hazards && available_station && !result.pc_changed {
            // Issue an instruction
            if let Some(instr) = self.decoded_instructions.pop_front() {
                let mut source_registers = HashMap::new();
                let mut required_registers = instr.imp.source_registers();
                required_registers.insert(PC);
                if let ArmCC::ARM_CC_AL = instr.cc {
                } else {
                    // A condition code means that we will need to read CPSR
                    required_registers.insert(CPSR);
                }
                for r in required_registers {
                    if r == PC {
                        source_registers.insert(PC, Register::Ready(instr.address));
                    } else {
                        source_registers.insert(r, self.register_value(r));
                    }
                }
                let station = self
                    .reservation_stations
                    .iter_mut()
                    .find(|r| r.instruction.is_none())
                    .unwrap();
                for r in instr.imp.dest_registers() {
                    self.pending_registers.insert(r, station.id);
                }
                station.issue(instr, source_registers);
            }
        }

        if result.pc_changed {
            assert_eq!(result.branches_taken, 1); // Should never be more than 1
        }

        result
    }

    fn register_value(&self, reg_id: RegId) -> Register {
        if let Some(station_id) = self.pending_registers.get(&reg_id) {
            return Register::Pending(*station_id, reg_id);
        }
        Register::Ready(self.registers.read_by_id(reg_id))
    }
}
