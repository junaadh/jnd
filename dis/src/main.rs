use std::{env, fs::File, process::exit};

use jnd::{disassembler::Disassembler, op::Op, Res};

fn main() -> Res<()> {
    let args = env::args().collect::<Vec<_>>();

    if args.len() < 2 {
        println!("Usage: {} <file>", args[0]);
        exit(1);
    }

    let mut file = File::open(&args[1]).expect("Failed to open file");

    let mut dis = Disassembler::default();
    dis.read(&mut file);

    let program = dis.dump();
    for word in program {
        let ins = Op::try_from(word)?;
        println!("{ins}");
    }

    Ok(())
}
