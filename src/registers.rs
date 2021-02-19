
#[derive(Default)]
pub struct RegisterFile {
    pub gprs: [u32; 13],
    pub sp: u32,
    pub lr: u32,
    pub pc: u32,
    pub psr: u32,
}

impl RegisterFile {

    pub fn get(&mut self, name: &str) -> &mut u32 {
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

}
