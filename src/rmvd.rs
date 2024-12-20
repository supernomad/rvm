use std::io::{self, Write};

use librvm::{compiler::compile, vm::Vm};

fn main() {
    loop {
        print!("> ");
        // Ensure the prompt is displayed before reading input
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        // Trim whitespace and check for exit condition
        let input = input.trim();
        if input.eq_ignore_ascii_case("exit") || input.eq_ignore_ascii_case("quit") {
            break;
        }

        // Skip empty lines
        if input.is_empty() {
            continue;
        }

        // Compile and run the input
        match evaluate(input) {
            Ok(result) => println!("= {}", result),
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}

fn evaluate(input: &str) -> Result<librvm::value::Value, &'static str> {
    // Attempt to compile the input
    let bytecode = match compile(input) {
        Ok(code) => code,
        Err(_) => return Err("Failed to compile expression"),
    };

    // Create VM and execute bytecode
    let mut vm = Vm::new(bytecode, 32);
    vm.run().ok_or("Failed to execute expression")
}
