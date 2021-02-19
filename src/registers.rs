
#[derive(Default)]
pub struct RegisterFile {
    pub gprs: [u32; 8],
    pub sp: u32,
    pub lr: u32,
    pub pc: u32,
}
