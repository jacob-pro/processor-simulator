use crate::cpu_state::station::ReservationStation;
use crate::cpu_state::CpuState;
use crate::instructions::{Instruction, PollResult};
use crate::DebugLevel;
use capstone::prelude::*;

#[derive(Default)]
pub struct StationChanges {
    pub register_changes: Vec<(RegId, u32)>,
    pub should_terminate: bool,
    pub did_execute_instruction: bool,
    pub did_skip_instruction: bool,
    pub instruction_is_branch: bool,
    pub next_state: Option<Box<dyn Instruction>>,
}

impl CpuState {
    pub fn execute(
        &self,
        debug_level: &DebugLevel,
        station: &ReservationStation,
    ) -> StationChanges {
        assert!(station.ready());
        let instr = station.instruction.as_ref().unwrap();
        let mut changes = StationChanges::default();
        let ex = station.evaluate_condition_code();
        // changes.instruction_is_branch = instr.imp.is_branch();
        if *debug_level >= DebugLevel::Minimal {
            let mut output = String::new();
            if ex {
                output.push_str(&instr.string);
            } else {
                output.push_str(&format!("{} (omitted)", instr.string));
            }
            if *debug_level >= DebugLevel::Full {
                let padding: String = vec![' '; 30 as usize - output.len()].iter().collect();
                output.push_str(&format!("{} [{}]", padding, self.registers.debug_string()));
            }
            println!("{}", output);
        }
        if ex {
            match instr.imp.poll(&station) {
                PollResult::Complete(c) => {
                    changes.register_changes = c;
                    changes.did_execute_instruction = true;
                }
                PollResult::Again(s) => {
                    changes.next_state = Some(s);
                }
                PollResult::Exception => {
                    changes.should_terminate = true;
                    changes.did_execute_instruction = true;
                }
            }
        } else {
            changes.did_skip_instruction = true;
        }
        changes
    }
}
