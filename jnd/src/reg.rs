use crate::{errors, vme};

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
            x if x == Register::A as u8 => Register::A,
            x if x == Register::B as u8 => Register::B,
            x if x == Register::C as u8 => Register::C,
            x if x == Register::M as u8 => Register::M,
            x if x == Register::SP as u8 => Register::SP,
            x if x == Register::PC as u8 => Register::PC,
            x if x == Register::BP as u8 => Register::BP,
            x if x == Register::Flags as u8 => Register::Flags,
            _ => return Err(vme!(InvalidRegister, "{}", value)),
        })
    }
}

impl TryFrom<u16> for Register {
    type Error = errors::Jerror;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        (value as u8).try_into()
    }
}
