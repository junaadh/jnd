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
            println!("  {reg:<7}: {}", self.get_reg(reg));
        });

        println!("===============");
    }

    fn print_mem_map(&self) {
        todo!()
    }
}
