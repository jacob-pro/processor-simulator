use std::collections::{HashMap, HashSet};
use capstone::RegId;
use std::rc::Rc;
use std::sync::{RwLock, Arc};
use crate::memory::Memory;
use crate::decoded::DecodedInstruction;
use capstone::arch::arm::{ArmOperand, ArmOperandType, ArmShift};

pub enum Register {
    Ready(u32),
    Pending(usize),
}

pub struct ReservationStation {
    pub id: usize,
    pub instruction: Option<DecodedInstruction>,
    source_registers: HashMap<RegId, Register>,
    output_registers: HashSet<RegId>,
    pub memory: Arc<RwLock<Memory>>
}

impl ReservationStation {

    pub fn new(id: usize, memory: Arc<RwLock<Memory>>) -> Self {
        Self {
            id,
            instruction: None,
            source_registers: Default::default(),
            output_registers: Default::default(),
            memory,
        }
    }

    pub fn clear(&mut self) {
        self.instruction = None;
        self.source_registers.clear();
        self.output_registers.clear();
    }

    pub fn issue(&mut self, instruction: DecodedInstruction, source_registers: HashMap<RegId, Register>) {
        self.instruction = Some(instruction);
        self.source_registers = source_registers;
    }

    pub fn read_by_id(&self, id: RegId) -> u32 {
        let k = self.source_registers.get(&id).unwrap();
        match k {
            Register::Ready(value) => *value,
            Register::Pending(_) => panic!(),
        }
    }

    pub fn value_of_flexible_second_operand(&self, op: &ArmOperand) -> u32 {
        match op.op_type {
            ArmOperandType::Reg(reg_id) => {
                assert!(op.shift == ArmShift::Invalid, "Shift not supported");
                self.read_by_id(reg_id)
            }
            ArmOperandType::Imm(value) => value as u32,
            _ => panic!("Unsupported type"),
        }
    }

}