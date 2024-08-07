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
    vm.setuo_stack(0x900);

    vm.load_vector(&internal_buf, 0)?;
    vm.execute()?;
    println!("A = {}", vm.get_reg(jnd::reg::Register::A));

    Ok(())
}
