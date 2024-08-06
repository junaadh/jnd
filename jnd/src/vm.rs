use std::collections::HashMap;

use crate::{
    disassembler::Disassemble,
    mem::{linear::LinearMemory, Addressable},
    op::Op,
    reg::Register,
    vme, Res,
};

pub type Interrupt = fn(&mut Machine) -> Res<()>;

pub struct Machine {
    registers: [u16; 8],
    interrupt_map: HashMap<u16, Interrupt>,
    pub state: bool,
    memory: Box<dyn Addressable>,
}

impl Default for Machine {
    fn default() -> Self {
        Self {
            registers: [0u16; 8],
            interrupt_map: HashMap::new(),
            state: false,
            memory: Box::new(LinearMemory::new(4096)),
        }
    }
}

impl Machine {
    pub fn mem_write(&mut self, addr: u16, byte: u8) -> Res<()> {
        self.memory.write(addr, byte)
    }

    pub fn mem_write2(&mut self, addr: u16, byte: u16) -> Res<()> {
        self.memory.write_u16(addr, byte)
    }

    pub fn mem_read(&self, addr: u16) -> Res<u8> {
        let sp = self.registers[Register::SP as usize];
        self.memory
            .read(addr)
            .ok_or(vme!(MemReadFail, "addr: 0x{addr:X} Sp: 0x{sp:X}"))
    }

    pub fn mem_read2(&self, addr: u16) -> Res<u16> {
        let sp = self.registers[Register::SP as usize];
        self.memory
            .read_u16(addr)
            .ok_or(vme!(MemRead2Fail, "addr: 0x{addr:X} Sp: 0x{sp:X}"))
    }

    pub fn push(&mut self, value: u16) -> Res<()> {
        let sp = self.registers[Register::SP as usize];
        self.mem_write2(sp, value)?;
        self.registers[Register::SP as usize] += 2;
        Ok(())
    }

    pub fn pop(&mut self) -> Res<u16> {
        let sp = self.registers[Register::SP as usize] - 2;
        let val = self.mem_read2(sp)?;
        self.registers[Register::SP as usize] -= 2;
        Ok(val)
    }

    pub fn get_reg(&self, r: Register) -> u16 {
        self.registers[r as usize]
    }

    fn get_interrupt_handler(&self, interrupt: u16) -> Res<&Interrupt> {
        self.interrupt_map
            .get(&interrupt)
            .ok_or(vme!(InterruptHandlerNotFound, "interrupt 0x{interrupt:X}"))
    }

    pub fn insert_interrupt(&mut self, interrupt: u16, int_fn: Interrupt) {
        self.interrupt_map.insert(interrupt, int_fn);
    }

    pub fn step(&mut self) -> Res<()> {
        let pc = self.registers[Register::PC as usize];
        let instruction = self.mem_read2(pc)?;
        self.registers[Register::PC as usize] = pc + 2;

        let op = Op::try_from(instruction)?;
        match op {
            Op::Nop => Ok(()),
            Op::Push(val) => self.push(val as u16),
            Op::PopRegister(r) => {
                let v = self.pop()?;
                self.registers[r as usize] = v;
                Ok(())
            }
            Op::AddStack => {
                let a = self.pop()?;
                let b = self.pop()?;
                self.push(a + b)
            }
            Op::AddRegister(r1, r2) => {
                self.registers[r1 as usize] += self.registers[r2 as usize];
                Ok(())
            } // _ => Err(format!("unknown operand: {op:?} at {pc}").into()),
            Op::Interrupt(sig) => {
                let int = self.get_interrupt_handler(sig as u16)?;
                int(self)
            }
        }
    }

    pub fn execute(&mut self) -> Res<()> {
        self.state = true;
        while self.state {
            self.step()?;
        }
        Ok(())
    }

    pub fn load_vector(&mut self, instruction_buf: &[u8], offset: u16) -> Res<()> {
        for (index, &byte) in instruction_buf.iter().enumerate() {
            self.mem_write(offset + (index as u16), byte)?;
        }
        Ok(())
    }
}

impl Drop for Machine {
    fn drop(&mut self) {
        if cfg!(debug_assertions) {
            self.print_reg();
        }
    }
}
