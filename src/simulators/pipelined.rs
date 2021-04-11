use crate::cpu_state::CpuState;
use crate::memory::Memory;
use crate::simulators::{SimulationStats, Simulator};
use crate::DebugLevel;

pub struct PipelinedSimulator {}

impl Simulator for PipelinedSimulator {
    fn run(&self, memory: Memory, entry: u32, debug_level: &DebugLevel) -> SimulationStats {
        let mut state = CpuState::new(memory, entry, 1);
        let mut stats = SimulationStats::default();
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(3)
            .build()
            .unwrap();
        /*
        In ARM processors that have no PFU, the target of a branch is not known until the end of the
         Execute stage. At the Execute stage it is known whether or not the branch is taken. In ARM
         processors without a PFU, the best performance is obtained by predicting all branches as
         not taken and filling the pipeline with the instructions that follow the branch in the
         current sequential path. In this case an untaken branch requires one cycle and a taken
         branch requires three or more cycles.
         */
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
                if state.reservation_stations.first().unwrap().ready() {
                    s.spawn(|_| {
                        execute = Some(
                            state
                                .execute(&debug_level, state.reservation_stations.first().unwrap()),
                        )
                    });
                }
            });

            let result = state.update(fetch.unwrap(), decode.unwrap(), vec![execute]);
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
        "3 stage pipelined scalar simulator"
    }
}
