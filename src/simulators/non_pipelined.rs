use crate::cpu_state::CpuState;
use crate::memory::Memory;
use crate::simulators::{SimulationStats, Simulator};
use crate::DebugLevel;

pub struct NonPipelinedSimulator {}

impl Simulator for NonPipelinedSimulator {
    fn run(&self, memory: Memory, entry: u32, debug_level: &DebugLevel) -> SimulationStats {
        let mut state = CpuState::new(memory, entry, 1);
        let mut stats = SimulationStats::default();
        loop {
            stats.total_cycles = stats.total_cycles + 1;
            let fetch = state.fetch();
            state.update(fetch, None, vec![None]);

            stats.total_cycles = stats.total_cycles + 1;
            let decode = state.decode();
            state.update(None, decode, vec![None]);

            while state
                .reservation_stations
                .first()
                .unwrap()
                .instruction
                .is_some()
            {
                stats.total_cycles = stats.total_cycles + 1;
                let execute =
                    state.execute(&debug_level, state.reservation_stations.first().unwrap());
                let result = state.update(None, None, vec![Some(execute)]);
                stats.update(&result);
            }

            if state.should_terminate {
                break;
            }
        }
        stats
    }

    fn name(&self) -> &'static str {
        "Non pipelined scalar simulator"
    }
}
