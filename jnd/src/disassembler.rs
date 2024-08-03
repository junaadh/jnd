use crate::vm::Machine;

pub trait Disassemble {
    fn print_reg(&self);
    fn print_mem_map(&self);
}

impl Disassemble for Machine {
    fn print_reg(&self) {
        println!("== Registers ==");

        for (i, &reg) in ["a", "b", "c", "m", "sp", "pc", "bp", "flags"]
            .iter()
            .enumerate()
        {
            println!(
                "  {reg:<7}: {}",
                self.get_reg((i as u8).try_into().unwrap())
            );
        }

        println!("===============");
    }

    fn print_mem_map(&self) {
        todo!()
    }
}
