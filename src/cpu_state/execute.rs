use crate::cpu_state::station::{Register, ReservationStation};
use crate::cpu_state::CpuState;
use crate::instructions::{Instruction, PollResult};
use crate::registers::ids::{CPSR, LR, PC, R0, R8, SP};
use crate::registers::ConditionFlag;
use crate::DebugLevel;
use capstone::prelude::*;

#[derive(Default, Debug)]
pub struct StationResults {
    pub register_changes: Option<Vec<(RegId, u32)>>, // None means instruction has not finished yet
    pub should_terminate: bool,
    pub did_execute_instruction: bool,
    pub did_skip_instruction: bool,
    pub instruction_is_branch: bool,
    pub next_state: Option<Box<dyn Instruction>>, // None means instruction is complete
}

impl CpuState {
    pub fn execute_station(
        &self,
        debug_level: &DebugLevel,
        station: &ReservationStation,
    ) -> Option<StationResults> {
        if !station.ready() {
            return None;
        }
        let instr = station.instruction.as_ref().unwrap();
        let mut changes = StationResults::default();
        let should_execute = station.evaluate_condition_code();
        changes.instruction_is_branch = instr.imp.dest_registers().contains(&PC);

        let print_debug = || {
            if *debug_level >= DebugLevel::Minimal {
                let mut output = String::new();
                if should_execute {
                    output.push_str(&instr.string);
                } else {
                    output.push_str(&format!("{} (omitted)", instr.string));
                }
                if *debug_level >= DebugLevel::Full {
                    let padding: String = vec![' '; 30 as usize - output.len()].iter().collect();
                    output.push_str(&format!(
                        "{} [{}]",
                        padding,
                        self.debug_string(|reg_id| {
                            if let Some(r) = station.source_registers.get(&reg_id) {
                                if let Register::Ready(val) = r {
                                    return *val;
                                }
                            }
                            self.registers.read_by_id(reg_id)
                        })
                    ));
                }
                println!("{}", output);
            }
        };

        if should_execute {
            match instr.imp.poll(&station) {
                PollResult::Complete(c) => {
                    changes.register_changes = Some(c);
                    changes.did_execute_instruction = true;
                    print_debug();
                }
                PollResult::Again(s) => {
                    changes.next_state = Some(s);
                }
                PollResult::Exception => {
                    changes.should_terminate = true;
                    changes.did_execute_instruction = true;
                    print_debug();
                }
            }
        } else {
            changes.did_skip_instruction = true;
            print_debug();
        }

        Some(changes)
    }

    fn debug_string<F>(&self, f: F) -> String
    where
        F: Fn(RegId) -> u32,
    {
        let mut output = String::new();
        let cpsr = f(CPSR);
        for i in R0.0..R8.0 {
            output.push_str(&format!("R{} {:08X} ", i - R0.0, f(RegId(i))));
        }
        output.push_str(&format!("LR {:08X} ", f(LR)));
        output.push_str(&format!("PC {:08X} ", f(PC) & 0xFFFFFFFE));
        output.push_str(&format!("SP {:08X} ", f(SP)));
        output.push_str(&format!("N{}", ConditionFlag::N.read_flag(cpsr) as u8));
        output.push_str(&format!("Z{}", ConditionFlag::Z.read_flag(cpsr) as u8));
        output.push_str(&format!("C{}", ConditionFlag::C.read_flag(cpsr) as u8));
        output.push_str(&format!("V{}", ConditionFlag::V.read_flag(cpsr) as u8));
        output
    }
}
