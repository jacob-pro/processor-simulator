use crate::cpu_state::CpuState;
use crate::memory::Memory;
use crate::simulators::{SimulationStats, Simulator};
use crate::DebugLevel;

pub struct OutOfOrderSimulator {}

impl Simulator for OutOfOrderSimulator {
    fn run(&self, memory: Memory, entry: u32, debug_level: &DebugLevel) -> SimulationStats {
        let mut state = CpuState::new(memory, entry, 1);
        let mut stats = SimulationStats::default();
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(3)
            .build()
            .unwrap();

        loop {
            stats.total_cycles = stats.total_cycles + 1;

            let mut fetch = None;
            let mut decode = None;
            let mut executes = Vec::new();

            pool.scope(|s| {
                s.spawn(|_| fetch = Some(state.fetch()));
                s.spawn(|_| decode = Some(state.decode()));
            });
            for station in state.reservation_stations.iter() {
                executes.push(state.execute_station(&debug_level, station));
            }

            let result = state.apply_stages(fetch.unwrap(), decode.unwrap(), executes);
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
        "Pipelined out-of-order superscalar simulator"
    }
}
