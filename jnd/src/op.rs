use crate::{reg::Register, vme, Res};

/// return-eq
macro_rules! retq {
    ($name: ident, $x: expr) => {
        $x == $crate::op::Op::$name.as_u8()
    };

    ($name:ident($($args:tt)*), $x: expr) => {
        $x == $crate::op::Op::$name($($args)*).as_u8()
    };
}

#[derive(Debug, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum Op {
    Nop,
    Push(u8),
    PopRegister(Register),
    AddStack,
    AddRegister(Register, Register),
    Interrupt(u8),
}

pub trait Parser<T> {
    fn parse(self) -> Res<T>;
}

impl Parser<Op> for u16 {
    fn parse(self) -> Res<Op> {
        crate::op::Op::parse(self)
    }
}

impl Op {
    pub fn as_u8(&self) -> u8 {
        // SAFETY: Because `Self` is marked `repr(u8)`, its layout is a `repr(C)` `union`
        // between `repr(C)` structs, each of which has the `u8` discriminant as its first
        // field, so we can read the discriminant without offsetting the pointer.
        unsafe { *<*const _>::from(self).cast::<u8>() }
    }

    fn parse(byte: u16) -> Res<Self> {
        let op = (byte & 0xff) as u8;
        Ok(match op {
            x if retq!(Nop, x) => Op::Nop,
            x if retq!(Push(0), x) => {
                let arg = ((byte & 0xff00) >> 8) as u8;
                Op::Push(arg)
            }
            x if retq!(PopRegister(Register::A), x) => {
                let reg: Register = ((byte & 0xf00) >> 8).try_into()?;
                Op::PopRegister(reg)
            }
            x if retq!(AddStack, x) => Op::AddStack,
            x if retq!(AddRegister(Register::A, Register::A), x) => {
                let reg1 = ((byte & 0xf00) >> 8) as u8;
                let reg2 = ((byte & 0xf0) >> 8) as u8;
                Op::AddRegister(reg1.try_into()?, reg2.try_into()?)
            }
            x if retq!(Interrupt(0), x) => {
                let arg = ((byte & 0xff00) >> 8) as u8;
                Op::Interrupt(arg)
            }
            _ => return Err(vme!(UnknownInstruction, "found 0x{byte:X}")),
        })
    }
}
