use crate::simulator::Simulator;
use crate::DebugLevel;
use std::time::Instant;

pub struct PipelinedSimulator {}

impl PipelinedSimulator {
    pub fn run(mut simulator: Simulator, debug_level: &DebugLevel) {
        let start_time = Instant::now();
        let mut cycle_counter = 0;
        loop {
            cycle_counter = cycle_counter + 1;
            let fetch_changes = simulator.fetch();

            // fetch_changes.apply(&mut simulator);

            let decode_changes = simulator.decode();

            fetch_changes.apply(&mut simulator);
            decode_changes.apply(&mut simulator);

            simulator = simulator.execute(&debug_level);
            if simulator.registers.changed_pc {
                simulator.flush_pipeline();
                simulator.registers.changed_pc = false;
            }
            if simulator.executed_instruction {
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
