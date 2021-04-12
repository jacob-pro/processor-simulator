use super::Instruction;
use crate::cpu_state::station::ReservationStation;
use crate::instructions::PollResult;
use capstone::RegId;
use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct NOP {}

impl NOP {
    pub fn new() -> Self {
        Self {}
    }
}

impl Instruction for NOP {
    fn poll(&self, _station: &ReservationStation) -> PollResult {
        PollResult::Complete(vec![])
    }

    fn source_registers(&self) -> HashSet<RegId> {
        hashset![]
    }

    fn dest_registers(&self) -> HashSet<RegId> {
        hashset![]
    }
}
