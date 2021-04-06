use crate::cpu_state::CpuState;
use crate::DebugLevel;

pub trait Simulator {
    fn run(&self, state: CpuState, debug_level: &DebugLevel);
    fn name(&self) -> &'static str;
}

pub struct NonPipelinedSimulator {}

impl Simulator for NonPipelinedSimulator {
    fn run(&self, mut state: CpuState, debug_level: &DebugLevel) {
        let mut cycle_counter = 0;
        loop {
            cycle_counter = cycle_counter + 1;
            let fetch = state.fetch();
            fetch.apply(&mut state);

            cycle_counter = cycle_counter + 1;
            let decode = state.decode();
            decode.apply(&mut state);

            cycle_counter = cycle_counter + 1;
            let execute = state.execute(&debug_level);
            execute.apply(&mut state);

            if state.should_terminate {
                break;
            }
        }
        println!("Total cycles: {}", cycle_counter);
    }

    fn name(&self) -> &'static str {
        "Non pipelined scalar simulator"
    }
}

pub struct PipelinedSimulator {}

impl Simulator for PipelinedSimulator {
    fn run(&self, mut state: CpuState, debug_level: &DebugLevel) {
        let mut cycle_counter = 0;
        let mut flushes = 0;
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(3)
            .build()
            .unwrap();
        loop {
            cycle_counter = cycle_counter + 1;

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

            fetch.unwrap().apply(&mut state);
            decode.unwrap().apply(&mut state);
            if execute.unwrap().apply(&mut state) {
                flushes = flushes + 1;
                state.flush_pipeline();
            }

            if state.should_terminate {
                break;
            }
        }
        println!("Total cycles: {}", cycle_counter);
        /*
        In ARM processors that have no PFU, the target of a branch is not known until the end of the
         Execute stage. At the Execute stage it is known whether or not the branch is taken. In ARM
         processors without a PFU, the best performance is obtained by predicting all branches as
         not taken and filling the pipeline with the instructions that follow the branch in the
         current sequential path. In this case an untaken branch requires one cycle and a taken
         branch requires three or more cycles.
         */
        println!("Flushes due to taken branch: {}", flushes);
    }

    fn name(&self) -> &'static str {
        "3 stage pipelined scalar simulator"
    }
}

pub struct SimulationResult {
    instructions_executed: u64,
    total_cycles: u64,
}
