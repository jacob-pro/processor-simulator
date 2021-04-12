use std::convert::TryInto;
use std::fmt::{Display, Formatter};

struct Page {
    write: bool,
    data: Vec<u8>,
    vaddr: u32,
}

#[derive(Default)]
pub struct Memory {
    pages: Vec<Page>,
}

#[derive(Debug, Clone)]
pub enum MemoryAccessError {
    BadAddress(u32),
    ReadOnlyAddress(u32),
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

    pub fn read_byte(&self, address: u32) -> Result<u8, MemoryAccessError> {
        for p in &self.pages {
            let p_end_addr = p.vaddr + p.data.len() as u32;
            if address >= p.vaddr && address < p_end_addr {
                let adj_addr = address - p.vaddr;
                return Ok(p.data[adj_addr as usize]);
            }
        }
        Err(MemoryAccessError::BadAddress(address))
    }

    pub fn write_byte(&mut self, address: u32, byte: u8) -> Result<(), MemoryAccessError> {
        for p in &mut self.pages {
            let p_end_addr = p.vaddr + p.data.len() as u32;
            if address >= p.vaddr && address < p_end_addr {
                let adj_addr = address - p.vaddr;
                if p.write {
                    p.data[adj_addr as usize] = byte;
                    return Ok(());
                } else {
                    return Err(MemoryAccessError::ReadOnlyAddress(address));
                }
            }
        }
        Err(MemoryAccessError::BadAddress(address))
    }

    pub fn read_bytes(&self, base_address: u32, length: u32) -> Result<Vec<u8>, MemoryAccessError> {
        let mut ret = Vec::with_capacity(length as usize);
        for i in 0..length {
            ret.push(self.read_byte(base_address + i)?)
        }
        Ok(ret)
    }

    pub fn read_u32(&self, address: u32) -> Result<u32, MemoryAccessError> {
        let bytes = self.read_bytes(address, 4)?;
        Ok(u32::from_le_bytes(bytes.as_slice().try_into().unwrap()))
    }

    pub fn read_u16(&self, address: u32) -> Result<u16, MemoryAccessError> {
        let bytes = self.read_bytes(address, 2)?;
        Ok(u16::from_le_bytes(bytes.as_slice().try_into().unwrap()))
    }

    pub fn write_bytes(
        &mut self,
        base_address: u32,
        bytes: &[u8],
    ) -> Result<(), MemoryAccessError> {
        for i in 0..bytes.len() {
            self.write_byte(base_address + i as u32, bytes[i])?;
        }
        Ok(())
    }
}

impl Display for MemoryAccessError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MemoryAccessError::BadAddress(address) => writeln!(
                f,
                "Attempt to access invalid memory address: {:#X}",
                address
            ),
            MemoryAccessError::ReadOnlyAddress(address) => writeln!(
                f,
                "Attempt to write to read only memory address: {:#X}",
                address
            ),
        }
    }
}

impl std::error::Error for MemoryAccessError {}
