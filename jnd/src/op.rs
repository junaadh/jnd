use crate::reg::Register;
use jop::Codable;

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
