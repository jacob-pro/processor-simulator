use super::Instruction;
use crate::cpu_state::CpuState;
use capstone::RegId;
use crate::station::ReservationStation;
use crate::instructions::PollResult;

#[derive(Clone)]
pub struct NOP {}

impl NOP {
    pub fn new() -> Self {
        Self {}
    }
}

impl Instruction for NOP {
    fn poll(&self, station: &ReservationStation) -> PollResult {
        PollResult::Complete(vec![])
    }

    fn source_registers(&self) -> Vec<RegId> {
        vec![]
    }

    fn dest_registers(&self) -> Vec<RegId> {
        vec![]
    }
}
