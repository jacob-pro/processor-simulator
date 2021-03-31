use std::convert::TryInto;

struct Page {
    write: bool,
    data: Vec<u8>,
    vaddr: u32,
}

#[derive(Default)]
pub struct Memory {
    pages: Vec<Page>,
}

impl Memory {
    pub fn mmap(&mut self, address: u32, data: Vec<u8>, write: bool) {
        for existing_p in &self.pages {
            let existing_p_end = existing_p.vaddr + existing_p.data.len() as u32;
            let new_p_end = address + data.len() as u32;
            if address >= existing_p.vaddr && address < existing_p_end {
                panic!("Cannot create page here");
            }
            if new_p_end >= existing_p.vaddr && new_p_end < existing_p_end {
                panic!("Cannot create page here");
            }
        }
        self.pages.push(Page {
            write,
            data,
            vaddr: address,
        });
    }

    pub fn read_byte(&self, address: u32) -> u8 {
        for p in &self.pages {
            let p_end_addr = p.vaddr + p.data.len() as u32;
            if address >= p.vaddr && address < p_end_addr {
                let adj_addr = address - p.vaddr;
                return p.data[adj_addr as usize];
            }
        }
        panic!("Invalid memory address {:#X}", address)
    }

    pub fn write_byte(&mut self, address: u32, byte: u8) {
        for p in &mut self.pages {
            let p_end_addr = p.vaddr + p.data.len() as u32;
            if address >= p.vaddr && address < p_end_addr {
                let adj_addr = address - p.vaddr;
                if p.write {
                    p.data[adj_addr as usize] = byte;
                    return;
                } else {
                    panic!(
                        "Tried to write to read only memory page, address {:#X}",
                        address
                    );
                }
            }
        }
        panic!("Invalid memory address {:#X}", address)
    }

    pub fn read_bytes(&self, base_address: u32, length: u32) -> Vec<u8> {
        let mut ret = Vec::with_capacity(length as usize);
        for i in 0..length {
            ret.push(self.read_byte(base_address + i))
        }
        ret
    }

    pub fn read_u32(&self, address: u32) -> u32 {
        let bytes = self.read_bytes(address, 4);
        u32::from_le_bytes(bytes.as_slice().try_into().unwrap())
    }

    pub fn read_u16(&self, address: u32) -> u16 {
        let bytes = self.read_bytes(address, 2);
        u16::from_le_bytes(bytes.as_slice().try_into().unwrap())
    }

    pub fn write_bytes(&mut self, base_address: u32, bytes: &[u8]) {
        for i in 0..bytes.len() {
            self.write_byte(base_address + i as u32, bytes[i]);
        }
    }
}
