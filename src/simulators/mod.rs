pub mod non_pipelined;
pub mod out_of_order;
pub mod pipelined;

use crate::cpu_state::UpdateResult;
use crate::memory::Memory;
use crate::DebugLevel;
use std::fmt::{Display, Formatter};

pub trait Simulator {
    fn run(&self, memory: Memory, entry: u32, debug_level: &DebugLevel) -> SimulationStats;
    fn name(&self) -> String;
}

#[derive(Default, Debug)]
pub struct SimulationStats {
    instructions_executed: u64,
    instructions_skipped: u64,
    total_cycles: u64,
    branches_not_taken: u64,
    branches_taken: u64,
}

impl SimulationStats {
    fn update(&mut self, from: &UpdateResult) {
        self.instructions_executed = self.instructions_executed + from.instructions_executed as u64;
        self.instructions_skipped = self.instructions_skipped + from.instructions_skipped as u64;
        self.branches_taken = self.branches_taken + from.branches_taken as u64;
        self.branches_not_taken = self.branches_not_taken + from.branches_not_taken as u64;
    }
}

impl Display for SimulationStats {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "Number of instructions executed: {} (+ {} skipped = {} total instructions)",
            self.instructions_executed,
            self.instructions_skipped,
            self.instructions_executed + self.instructions_skipped
        )?;
        writeln!(f, "Number of cycles: {}", self.total_cycles)?;
        writeln!(
            f,
            "Number of instructions per cycle: {:.3}",
            self.instructions_executed as f64 / self.total_cycles as f64
        )?;
        writeln!(f, "Number of branches taken: {}", self.branches_taken)?;
        writeln!(
            f,
            "Number of branches not taken: {}",
            self.branches_not_taken
        )?;
        Ok(())
    }
}
