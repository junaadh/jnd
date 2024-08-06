use core::fmt;
use std::str::FromStr;

use crate::{asme, errors, vme};

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
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

impl FromStr for Register {
    type Err = errors::Jerror;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "%a" | "%A" => Register::A,
            "%b" | "%B" => Register::B,
            "%c" | "%C" => Register::C,
            "%m" | "%M" => Register::M,
            "%sp" | "%SP" => Register::SP,
            "%pc" | "%PC" => Register::PC,
            "%bp" | "%BP" => Register::BP,
            "%flags" | "%FLAGS" => Register::Flags,
            _ => return Err(asme!(ParseReg, "found: {s}")),
        })
    }
}

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::A => write!(f, "a"),
            Self::B => write!(f, "b"),
            Self::C => write!(f, "c"),
            Self::M => write!(f, "m"),
            Self::SP => write!(f, "sp"),
            Self::PC => write!(f, "pc"),
            Self::BP => write!(f, "bp"),
            Self::Flags => write!(f, "flags"),
        }
    }
}

#[macro_export]
macro_rules! reg {
    ($kind: ident) => {{
        $crate::reg::Register::$kind as u8
    }};
}
