use crate::{opcode::Opcode, stack::Stack, value::Value};

pub struct Vm {
    stack: Stack,
    bytecode: Vec<u8>,
}

impl Vm {
    pub fn new<C>(bytecode: C, stack_size: usize) -> Vm
    where
        C: Into<Vec<u8>>,
    {
        Vm {
            stack: Stack::new(stack_size),
            bytecode: bytecode.into(),
        }
    }

    #[inline]
    fn execute_binary_op<F>(&mut self, op: F)
    where
        F: FnOnce(Value, Value) -> Value,
    {
        let rhs = self.stack.pop();
        let lhs = self.stack.pop();
        self.stack.push(op(lhs, rhs));
    }

    pub fn run(&mut self) -> Option<Value> {
        let mut position = 0;
        while position < self.bytecode.len() {
            let opcode = self.bytecode[position];
            position += 1;

            match Opcode::from(opcode) {
                Opcode::Literal => {
                    let value = Value::from(&self.bytecode[position..]);
                    position += value.size();
                    self.stack.push(value);
                }
                Opcode::Addition => self.execute_binary_op(|lhs, rhs| lhs + rhs),
                Opcode::Subtract => self.execute_binary_op(|lhs, rhs| lhs - rhs),
                Opcode::Multiply => self.execute_binary_op(|lhs, rhs| lhs * rhs),
                Opcode::Divide => self.execute_binary_op(|lhs, rhs| lhs / rhs),
                Opcode::Modulo => self.execute_binary_op(|lhs, rhs| lhs % rhs),
                Opcode::Factorial => {
                    let value = self.stack.pop();
                    match value {
                        Value::Int(value) => {
                            self.stack.push(Value::Int((1..=value).product()));
                        }
                        _ => panic!("invalid value type"),
                    }
                }
                Opcode::Return => {
                    return Some(self.stack.pop());
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    fn create_binary_op_bytecode(lhs: i64, rhs: i64, op: Opcode) -> Vec<u8> {
        let mut bytecode = vec![Opcode::Literal as u8];
        bytecode.extend(Value::Int(lhs).to_vec());
        bytecode.push(Opcode::Literal as u8);
        bytecode.extend(Value::Int(rhs).to_vec());
        bytecode.push(op as u8);
        bytecode.push(Opcode::Return as u8);
        bytecode
    }

    fn create_unary_op_bytecode(value: i64, op: Opcode) -> Vec<u8> {
        let mut bytecode = vec![Opcode::Literal as u8];
        bytecode.extend(Value::Int(value).to_vec());
        bytecode.push(op as u8);
        bytecode.push(Opcode::Return as u8);
        bytecode
    }

    #[rstest]
    #[case(1, 2, 3)]
    #[case(10, 5, 15)]
    #[case(-1, 1, 0)]
    #[case(100, 200, 300)]
    fn test_addition(#[case] lhs: i64, #[case] rhs: i64, #[case] expected: i64) {
        let bytecode = create_binary_op_bytecode(lhs, rhs, Opcode::Addition);
        let mut vm = Vm::new(bytecode, 10);
        let ret = vm.run().unwrap();
        assert_eq!(ret, Value::Int(expected));
    }

    #[rstest]
    #[case(1, 2, -1)]
    #[case(10, 3, 7)]
    #[case(0, 5, -5)]
    #[case(100, 50, 50)]
    fn test_subtraction(#[case] lhs: i64, #[case] rhs: i64, #[case] expected: i64) {
        let bytecode = create_binary_op_bytecode(lhs, rhs, Opcode::Subtract);
        let mut vm = Vm::new(bytecode, 10);
        let ret = vm.run().unwrap();
        assert_eq!(ret, Value::Int(expected));
    }

    #[rstest]
    #[case(2, 3, 6)]
    #[case(5, 4, 20)]
    #[case(-2, 3, -6)]
    #[case(10, 10, 100)]
    fn test_multiplication(#[case] lhs: i64, #[case] rhs: i64, #[case] expected: i64) {
        let bytecode = create_binary_op_bytecode(lhs, rhs, Opcode::Multiply);
        let mut vm = Vm::new(bytecode, 10);
        let ret = vm.run().unwrap();
        assert_eq!(ret, Value::Int(expected));
    }

    #[rstest]
    #[case(6, 2, 3)]
    #[case(15, 3, 5)]
    #[case(100, 10, 10)]
    #[case(-12, 3, -4)]
    fn test_division(#[case] lhs: i64, #[case] rhs: i64, #[case] expected: i64) {
        let bytecode = create_binary_op_bytecode(lhs, rhs, Opcode::Divide);
        let mut vm = Vm::new(bytecode, 10);
        let ret = vm.run().unwrap();
        assert_eq!(ret, Value::Int(expected));
    }

    #[rstest]
    #[case(7, 3, 1)]
    #[case(10, 3, 1)]
    #[case(15, 4, 3)]
    #[case(100, 30, 10)]
    fn test_modulo(#[case] lhs: i64, #[case] rhs: i64, #[case] expected: i64) {
        let bytecode = create_binary_op_bytecode(lhs, rhs, Opcode::Modulo);
        let mut vm = Vm::new(bytecode, 10);
        let ret = vm.run().unwrap();
        assert_eq!(ret, Value::Int(expected));
    }

    #[rstest]
    #[case(5, 120)]  // 5! = 5 * 4 * 3 * 2 * 1 = 120
    #[case(3, 6)]    // 3! = 3 * 2 * 1 = 6
    #[case(4, 24)]   // 4! = 4 * 3 * 2 * 1 = 24
    #[case(0, 1)]    // 0! = 1
    fn test_factorial(#[case] value: i64, #[case] expected: i64) {
        let bytecode = create_unary_op_bytecode(value, Opcode::Factorial);
        let mut vm = Vm::new(bytecode, 10);
        let ret = vm.run().unwrap();
        assert_eq!(ret, Value::Int(expected));
    }
}
