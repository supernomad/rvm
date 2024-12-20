use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Rem, Sub},
};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Value {
    Int(i64),
    Float(f64),
}

impl Value {
    pub fn to_vec(&self) -> Vec<u8> {
        use Value::*;
        match self {
            Int(value) => {
                let mut bytes = vec![0];
                bytes.extend_from_slice(&value.to_be_bytes());
                bytes
            }
            Float(value) => {
                let mut bytes = vec![1];
                bytes.extend_from_slice(&value.to_be_bytes());
                bytes
            }
        }
    }

    pub fn size(&self) -> usize {
        use Value::*;
        match self {
            Int(_) => 9,
            Float(_) => 9,
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Int(value) => write!(f, "{}", value),
            Value::Float(value) => write!(f, "{}", value),
        }
    }
}

impl From<&[u8]> for Value {
    fn from(bytes: &[u8]) -> Self {
        match bytes[0] {
            0 => {
                debug_assert!(bytes.len() >= 9, "invalid byte length");
                Value::Int(i64::from_be_bytes(bytes[1..9].try_into().unwrap()))
            }
            1 => Value::Float(f64::from_be_bytes(bytes[1..9].try_into().unwrap())),
            _ => panic!("invalid value type"),
        }
    }
}

impl Add for Value {
    type Output = Value;

    fn add(self, rhs: Self) -> Self::Output {
        use Value::*;
        match (self, rhs) {
            (Int(a), Int(b)) => Int(a + b),
            (Float(a), Float(b)) => Float(a + b),
            (Int(a), Float(b)) => Float(a as f64 + b),
            (Float(a), Int(b)) => Float(a + b as f64),
        }
    }
}

impl Sub for Value {
    type Output = Value;

    fn sub(self, rhs: Self) -> Self::Output {
        use Value::*;
        match (self, rhs) {
            (Int(a), Int(b)) => Int(a - b),
            (Float(a), Float(b)) => Float(a - b),
            (Int(a), Float(b)) => Float(a as f64 - b),
            (Float(a), Int(b)) => Float(a - b as f64),
        }
    }
}

impl Mul for Value {
    type Output = Value;

    fn mul(self, rhs: Self) -> Self::Output {
        use Value::*;
        match (self, rhs) {
            (Int(a), Int(b)) => Int(a * b),
            (Float(a), Float(b)) => Float(a * b),
            (Int(a), Float(b)) => Float(a as f64 * b),
            (Float(a), Int(b)) => Float(a * b as f64),
        }
    }
}

impl Div for Value {
    type Output = Value;
    fn div(self, rhs: Self) -> Self::Output {
        use Value::*;
        match (self, rhs) {
            (Int(a), Int(b)) => Int(a / b),
            (Float(a), Float(b)) => Float(a / b),
            (Int(a), Float(b)) => Float(a as f64 / b),
            (Float(a), Int(b)) => Float(a / b as f64),
        }
    }
}

