use crate::cpu_state::CpuState;
use crate::registers::ids::PC;
use crate::registers::ConditionFlag;
use crate::DebugLevel;
use capstone::prelude::*;

#[derive(Default)]
pub struct ExecuteChanges {
    register_changes: Vec<(RegId, u32)>,
    flag_changes: Vec<(ConditionFlag, bool)>,
    pub should_terminate: bool,
    pub did_execute_instruction: bool,
    pub did_omit_instruction: bool,
    pub instruction_was_branch: bool,
}

pub struct ExecutionStageResult {
    pub dirty_pc: bool,
    pub did_execute_instruction: bool,
    pub did_omit_instruction: bool,
    pub instruction_was_branch: bool,
}

impl ExecuteChanges {
    pub fn register_change(&mut self, reg_id: RegId, value: u32) {
        self.register_changes.push((reg_id, value));
    }

    pub fn flag_change(&mut self, flag: ConditionFlag, value: bool) {
        self.flag_changes.push((flag, value));
    }

    pub fn apply(self, state: &mut CpuState) -> ExecutionStageResult {
        let mut changed_pc = false;
        for (reg_id, value) in self.register_changes {
            state.registers.write_by_id(reg_id, value);
            if reg_id == PC {
                // If the PC is changed we must ensure the next fetch uses the updated PC
                state.next_instr_addr = value;
                changed_pc = true;
            }
        }
        for (flag, value) in self.flag_changes {
            state.registers.cond_flags.write_flag(flag, value);
        }
        state.should_terminate = self.should_terminate;
        ExecutionStageResult {
            dirty_pc: changed_pc,
            did_execute_instruction: self.did_execute_instruction,
            did_omit_instruction: self.did_omit_instruction,
            instruction_was_branch: self.instruction_was_branch,
        }
    }
}

impl CpuState {
    pub fn execute(&self, debug_level: &DebugLevel) -> ExecuteChanges {
        let mut changes = ExecuteChanges::default();
        match &self.decoded_instruction {
            None => {}
            Some(dec) => {
                let ex = self.registers.cond_flags.should_execute(&dec.cc);
                changes.did_execute_instruction = ex;
                changes.did_omit_instruction = !ex;
                changes.instruction_was_branch = dec.imp.is_branch();
                if *debug_level >= DebugLevel::Minimal {
                    let mut output = String::new();
                    if ex {
                        output.push_str(&dec.string);
                    } else {
                        output.push_str(&format!("{} (omitted)", dec.string));
                    }
                    if *debug_level >= DebugLevel::Full {
                        let padding: String =
                            vec![' '; 30 as usize - output.len()].iter().collect();
                        output.push_str(&format!(
                            "{} [{}]",
                            padding,
                            self.registers.debug_string()
                        ));
                    }
                    println!("{}", output);
                }
                if ex {
                    dec.imp.poll(self, &mut changes);
                }
            }
        }
        changes
    }
}
