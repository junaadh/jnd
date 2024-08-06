use std::{
    env,
    fs::File,
    io::{self, Read, Write},
    process::exit,
};

use jnd::{asme, jassert, op::Code, reg::Register, traits::Codable, Res};

fn main() -> Res<()> {
    let args = env::args().collect::<Vec<_>>();

    if args.len() < 2 {
        println!("Usage: {} <file>", args[0]);
        exit(1);
    }

    let mut file = File::open(&args[1]).expect("Failed to open file");
    let mut buf = String::new();

    file.read_to_string(&mut buf).unwrap();

    let mut out = Vec::new();
    for word in buf.trim().split("\n") {
        // let byte = word.parse::<u8>().unwrap();
        let word = word.split_whitespace().collect::<Vec<_>>();
        if word[0] == ";" {
            continue;
        }

        let code = word[0].parse::<Code>()?;

        match code {
            Code::Nop => continue,
            Code::Push => {
                jassert!(word.len() >= 2, "Push: expected: 2, found: {}", word.len())?;
                let arg = parse_numeric(word[1])?;
                out.push_raw((Code::Push as u16).encode_op().encode_arg(arg));
            }
            Code::PopRegister => {
                jassert!(word.len() >= 2, "Push: expected: 2, found: {}", word.len())?;
                let reg = word[1].parse::<Register>()?;
                out.push_raw(
                    (Code::PopRegister as u16)
                        .encode_op()
                        .encode_reg1(reg as u8),
                );
            }
            Code::AddStack => out.push_raw(Code::AddStack as u16),
            Code::AddRegister => {
                jassert!(
                    word[1..].len() >= 2,
                    "expected: 1, found: {}",
                    word[1..].len()
                )?;
                let r1 = word[1].parse::<Register>()?;
                let r2 = word[2].parse::<Register>()?;

                out.push_raw(
                    (Code::PopRegister as u16)
                        .encode_op()
                        .encode_reg2(r1 as u8, r2 as u8),
                );
            }
            Code::Interrupt => {
                jassert!(word.len() >= 2, "expected: 2, found: {}", word.len())?;
                let arg = parse_numeric(word[1])?;
                out.push_raw((Code::Interrupt as u16).encode_op().encode_arg(arg));
            }
        }
    }

    let mut stdout = io::stdout().lock();
    stdout.write_all(&out).expect("Writing to stdout");
    Ok(())
}

fn parse_numeric(value: &str) -> Res<u8> {
    let (base, offset) = match value {
        x if &x[..2] == "0x" => (16, 2),
        x if &x[..1] == "b" => (2, 1),
        x if &x[..1] == "#" => (10, 1),
        _ => (16, 0),
    };
    u8::from_str_radix(&value[offset..], base).map_err(|x| asme!(Parseu8, "{x}"))
}

pub trait WriteEncoded {
    fn push_raw(&mut self, value: u16);
}

impl WriteEncoded for Vec<u8> {
    fn push_raw(&mut self, value: u16) {
        self.push((value & 0xff) as u8);
        self.push((value >> 8) as u8);
    }
}
