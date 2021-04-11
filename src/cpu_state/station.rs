use crate::cpu_state::decode::DecodedInstruction;
use crate::memory::Memory;
use crate::registers::ids::{CPSR, PC};
use crate::registers::{ConditionFlag, RegisterFile};
use capstone::arch::arm::{ArmCC, ArmOpMem, ArmOperand, ArmOperandType, ArmShift};
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
    pub memory: Arc<RwLock<Memory>>,
}

impl ReservationStation {
    pub fn new(id: usize, memory: Arc<RwLock<Memory>>) -> Self {
        Self {
            id,
            instruction: None,
            source_registers: Default::default(),
            memory,
        }
    }

    pub fn clear(&mut self) {
        self.instruction = None;
        self.source_registers.clear();
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
        let gen_msg = || {
            format!("Tried to read unknown register {} - did the instruction report source_registers() correctly?", RegisterFile::reg_name(id))
        };
        let k = self.source_registers.get(&id).expect(gen_msg().as_str());
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
    pub fn evaluate_condition_code(&self) -> bool {
        let cc = self.instruction.as_ref().unwrap().cc;
        if let ArmCC::ARM_CC_AL = cc {
            return true;
        }
        let cpsr = self.read_by_id(CPSR);
        let n = ConditionFlag::N.read_flag(cpsr);
        let c = ConditionFlag::C.read_flag(cpsr);
        let z = ConditionFlag::Z.read_flag(cpsr);
        let v = ConditionFlag::V.read_flag(cpsr);
        return match cc {
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
                    let val = changes.iter().find(|(a, _)| a == id).unwrap().1;
                    *reg = Register::Ready(val);
                }
            }
        }
    }

    pub fn eval_ldr_str_op_mem(&self, op_mem: &ArmOpMem) -> u32 {
        /* PC appears WORD aligned to LDR/STR PC relative instructions
          PC always appears as the current instruction address + 4 bytes - even in Thumb state
        * https://community.arm.com/developer/ip-products/processors/f/cortex-m-forum/4541/real-value-of-pc-register/11430#11430
        */
        let base_reg_val = if op_mem.base() == PC {
            let pc_val = self.read_by_id(PC) as i64 + 4;
            pc_val & 0xFFFFFFFC
        } else {
            self.read_by_id(op_mem.base()) as i64
        };

        // Immediate offset
        let displacement: i32 = if op_mem.index().0 == 0 {
            op_mem.disp()
        } else {
            // Register offset
            let index_val = self.read_by_id(op_mem.index());
            (index_val as i32) * op_mem.scale() // Scale for index register (can be 1, or -1)
        };
        let result = base_reg_val + displacement as i64;
        result as u32
    }
}