impl Rem for Value {
    type Output = Value;
    fn rem(self, rhs: Self) -> Self::Output {
        use Value::*;
        match (self, rhs) {
            (Int(a), Int(b)) => Int(a % b),
            (Float(a), Float(b)) => Float(a % b),
            (Int(a), Float(b)) => Float(a as f64 % b),
            (Float(a), Int(b)) => Float(a % b as f64),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(Value::Int(5), Value::Int(3), Value::Int(8))]
    #[case(Value::Float(5.0), Value::Float(3.0), Value::Float(8.0))]
    #[case(Value::Int(5), Value::Float(3.0), Value::Float(8.0))]
    #[case(Value::Float(5.0), Value::Int(3), Value::Float(8.0))]
    #[case(Value::Int(-5), Value::Int(3), Value::Int(-2))]
    #[case(Value::Float(-5.0), Value::Float(3.0), Value::Float(-2.0))]
    fn test_addition(#[case] a: Value, #[case] b: Value, #[case] expected: Value) {
        assert_eq!(a + b, expected);
    }

    #[rstest]
    #[case(Value::Int(5), Value::Int(3), Value::Int(2))]
    #[case(Value::Float(5.0), Value::Float(3.0), Value::Float(2.0))]
    #[case(Value::Int(5), Value::Float(3.0), Value::Float(2.0))]
    #[case(Value::Float(5.0), Value::Int(3), Value::Float(2.0))]
    #[case(Value::Int(-5), Value::Int(-3), Value::Int(-2))]
    #[case(Value::Float(-5.0), Value::Float(-3.0), Value::Float(-2.0))]
    fn test_subtraction(#[case] a: Value, #[case] b: Value, #[case] expected: Value) {
        assert_eq!(a - b, expected);
    }

    #[rstest]
    #[case(Value::Int(5), Value::Int(3), Value::Int(15))]
    #[case(Value::Float(5.0), Value::Float(3.0), Value::Float(15.0))]
    #[case(Value::Int(5), Value::Float(3.0), Value::Float(15.0))]
    #[case(Value::Float(5.0), Value::Int(3), Value::Float(15.0))]
    #[case(Value::Int(-5), Value::Int(-3), Value::Int(15))]
    #[case(Value::Float(-5.0), Value::Float(-3.0), Value::Float(15.0))]
    fn test_multiplication(#[case] a: Value, #[case] b: Value, #[case] expected: Value) {
        assert_eq!(a * b, expected);
    }

    #[rstest]
    #[case(Value::Int(6), Value::Int(2), Value::Int(3))]
    #[case(Value::Float(6.0), Value::Float(2.0), Value::Float(3.0))]
    #[case(Value::Int(6), Value::Float(2.0), Value::Float(3.0))]
    #[case(Value::Float(6.0), Value::Int(2), Value::Float(3.0))]
    #[case(Value::Int(5), Value::Int(2), Value::Int(2))]
    #[case(Value::Int(5), Value::Float(2.0), Value::Float(2.5))]
    #[case(Value::Int(-6), Value::Int(-2), Value::Int(3))]
    #[case(Value::Float(-6.0), Value::Float(-2.0), Value::Float(3.0))]
    fn test_division(#[case] a: Value, #[case] b: Value, #[case] expected: Value) {
        assert_eq!(a / b, expected);
    }

    #[rstest]
    #[case(Value::Int(7), Value::Int(3), Value::Int(1))]
    #[case(Value::Float(7.0), Value::Float(3.0), Value::Float(1.0))]
    #[case(Value::Int(7), Value::Float(3.0), Value::Float(1.0))]
    #[case(Value::Float(7.0), Value::Int(3), Value::Float(1.0))]
    #[case(Value::Int(-7), Value::Int(3), Value::Int(-1))]
    #[case(Value::Float(-7.0), Value::Float(3.0), Value::Float(-1.0))]
    fn test_remainder(#[case] a: Value, #[case] b: Value, #[case] expected: Value) {
        assert_eq!(a % b, expected);
    }

    #[test]
    fn test_value_serialization() {
        // Test Int serialization/deserialization
        let int_value = Value::Int(42);
        let bytes = int_value.to_vec();
        assert_eq!(Value::from(bytes.as_slice()), int_value);

        // Test Float serialization/deserialization
        let float_value = Value::Float(3.11);
        let bytes = float_value.to_vec();
        assert_eq!(Value::from(bytes.as_slice()), float_value);
    }

    #[test]
    fn test_display() {
        assert_eq!(Value::Int(42).to_string(), "42");
        assert_eq!(Value::Float(3.11).to_string(), "3.11");
    }

    #[test]
    #[should_panic(expected = "invalid byte length")]
    fn test_invalid_deserialization() {
        // Test with invalid byte length
        let invalid_bytes = vec![0, 1, 2];
        let _ = Value::from(invalid_bytes.as_slice());
    }

    #[test]
    #[should_panic(expected = "invalid value type")]
    fn test_invalid_value_type() {
        let invalid_bytes = vec![2, 0, 0, 0, 0, 0, 0, 0, 0]; // First byte is 2, which is invalid
        let _ = Value::from(invalid_bytes.as_slice());
    }

    #[test]
    #[should_panic(expected = "invalid byte length")]
    fn test_invalid_byte_length_short() {
        let invalid_bytes = vec![0, 1, 2]; // Too short
        let _ = Value::from(invalid_bytes.as_slice());
    }
}
