use crate::cpu_state::CpuState;
use crate::instructions::NextInstructionState;
use crate::registers::ConditionFlag;
use crate::DebugLevel;
use capstone::prelude::*;
use crate::station::ReservationStation;

#[derive(Default)]
pub struct ExecuteChanges {
    pub register_changes: Vec<(RegId, u32)>,
    pub flag_changes: Vec<(ConditionFlag, bool)>,
    pub should_terminate: bool,
    pub did_execute_instruction: bool,
    pub did_skip_instruction: bool,
    pub instruction_is_branch: bool,
    pub next_state: NextInstructionState,
}

impl ExecuteChanges {
    pub fn register_change(&mut self, reg_id: RegId, value: u32) {
        self.register_changes.push((reg_id, value));
    }

    pub fn flag_change(&mut self, flag: ConditionFlag, value: bool) {
        self.flag_changes.push((flag, value));
    }
}

impl CpuState {
    pub fn execute(&self, debug_level: &DebugLevel, station: &ReservationStation) -> ExecuteChanges {
        let mut changes = ExecuteChanges::default();
        let ex = station.should_execute();
        // changes.instruction_is_branch = dec.imp.is_branch();
        // if *debug_level >= DebugLevel::Minimal {
        //     let mut output = String::new();
        //     if ex {
        //         output.push_str(&dec.string);
        //     } else {
        //         output.push_str(&format!("{} (omitted)", dec.string));
        //     }
        //     if *debug_level >= DebugLevel::Full {
        //         let padding: String = vec![' '; 30 as usize - output.len()].iter().collect();
        //         output.push_str(&format!("{} [{}]", padding, self.registers.debug_string()));
        //     }
        //     println!("{}", output);
        // }
        if ex {
            // changes.next_state = dec.imp.poll(self, &mut changes);
            // if changes.next_state.is_none() {
            //     changes.did_execute_instruction = true;
            // }
        } else {
            changes.did_skip_instruction = true;
        }
        changes
    }
}
