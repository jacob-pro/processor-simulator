use crate::cpu_state::CpuState;
use crate::DebugLevel;
use std::time::Instant;

pub struct Simulator {}

impl Simulator {
    pub fn run(mut simulator: CpuState, debug_level: &DebugLevel) {
        let start_time = Instant::now();
        let mut cycle_counter = 0;
        loop {
            cycle_counter = cycle_counter + 1;

            let fetch_changes = simulator.fetch();
            let decode_changes = simulator.decode();
            let ex_changes = simulator.execute(&debug_level);

            fetch_changes.apply(&mut simulator);
            decode_changes.apply(&mut simulator);
            match ex_changes.apply(&mut simulator) {
                None => {}
                Some(pc) => {
                    simulator.flush_pipeline();
                    simulator.next_instr_addr = pc;
                }
            }
            if simulator.should_terminate {
                break;
            }
        }
        let elapsed = start_time.elapsed();
        println!(
            "Simulator run {} cycles in {} seconds",
            cycle_counter,
            elapsed.as_millis() as f64 / 1000.0
        );
    }
}
