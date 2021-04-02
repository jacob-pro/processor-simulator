use crate::cpu_state::CpuState;
use crate::registers::{ConditionFlag, PC};
use crate::DebugLevel;
use capstone::prelude::*;

#[derive(Default)]
pub struct ExecuteChanges {
    register_changes: Vec<(RegId, u32)>,
    flag_changes: Vec<(ConditionFlag, bool)>,
    should_terminate: bool,
}

impl ExecuteChanges {
    pub fn register_change(&mut self, reg_id: RegId, value: u32) {
        self.register_changes.push((reg_id, value));
    }

    pub fn flag_change(&mut self, flag: ConditionFlag, value: bool) {
        self.flag_changes.push((flag, value));
    }

    pub fn apply(self, sim: &mut CpuState) -> Option<u32> {
        let mut changed_pc = None;
        for (reg_id, value) in self.register_changes {
            sim.registers.write_by_id(reg_id, value);
            if reg_id == PC {
                changed_pc = Some(value);
            }
        }
        for (flag, value) in self.flag_changes {
            sim.registers.cond_flags.write_flag(flag, value);
        }
        sim.should_terminate = self.should_terminate;
        changed_pc
    }
}

impl CpuState {
    pub fn execute(&self, debug: &DebugLevel) -> ExecuteChanges {
        let mut changes = ExecuteChanges::default();
        match &self.decoded_instruction {
            None => {}
            Some(dec) => {
                let ex = self.registers.cond_flags.should_execute(&dec.cc);
                if *debug >= DebugLevel::Minimal {
                    let mut output = String::new();
                    if ex {
                        output.push_str(&dec.string);
                    } else {
                        output.push_str(&format!("{} (omitted)", dec.string));
                    }
                    if *debug >= DebugLevel::Full {
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
                    changes.should_terminate = dec.imp.execute(self, &mut changes);
                }
            }
        }
        changes
    }
}
