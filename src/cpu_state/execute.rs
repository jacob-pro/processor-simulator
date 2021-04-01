use crate::cpu_state::CpuState;
use crate::registers::{ConditionFlag, PC};
use crate::DebugLevel;
use capstone::arch::arm::ArmCC;
use capstone::prelude::*;

#[derive(Default)]
pub struct ExecuteChanges {
    register_changes: Vec<(RegId, u32)>,
    flag_changes: Vec<(ConditionFlag, bool)>,
    should_terminate: bool,
}

impl ExecuteChanges {
    pub fn register_change(&mut self, reg_id: RegId, value: u32) {
        self.register_changes.push((reg_id, value));
    }

    pub fn flag_change(&mut self, flag: ConditionFlag, value: bool) {
        self.flag_changes.push((flag, value));
    }

    pub fn apply(self, sim: &mut CpuState) -> bool {
        let mut changed_pc = false;
        for (reg_id, value) in self.register_changes {
            sim.registers.write_by_id(reg_id, value);
            if reg_id == PC {
                changed_pc = true;
            }
        }
        for (flag, value) in self.flag_changes {
            sim.registers.cond_flags.write_flag(flag, value);
        }
        sim.should_terminate = self.should_terminate;
        changed_pc
    }
}

impl CpuState {
    pub fn execute(&self, debug: &DebugLevel) -> ExecuteChanges {
        let mut changes = ExecuteChanges::default();
        if self.decoded_instruction.is_none() {
            return changes;
        }
        let dec = self.decoded_instruction.clone().unwrap();
        let ex = self.should_execute(&dec.cc);
        if *debug >= DebugLevel::Minimal {
            let mut output = String::new();
            if ex {
                output.push_str(&dec.string);
            } else {
                output.push_str(&format!("{} (omitted)", dec.string));
            }
            if *debug >= DebugLevel::Full {
                let padding: String = vec![' '; 30 as usize - output.len()].iter().collect();
                output.push_str(&format!("{} [{}]", padding, self.registers.debug_string()));
            }
            println!("{}", output);
        }
        if ex {
            changes.should_terminate = dec.imp.execute(self, &mut changes);
        }
        changes
    }

    // https://community.arm.com/developer/ip-products/processors/b/processors-ip-blog/posts/condition-codes-1-condition-flags-and-codes
    fn should_execute(&self, cc: &ArmCC) -> bool {
        let flags = &self.registers.cond_flags;
        return match cc {
            ArmCC::ARM_CC_INVALID => panic!("CC Invalid"),
            ArmCC::ARM_CC_EQ => flags.z == true,
            ArmCC::ARM_CC_NE => flags.z == false,
            ArmCC::ARM_CC_HS => flags.c == true,
            ArmCC::ARM_CC_LO => flags.c == false,
            ArmCC::ARM_CC_MI => flags.n == true,
            ArmCC::ARM_CC_PL => flags.n == false,
            ArmCC::ARM_CC_VS => flags.v == true,
            ArmCC::ARM_CC_VC => flags.v == false,
            ArmCC::ARM_CC_HI => flags.c == true && flags.z == false,
            ArmCC::ARM_CC_LS => flags.c == false || flags.z == true,
            ArmCC::ARM_CC_GE => flags.n == flags.v,
            ArmCC::ARM_CC_LT => flags.n != flags.v,
            ArmCC::ARM_CC_GT => flags.z == false && flags.n == flags.v,
            ArmCC::ARM_CC_LE => flags.z == true || flags.n != flags.v,
            ArmCC::ARM_CC_AL => true,
        };
    }
}
