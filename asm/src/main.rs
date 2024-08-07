use std::{
    env,
    fs::File,
    io::{self, Read, Write},
    process::exit,
};

use jnd::{assembler::Assembler, Res};

fn main() -> Res<()> {
    let args = env::args().collect::<Vec<_>>();

    if args.len() < 2 {
        println!("Usage: {} <file>", args[0]);
        exit(1);
    }

    let mut file = File::open(&args[1]).expect("Failed to open file");
    let mut buf = String::new();

    file.read_to_string(&mut buf).unwrap();
    let mut asm = Assembler::default();

    for word in buf.trim().split("\n") {
        let word = word.split_whitespace().collect::<Vec<_>>();
        if word.is_empty() || word[0] == ";" {
            continue;
        }

        asm.assemble(&word)?;
    }

    let mut stdout = io::stdout().lock();
    stdout.write_all(asm.buffer()).expect("Writing to stdout");
    Ok(())
}
