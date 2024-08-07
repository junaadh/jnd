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
    Pop,
    #[opcode(0x3)]
    PopRegister(Register),
    #[opcode(0x4)]
    PushRegister(Register),
    #[opcode(0x5)]
    AddStack,
    #[opcode(0x6)]
    SubStack,
    #[opcode(0x7)]
    AddRegister(Register, Register),
    #[opcode(0x8)]
    SubRegister(Register, Register),
    #[opcode(0xb)]
    IfZero(Register),
    #[opcode(0xc)]
    BranchImm(i8),
    #[opcode(0x16)]
    Interrupt(u8),
}
