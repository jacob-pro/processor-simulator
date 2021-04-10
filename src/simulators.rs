use crate::cpu_state::{CpuState, UpdateResult};
use crate::DebugLevel;
use std::fmt::{Display, Formatter};

pub trait Simulator {
    fn run(&self, state: CpuState, debug_level: &DebugLevel) -> SimulationStats;
    fn name(&self) -> &'static str;
}

pub struct NonPipelinedSimulator {}

impl Simulator for NonPipelinedSimulator {
    fn run(&self, mut state: CpuState, debug_level: &DebugLevel) -> SimulationStats {
        let mut stats = SimulationStats::default();
        // loop {
        //     stats.total_cycles = stats.total_cycles + 1;
        //     let fetch = state.fetch();
        //     state.update(fetch, None, None);
        //
        //     stats.total_cycles = stats.total_cycles + 1;
        //     let decode = state.decode();
        //     state.update(None, decode, None);
        //
        //     while state.decoded_instruction.is_some() {
        //         stats.total_cycles = stats.total_cycles + 1;
        //         let execute = state.execute(&debug_level);
        //         let result = state.update(None, None, execute);
        //         stats.update(&result);
        //     }
        //
        //     if state.should_terminate {
        //         break;
        //     }
        // }
        stats
    }

    fn name(&self) -> &'static str {
        "Non pipelined scalar simulator"
    }
}

pub struct PipelinedSimulator {}

impl Simulator for PipelinedSimulator {
    fn run(&self, mut state: CpuState, debug_level: &DebugLevel) -> SimulationStats {
        let mut stats = SimulationStats::default();
        // let pool = rayon::ThreadPoolBuilder::new()
        //     .num_threads(3)
        //     .build()
        //     .unwrap();
        // /*
        // In ARM processors that have no PFU, the target of a branch is not known until the end of the
        //  Execute stage. At the Execute stage it is known whether or not the branch is taken. In ARM
        //  processors without a PFU, the best performance is obtained by predicting all branches as
        //  not taken and filling the pipeline with the instructions that follow the branch in the
        //  current sequential path. In this case an untaken branch requires one cycle and a taken
        //  branch requires three or more cycles.
        //  */
        // loop {
        //     stats.total_cycles = stats.total_cycles + 1;
        //
        //     let mut fetch = None;
        //     let mut decode = None;
        //     let mut execute = None;
        //
        //     // These operations are stateless, they can take place in any order / concurrently
        //     // However because they are not actually computationally demanding it is actually slower
        //     // running in parallel (overhead of threading library)!
        //     // But is still here to demonstrate the ability to do it.
        //     pool.scope(|s| {
        //         s.spawn(|_| fetch = Some(state.fetch()));
        //         s.spawn(|_| decode = Some(state.decode()));
        //         s.spawn(|_| execute = Some(state.execute(&debug_level)));
        //     });
        //
        //     let result = state.update(fetch.unwrap(), decode.unwrap(), execute.unwrap());
        //     stats.update(&result);
        //
        //     if result.pc_changed {
        //         state.flush_pipeline();
        //     }
        //     if state.should_terminate {
        //         break;
        //     }
        // }
        stats
    }

    fn name(&self) -> &'static str {
        "3 stage pipelined scalar simulator"
    }
}


pub struct OutOfOrderSimulator {}

impl Simulator for OutOfOrderSimulator {
    fn run(&self, mut state: CpuState, debug_level: &DebugLevel) -> SimulationStats {
        let mut stats = SimulationStats::default();
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(3)
            .build()
            .unwrap();

        loop {
            stats.total_cycles = stats.total_cycles + 1;

            let mut fetch = None;
            let mut decode = None;
            let mut execute = None;

            // These operations are stateless, they can take place in any order / concurrently
            // However because they are not actually computationally demanding it is actually slower
            // running in parallel (overhead of threading library)!
            // But is still here to demonstrate the ability to do it.
            pool.scope(|s| {
                s.spawn(|_| fetch = Some(state.fetch()));
                s.spawn(|_| decode = Some(state.decode()));
                s.spawn(|_| execute = Some(state.execute(&debug_level)));
            });

            let result = state.update(fetch.unwrap(), decode.unwrap(), execute.unwrap());
            stats.update(&result);

            if result.pc_changed {
                state.flush_pipeline();
            }
            if state.should_terminate {
                break;
            }
        }
        stats
    }

    fn name(&self) -> &'static str {
        "out-of-order superscalar simulator"
    }
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
            "Number of instructions per cycle: {}",
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
