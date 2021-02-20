use std::convert::TryInto;

pub struct Memory {
    text: Vec<u8>,
    stack: Vec<u8>,
}

impl Memory {

    pub fn initialise(text: Vec<u8>, stack_size: u32) -> Self {
        Memory{ text, stack: vec![0; stack_size as usize] }
    }

    fn stack_start(&self) -> u32 {
        std::u32::MAX - self.stack.len() as u32
    }

    pub fn read_bytes(&self, base_address: u32, length: u32) -> &[u8] {
        if (base_address as usize) < self.text.len() {
            if ((base_address + length) as usize) <= self.text.len() {
                return &self.text[base_address as usize..(base_address + length) as usize]
            }
        } else if base_address > self.stack_start() {
            if base_address.checked_add(length - 1).is_some() {
                let mapped = base_address - self.stack_start();
                return &self.stack[mapped as usize..(mapped + length) as usize]
            }
        }
        panic!("Invalid memory address range {} + {}", base_address, length)
    }

    pub fn read_u32(&self, address: u32) -> u32 {
        let bytes = self.read_bytes(address, 4);
        u32::from_le_bytes( bytes.try_into().unwrap())
    }

    pub fn write_bytes(&mut self, base_address: u32, bytes: &[u8]) {
        if (base_address as usize) < self.text.len() {
            panic!("Text section is read only")
        }
        if base_address > self.stack_start() {
            if base_address.checked_add(bytes.len() as u32 - 1).is_some() {
                let mapped = base_address - self.stack_start();
                self.stack[mapped as usize..(mapped as usize + bytes.len())].copy_from_slice(bytes);
                return;
            }
        }
        panic!("Invalid memory address range {} + {}", base_address, bytes.len())
    }

}
