use crate::cpu_state::CpuState;
use crate::DebugLevel;
use std::time::Instant;

pub struct Simulator {}

impl Simulator {
    pub fn run(mut state: CpuState, debug_level: &DebugLevel, pipelined: bool) {
        let start_time = Instant::now();
        let mut cycle_counter = 0;
        let mut flushes = 0;
        loop {
            if pipelined {
                cycle_counter = cycle_counter + 1;

                // These operations are stateless, they can take place in any order / concurrently
                let fetch = state.fetch();
                let decode = state.decode();
                let execute = state.execute(&debug_level);

                fetch.apply(&mut state);
                decode.apply(&mut state);
                if execute.apply(&mut state) {
                    flushes = flushes + 1;
                    state.flush_pipeline();
                }
            } else {
                cycle_counter = cycle_counter + 1;
                let fetch = state.fetch();
                fetch.apply(&mut state);

                cycle_counter = cycle_counter + 1;
                let decode = state.decode();
                decode.apply(&mut state);

                cycle_counter = cycle_counter + 1;
                let execute = state.execute(&debug_level);
                execute.apply(&mut state);
            }

            if state.should_terminate {
                break;
            }
        }
        let elapsed = start_time.elapsed();
        println!(
            "Simulator run {} cycles in {} seconds",
            cycle_counter,
            elapsed.as_millis() as f64 / 1000.0
        );

        /*
        In ARM processors that have no PFU, the target of a branch is not known until the end of the
         Execute stage. At the Execute stage it is known whether or not the branch is taken. In ARM
         processors without a PFU, the best performance is obtained by predicting all branches as
         not taken and filling the pipeline with the instructions that follow the branch in the
         current sequential path. In this case an untaken branch requires one cycle and a taken
         branch requires three or more cycles.
         */
        if pipelined {
            println!("Flushes due to taken branch: {}", flushes);
        }
    }
}
