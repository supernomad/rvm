#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Opcode {
    Literal = 0x00,
    Addition = 0x01,
    Subtract = 0x02,
    Multiply = 0x03,
    Divide = 0x04,
    Modulo = 0x05,
    Return = 0x06,
    Factorial = 0x07,
}

impl From<u8> for Opcode {
    fn from(value: u8) -> Self {
        match value {
            0x00 => Opcode::Literal,
            0x01 => Opcode::Addition,
            0x02 => Opcode::Subtract,
            0x03 => Opcode::Multiply,
            0x04 => Opcode::Divide,
            0x05 => Opcode::Modulo,
            0x06 => Opcode::Return,
            0x07 => Opcode::Factorial,
            _ => panic!("invalid opcode"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(0x00, Opcode::Literal)]
    #[case(0x01, Opcode::Addition)]
    #[case(0x02, Opcode::Subtract)]
    #[case(0x03, Opcode::Multiply)]
    #[case(0x04, Opcode::Divide)]
    #[case(0x05, Opcode::Modulo)]
    #[case(0x06, Opcode::Return)]
    #[case(0x07, Opcode::Factorial)]
    fn test_valid_opcodes(#[case] input: u8, #[case] expected: Opcode) {
        assert_eq!(Opcode::from(input), expected);
    }

    #[rstest]
    #[case(0x08)]
    #[case(0xFF)]
    #[should_panic(expected = "invalid opcode")]
    fn test_invalid_opcodes(#[case] invalid_opcode: u8) {
        let _ = Opcode::from(invalid_opcode);
    }

    #[rstest]
    #[case(Opcode::Literal, 0x00)]
    #[case(Opcode::Addition, 0x01)]
    #[case(Opcode::Subtract, 0x02)]
    #[case(Opcode::Multiply, 0x03)]
    #[case(Opcode::Divide, 0x04)]
    #[case(Opcode::Modulo, 0x05)]
    #[case(Opcode::Return, 0x06)]
    #[case(Opcode::Factorial, 0x07)]
    fn test_opcode_as_u8(#[case] opcode: Opcode, #[case] expected: u8) {
        assert_eq!(opcode as u8, expected);
    }
}
