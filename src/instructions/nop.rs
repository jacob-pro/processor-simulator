use super::Instruction;
use crate::instructions::PollResult;
use crate::station::ReservationStation;
use capstone::RegId;

#[derive(Clone)]
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

    fn source_registers(&self) -> Vec<RegId> {
        vec![]
    }

    fn dest_registers(&self) -> Vec<RegId> {
        vec![]
    }
}
