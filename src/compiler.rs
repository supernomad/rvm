use nom::{
    branch::alt,
    character::complete::{char, digit1, multispace0, one_of},
    combinator::{map_res, opt, recognize},
    multi::fold_many0,
    sequence::{delimited, pair, tuple},
    IResult,
};

use crate::{opcode::Opcode, value::Value};

#[derive(Debug, PartialEq, Clone)]
enum Expr {
    Number(Value),
    BinOp(Box<Expr>, char, Box<Expr>),
    UnaryOp(char, Box<Expr>),
}

// Parse integers or floats
fn number(input: &str) -> IResult<&str, Expr> {
    alt((
        // Parse floats (must have decimal point)
        map_res(
            recognize(tuple((
                opt(char('-')),
                digit1,
                char('.'),
                digit1
            ))),
            |s: &str| s.parse::<f64>().map(|n| Expr::Number(Value::Float(n)))
        ),
        // Parse integers (with optional negative sign)
        map_res(
            recognize(pair(
                opt(char('-')),
                digit1,
            )),
            |s: &str| s.parse::<i64>().map(|n| Expr::Number(Value::Int(n)))
        ),
    ))(input)
}

// Parse expressions in parentheses
fn parens(input: &str) -> IResult<&str, Expr> {
    delimited(
        char('('),
        delimited(multispace0, expr, multispace0),
        char(')'),
    )(input)
}

// Parse a term (number or parenthesized expression)
fn term(input: &str) -> IResult<&str, Expr> {
    let (input, num) = delimited(multispace0, alt((number, parens)), multispace0)(input)?;
    
    // Look for optional unary operators
    let (input, op) = opt(alt((char('!'), char('√'))))(input)?;
    
    match op {
        Some('!') => Ok((input, Expr::UnaryOp('!', Box::new(num)))),
        Some('√') => Ok((input, Expr::UnaryOp('√', Box::new(num)))),
        _ => Ok((input, num)),
    }
}

// Parse operators by precedence level
fn op(input: &str) -> IResult<&str, char> {
    delimited(multispace0, one_of("+-*/%"), multispace0)(input)
}

// Main expression parser
fn expr(input: &str) -> IResult<&str, Expr> {
    let (input, initial) = term(input)?;

    fold_many0(
        pair(op, term),
        move || initial.clone(),
        |acc, (op, val)| Expr::BinOp(Box::new(acc), op, Box::new(val)),
    )(input)
}

pub fn compile(input: &str) -> Result<Vec<u8>, &'static str> {
    let (_, ast) = expr(input).map_err(|_| "Failed to parse expression")?;
    let mut bytecode = Vec::new();
    compile_expr(&ast, &mut bytecode);
    bytecode.push(Opcode::Return as u8);
    Ok(bytecode)
}

