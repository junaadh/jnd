use std::{fs::File, io::Read};

use crate::{reg::Register, vm::Machine};

pub trait Disassemble {
    fn print_reg(&self);
    fn print_mem_map(&self);
}

impl Disassemble for Machine {
    fn print_reg(&self) {
        println!("== Registers ==");

        (0..8u8).for_each(|i| {
            let reg = Register::try_from(i).unwrap();
            if reg != Register::Flags {
                println!("  {reg:<7}: {}", self.get_reg(reg));
            } else {
                let val = self.get_reg(reg);
                println!("  {reg:<7}: {val}  {:016b}", val);
            }
        });

        println!("===============");
    }

    fn print_mem_map(&self) {
        todo!()
    }
}

#[derive(Debug, Default)]
pub struct Disassembler {
    internal_buf: Vec<u8>,
}

impl Disassembler {
    pub fn read(&mut self, file: &mut File) {
        file.read_to_end(&mut self.internal_buf).unwrap();
    }

    pub fn dump(&mut self) -> Vec<u16> {
        assert!(
            self.internal_buf.len() % 2 == 0,
            "Internal buffer is invalid"
        );

        self.internal_buf
            .chunks_exact(2)
            .map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]]))
            .collect::<Vec<_>>()
    }
}
