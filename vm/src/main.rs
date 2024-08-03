use std::{env, fs, io::Read, process::exit};

use jnd::{interrupts::halt, vm::Machine, Res};

fn main() -> Res<()> {
    let args = env::args().collect::<Vec<_>>();

    if args.len() < 2 {
        println!("Usage: {} <binary>", &args[0]);
        exit(1);
    }

    let mut internal_buf = Vec::new();
    let mut file = fs::File::open(&args[1]).expect("Failed to open file");
    file.read_to_end(&mut internal_buf)
        .expect("Failed to read file");

    let mut vm = Machine::default();
    vm.insert_interrupt(0xf0_u16, halt);

    vm.load_vector(&internal_buf, 0)?;

    /*
     * push 10
     * push 8
     * addstack
     * popregister
     */

    // vm.mem_write(0, 0x1)?;
    // vm.mem_write(1, 10)?;
    // vm.mem_write(2, 0x1)?;
    // vm.mem_write(3, 8)?;
    // vm.mem_write(4, 0x3)?;
    // vm.mem_write(6, 0x2)?;
    // vm.mem_write(7, 0x0)?;

    // vm.step()?;
    // vm.step()?;
    // vm.step()?;
    // vm.step()?;

    vm.execute()?;
    println!("A = {}", vm.get_reg(jnd::reg::Register::A));

    Ok(())
}
