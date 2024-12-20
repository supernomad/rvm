use crate::value::Value;

pub struct Stack {
    max: usize,
    data: Vec<Value>,
}

impl Stack {
    pub fn new(max: usize) -> Stack {
        Stack {
            max,
            data: Vec::with_capacity(max),
        }
    }

    pub fn push(&mut self, value: Value) {
        assert!(self.data.len() < self.max, "stack overflow");
        self.data.push(value);
    }

    pub fn pop(&mut self) -> Value {
        assert!(!self.data.is_empty(), "stack underflow");
        self.data.pop().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_stack() {
        let stack = Stack::new(5);
        assert_eq!(stack.data.len(), 0);
        assert_eq!(stack.max, 5);
    }

    #[test]
    fn test_push_and_pop() {
        let mut stack = Stack::new(2);
        stack.push(Value::Int(1));
        stack.push(Value::Int(2));
        assert_eq!(stack.pop(), Value::Int(2));
        assert_eq!(stack.pop(), Value::Int(1));
    }

    #[test]
    #[should_panic(expected = "stack overflow")]
    fn test_stack_overflow() {
        let mut stack = Stack::new(2);
        stack.push(Value::Int(1));
        stack.push(Value::Int(2));
        stack.push(Value::Int(3)); // Should panic
    }

    #[test]
    #[should_panic(expected = "stack underflow")]
    fn test_stack_underflow() {
        let mut stack = Stack::new(2);
        stack.pop(); // Should panic
    }

    #[test]
    fn test_multiple_operations() {
        let mut stack = Stack::new(3);
        
        // Push some values
        stack.push(Value::Int(1));
        stack.push(Value::Int(2));
        
        // Pop one and verify
        assert_eq!(stack.pop(), Value::Int(2));
        
        // Push more
        stack.push(Value::Int(3));
        stack.push(Value::Int(4));
        
        // Verify final state
        assert_eq!(stack.pop(), Value::Int(4));
        assert_eq!(stack.pop(), Value::Int(3));
        assert_eq!(stack.pop(), Value::Int(1));
    }
}
