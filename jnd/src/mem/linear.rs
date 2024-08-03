use super::Addressable;
use crate::{vme, Res};

#[derive(Debug)]
pub struct LinearMemory {
    bytes: Vec<u8>,
}

impl LinearMemory {
    pub fn new(n: usize) -> Self {
        Self {
            bytes: vec![0u8; n],
        }
    }
}

impl Addressable for LinearMemory {
    fn read(&self, addr: u16) -> Option<u8> {
        self.bytes.get(addr as usize).copied()
    }

    fn write(&mut self, addr: u16, value: u8) -> Res<()> {
        let addr = addr as usize;
        if addr > self.bytes.len() {
            return Err(vme!(WriteOutOfBounds, "addr: 0x{addr:X} - values: {value}"));
        }

        self.bytes[addr] = value;
        Ok(())
    }
}