fn compile_expr(expr: &Expr, bytecode: &mut Vec<u8>) {
    match expr {
        Expr::Number(value) => {
            bytecode.push(Opcode::Literal as u8);
            bytecode.extend(value.to_vec());
        }
        Expr::UnaryOp('!', expr) => {
            compile_expr(expr, bytecode);
            bytecode.push(Opcode::Factorial as u8);
        }
        Expr::UnaryOp('√', expr) => {
            compile_expr(expr, bytecode);
            bytecode.push(Opcode::Sqrt as u8);
        }
        Expr::UnaryOp(_, _) => {
            panic!("Unsupported unary operator");
        }
        Expr::BinOp(left, op, right) => {
            compile_expr(left, bytecode);
            compile_expr(right, bytecode);

            let opcode = match op {
                '+' => Opcode::Addition,
                '-' => Opcode::Subtract,
                '*' => Opcode::Multiply,
                '/' => Opcode::Divide,
                '%' => Opcode::Modulo,
                _ => panic!("Unsupported operator"),
            };
            bytecode.push(opcode as u8);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vm::Vm;
    use rstest::rstest;

    fn eval(input: &str) -> Value {
        let bytecode = compile(input).unwrap();
        let mut vm = Vm::new(bytecode, 32);
        vm.run().unwrap()
    }

    #[rstest]
    #[case("1 + 2", Value::Int(3))]
    #[case("2 * (3 + 4)", Value::Int(14))]
    #[case("1 + (2 * 3)", Value::Int(7))]
    #[case("7 % 3", Value::Int(1))]
    fn test_integer_operations(#[case] input: &str, #[case] expected: Value) {
        assert_eq!(eval(input), expected);
    }

    #[rstest]
    #[case("2.5 + 1.5", Value::Float(4.0))]
    #[case("2.5 + 3", Value::Float(5.5))]
    #[case("5 + 2.5", Value::Float(7.5))]
    #[case("2 * 3.5", Value::Float(7.0))]
    #[case("3.0 * 2", Value::Float(6.0))]
    fn test_basic_float_operations(#[case] input: &str, #[case] expected: Value) {
        assert_eq!(eval(input), expected);
    }

    #[rstest]
    #[case("2.5 + (3 * 2)", Value::Float(8.5))]
    #[case("(5 - 2.5) * 3", Value::Float(7.5))]
    #[case("10 / 2.5", Value::Float(4.0))]
    #[case("2.5 * (3 + 4.5)", Value::Float(18.75))]
    fn test_complex_float_operations(#[case] input: &str, #[case] expected: Value) {
        assert_eq!(eval(input), expected);
    }

    #[rstest]
    #[case("-2.5 + 3", Value::Float(0.5))]
    #[case("5 + -2.5", Value::Float(2.5))]
    #[case("-2.5 * -2", Value::Float(5.0))]
    #[case("-10 / 2.5", Value::Float(-4.0))]
    fn test_negative_numbers(#[case] input: &str, #[case] expected: Value) {
        assert_eq!(eval(input), expected);
    }

    #[rstest]
    #[case("1 + (2 * 3.5)", Value::Float(8.0))]
    #[case("2.5 * 3 + 1", Value::Float(8.5))]
    #[case("(1 + 2) * 3.5", Value::Float(10.5))]
    #[case("10 / 2 + 1.5", Value::Float(6.5))]
    fn test_precedence(#[case] input: &str, #[case] expected: Value) {
        assert_eq!(eval(input), expected);
    }

    #[rstest]
    #[case("5!", Value::Int(120))]
    #[case("(2 + 3)!", Value::Int(120))]
    fn test_factorial_operations(#[case] input: &str, #[case] expected: Value) {
        assert_eq!(eval(input), expected);
    }

    #[test]
    #[should_panic(expected = "Unsupported unary operator")]
    fn test_invalid_unary_operator() {
        let ast = Expr::UnaryOp('~', Box::new(Expr::Number(Value::Int(5))));
        let mut bytecode = Vec::new();
        compile_expr(&ast, &mut bytecode);
    }

    #[test]
    #[should_panic(expected = "Unsupported operator")]
    fn test_invalid_binary_operator() {
        let ast = Expr::BinOp(
            Box::new(Expr::Number(Value::Int(5))),
            '^',  // Invalid operator
            Box::new(Expr::Number(Value::Int(2)))
        );
        let mut bytecode = Vec::new();
        compile_expr(&ast, &mut bytecode);
    }

    #[rstest]
    #[case("4√", Value::Float(2.0))]
    #[case("16√", Value::Float(4.0))]
    #[case("2√", Value::Float(1.4142135623730951))]
    #[case("(2 + 2)√", Value::Float(2.0))]
    fn test_sqrt_operations(#[case] input: &str, #[case] expected: Value) {
        assert_eq!(eval(input), expected);
    }

    #[rstest]
    #[case("(4 + 5)√", Value::Float(3.0))]
    #[case("2 * 16√", Value::Float(8.0))]
    #[case("(3 * 3)√", Value::Float(3.0))]
    fn test_sqrt_with_expressions(#[case] input: &str, #[case] expected: Value) {
        assert_eq!(eval(input), expected);
    }
}
