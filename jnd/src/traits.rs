pub trait Codable {
    fn encode_op(&self) -> u16;
    fn encode_reg1(&self, r1: u8) -> u16;
    fn encode_reg2(&self, r1: u8, r2: u8) -> u16;
    fn encode_arg(&self, arg: u8) -> u16;

    fn decode_op(&self) -> u8;
    fn decode_reg1(&self) -> u8;
    fn decode_reg2(&self) -> (u8, u8);
    fn decode_arg(&self) -> u8;
}

pub trait WriteEncoded {
    fn push_raw(&mut self, value: u16);
}

#[cfg(test)]
mod test {
    use crate::reg::Register;

    use super::*;

    #[test]
    fn test_encode_op() {
        let instruction: u16 = 0x12;
        assert_eq!(instruction.encode_op(), 0x12);
    }

    #[test]
    fn test_encode_reg1() {
        let instruction: u16 = 0x12;
        assert_eq!(instruction.encode_reg1(Register::B as u8), 0x112);
    }

    #[test]
    fn test_encode_reg2() {
        let instruction: u16 = 0x12;
        assert_eq!(
            instruction.encode_reg2(Register::B as u8, Register::C as u8),
            0x2112
        );
    }

    #[test]
    fn test_encode_arg() {
        let instruction: u16 = 0x12;
        assert_eq!(instruction.encode_arg(0x34), 0x3412);
    }

    #[test]
    fn test_decode_op() {
        let encoded: u16 = 0x3412;
        assert_eq!(encoded.decode_op(), 0x12);
    }

    #[test]
    fn test_decode_reg1() {
        let encoded: u16 = 0x112;
        assert_eq!(encoded.decode_reg1(), Register::B as u8);
    }

    #[test]
    fn test_decode_reg2() {
        let encoded: u16 = 0x1012;
        assert_eq!(
            encoded.decode_reg2(),
            (Register::A as u8, Register::B as u8)
        );
    }

    #[test]
    fn test_decode_arg() {
        let encoded: u16 = 0x3412;
        assert_eq!(encoded.decode_arg(), 0x34);
    }
}
