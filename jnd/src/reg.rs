use crate::{errors, op::Op, vme};

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Register {
    A,
    B,
    C,
    M,
    SP,
    PC,
    BP,
    Flags,
}

impl TryFrom<u8> for Register {
    type Error = errors::Jerror;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Register::A,
            1 => Register::B,
            2 => Register::C,
            3 => Register::M,
            4 => Register::SP,
            5 => Register::PC,
            6 => Register::BP,
            7 => Register::Flags,
            _ => return Err(vme!(InvalidRegister, "{}", value)),
        })
    }
}

impl TryFrom<u16> for Register {
    type Error = errors::Jerror;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        ((value & 0xff) as u8).try_into()
    }
}

#[macro_export]
macro_rules! reg {
    ($kind: ident) => {{
        $crate::reg::Register::$kind as u8
    }};
}

pub trait Parser {
    fn encode(&self) -> u16;
    fn decode(&self) -> Op;
}
