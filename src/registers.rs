use capstone::prelude::*;
use std::rc::Rc;

pub struct RegisterFile {
    pub gprs: [u32; 13],
    pub sp: u32,
    pub lr: u32,
    pub pc: u32,
    pub psr: u32,
    capstone: Rc<Capstone>,
}

impl RegisterFile {
    
    pub fn new(capstone: Rc<Capstone>, pc: u32) -> Self {
        Self {
            gprs: Default::default(),
            sp: std::u32::MAX,
            lr: 0,
            pc,
            psr: 0,
            capstone
        }
    }

    pub fn get_by_name(&mut self, name: &str) -> &mut u32 {
        let name = name.to_ascii_uppercase();
        if name.starts_with("R") {
            let number = name[1..].parse::<usize>().expect("Invalid register");
            return &mut self.gprs[number]
        }
        return match name.as_str() {
            "SP" => &mut self.sp,
            "LR" => &mut self.lr,
            "PC" => &mut self.pc,
            "PSR" => &mut self.psr,
            _ => panic!("Unknown register")
        }
    }

    pub fn get_by_id(&mut self, id: RegId) -> &mut u32 {
        let n = self.capstone.reg_name(id).expect("Couldn't get reg_name");
        self.get_by_name(&n)
    }

}
