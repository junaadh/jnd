use std::{
    env,
    fs::File,
    io::{self, Read, Write},
    process::exit,
};

fn main() {
    let args = env::args().collect::<Vec<_>>();

    if args.len() < 2 {
        println!("Usage: {} <file>", args[0]);
        exit(1);
    }

    let mut file = File::open(&args[1]).expect("Failed to open file");
    let mut buf = String::new();

    file.read_to_string(&mut buf).unwrap();

    let mut out = Vec::new();
    for word in buf.split_whitespace() {
        // let byte = word.parse::<u8>().unwrap();
        let byte = u8::from_str_radix(word, 16).unwrap();
        out.push(byte);
    }
    let mut stdout = io::stdout().lock();
    stdout.write_all(&out).expect("Writing to stdout");
}
