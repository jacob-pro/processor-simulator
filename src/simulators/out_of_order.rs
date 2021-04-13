use crate::cpu_state::CpuState;
use crate::memory::Memory;
use crate::simulators::{SimulationStats, Simulator};
use crate::DebugLevel;
use rayon::prelude::*;

pub struct OutOfOrderSimulator {
    stations: usize,
}

impl OutOfOrderSimulator {
    pub fn new(stations: usize) -> Self {
        assert!(stations > 0);
        Self { stations }
    }
}

impl Simulator for OutOfOrderSimulator {
    fn run(&self, memory: Memory, entry: u32, debug_level: &DebugLevel) -> SimulationStats {
        let mut state = CpuState::new(memory, entry, self.stations);
        let mut stats = SimulationStats::default();
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(2 + self.stations)
            .build()
            .unwrap();

        loop {
            stats.total_cycles = stats.total_cycles + 1;

            let mut fetch = None;
            let mut decode = None;
            let mut executes = None;

            pool.scope(|s| {
                s.spawn(|_| fetch = Some(state.fetch()));
                s.spawn(|_| decode = Some(state.decode()));
                executes = Some(
                    state
                        .reservation_stations
                        .par_iter()
                        .map(|station| state.execute_station(&debug_level, station))
                        .collect(),
                );
            });

            let result = state.apply_stages(fetch.unwrap(), decode.unwrap(), executes.unwrap());
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

    fn name(&self) -> String {
        format!(
            "Pipelined out-of-order superscalar simulator ({} stations / execution units)",
            self.stations
        )
    }
}
