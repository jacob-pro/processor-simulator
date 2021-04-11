use crate::decoded::DecodedInstruction;
use crate::memory::Memory;
use crate::registers::ids::CPSR;
use crate::registers::ConditionFlag;
use capstone::arch::arm::{ArmCC, ArmOperand, ArmOperandType, ArmShift};
use capstone::RegId;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};

pub enum Register {
    Ready(u32),
    Pending(usize, RegId),
}

pub struct ReservationStation {
    pub id: usize,
    pub instruction: Option<DecodedInstruction>,
    source_registers: HashMap<RegId, Register>,
    output_registers: HashSet<RegId>,
    pub memory: Arc<RwLock<Memory>>,
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

    pub fn issue(
        &mut self,
        instruction: DecodedInstruction,
        source_registers: HashMap<RegId, Register>,
    ) {
        self.instruction = Some(instruction);
        self.source_registers = source_registers;
    }

    pub fn read_by_id(&self, id: RegId) -> u32 {
        let k = self.source_registers.get(&id).unwrap();
        match k {
            Register::Ready(value) => *value,
            Register::Pending(_, _) => panic!(),
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

    pub fn ready(&self) -> bool {
        if self.instruction.is_none() {
            return false;
        }
        for (_, r) in &self.source_registers {
            if let Register::Pending(_, _) = r {
                return false;
            }
        }
        true
    }

    // https://community.arm.com/developer/ip-products/processors/b/processors-ip-blog/posts/condition-codes-1-condition-flags-and-codes
    pub fn should_execute(&self) -> bool {
        let cpsr = self.read_by_id(CPSR);
        let n = ConditionFlag::N.read_flag(cpsr);
        let c = ConditionFlag::C.read_flag(cpsr);
        let z = ConditionFlag::Z.read_flag(cpsr);
        let v = ConditionFlag::V.read_flag(cpsr);
        return match &self.instruction.as_ref().unwrap().cc {
            ArmCC::ARM_CC_INVALID => panic!("CC Invalid"),
            ArmCC::ARM_CC_EQ => z == true,
            ArmCC::ARM_CC_NE => z == false,
            ArmCC::ARM_CC_HS => c == true,
            ArmCC::ARM_CC_LO => c == false,
            ArmCC::ARM_CC_MI => n == true,
            ArmCC::ARM_CC_PL => n == false,
            ArmCC::ARM_CC_VS => v == true,
            ArmCC::ARM_CC_VC => v == false,
            ArmCC::ARM_CC_HI => c == true && z == false,
            ArmCC::ARM_CC_LS => c == false || z == true,
            ArmCC::ARM_CC_GE => n == v,
            ArmCC::ARM_CC_LT => n != v,
            ArmCC::ARM_CC_GT => z == false && n == v,
            ArmCC::ARM_CC_LE => z == true || n != v,
            ArmCC::ARM_CC_AL => true,
        };
    }

    pub fn receive_broadcast(&mut self, source_id: usize, changes: &Vec<(RegId, u32)>) {
        for (_, reg) in &mut self.source_registers {
            if let Register::Pending(station_id, id) = reg {
                if *station_id == source_id {
                    let val = changes.iter().find(|(a, b)| a == id).unwrap().1;
                    *reg = Register::Ready(val);
                }
            }
        }
    }
}
