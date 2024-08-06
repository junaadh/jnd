use crate::reg::Register;
use jop::Codable;

// pub trait Codable {
//     fn encode_op(&self) -> u16;
//     fn encode_reg1(&self, r1: Register) -> u16;
//     fn encode_reg2(&self, r1: Register, r2: Register) -> u16;
//     fn encode_arg(&self, arg: u8) -> u16;

//     fn decode_op(&self) -> u8;
//     fn decode_reg1(&self) -> Res<Register>;
//     fn decode_reg2(&self) -> Res<(Register, Register)>;
//     fn decode_arg(&self) -> u8;
// }

// impl Codable for u16 {
//     fn encode_op(&self) -> u16 {
//         self & 0xff
//     }

//     fn encode_reg1(&self, r1: Register) -> u16 {
//         self.encode_op() | ((r1 as u16) & 0xf) << 8
//     }

//     fn encode_reg2(&self, r1: Register, r2: Register) -> u16 {
//         self.encode_op() | ((r2 as u16) & 0xf) << 8 | ((r1 as u16) & 0xf) << 12
//     }

//     fn encode_arg(&self, arg: u8) -> u16 {
//         self.encode_op() | (arg as u16) << 8
//     }

//     fn decode_op(&self) -> u8 {
//         (self & 0xff) as u8
//     }

//     fn decode_reg1(&self) -> Res<Register> {
//         (((self >> 8) as u8) & 0xf).try_into()
//     }

//     fn decode_reg2(&self) -> Res<(Register, Register)> {
//         let r1 = Register::try_from((self >> 12) as u8)?;
//         let r2 = Register::try_from(((self >> 8) & 0xf) as u8)?;
//         Ok((r1, r2))
//     }

//     fn decode_arg(&self) -> u8 {
//         (self >> 8) as u8
//     }
// }

// macro_rules! retq {
//      ($name: ident, $x: expr) => {
//          $x == $crate::op::Op::$name.as_u8()
//      };

//      ($name:ident($($args:tt)*), $x: expr) => {
//          $x == $crate::op::Op::$name($($args)*).as_u8()
//      };
//  }

#[derive(Debug, PartialEq, PartialOrd, Codable)]
#[repr(u8)]
pub enum Op {
    #[opcode(0x0)]
    Nop,
    #[opcode(0x1)]
    Push(u8),
    #[opcode(0x2)]
    PopRegister(Register),
    #[opcode(0x3)]
    AddStack,
    #[opcode(0x4)]
    AddRegister(Register, Register),
    #[opcode(0x5)]
    Interrupt(u8),
}

// pub trait Parser<T> {
//     fn parse(self) -> Res<T>;
// }

// impl Parser<Op> for u16 {
//     fn parse(self) -> Res<Op> {
//         crate::op::Op::parse(self)
//     }
// }

// impl Op {
//     pub fn as_u8(&self) -> u8 {
//         // SAFETY: Because `Self` is marked `repr(u8)`, its layout is a `repr(C)` `union`
//         // between `repr(C)` structs, each of which has the `u8` discriminant as its first
//         // field, so we can read the discriminant without offsetting the pointer.
//         unsafe { *<*const _>::from(self).cast::<u8>() }
//     }

//     fn parse(byte: u16) -> Res<Self> {
//         let op = (byte & 0xff) as u8;
//         Ok(match op {
//             x if retq!(Nop, x) => Op::Nop,
//             x if retq!(Push(0), x) => {
//                 let arg = ((byte & 0xff00) >> 8) as u8;
//                 Op::Push(arg)
//             }
//             x if retq!(PopRegister(Register::A), x) => {
//                 let reg: Register = ((byte & 0xf00) >> 8).try_into()?;
//                 Op::PopRegister(reg)
//             }
//             x if retq!(AddStack, x) => Op::AddStack,
//             x if retq!(AddRegister(Register::A, Register::A), x) => {
//                 let reg1 = ((byte & 0xf00) >> 8) as u8;
//                 let reg2 = ((byte & 0xf0) >> 8) as u8;
//                 Op::AddRegister(reg1.try_into()?, reg2.try_into()?)
//             }
//             x if retq!(Interrupt(0), x) => {
//                 let arg = ((byte & 0xff00) >> 8) as u8;
//                 Op::Interrupt(arg)
//             }
//             _ => return Err(vme!(UnknownInstruction, "found 0x{byte:X}")),
//         })
//     }
// }

// // impl FromStr for Op {
// //     type Err = errors::Jerror;

// //     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         let byte = (match s.to_lowercase().as_str() {
//             "nop" => Op::Nop,
//             ""
//             _ => return Err(vme!(UnknownInstruction, "instruction: {s}")),
//         })
//     }
// }
