// The Main.rs script is just to be used as a staging module, any external function or actions must be handeled
// in an external script and then imported here. This prevents a messy codebase.

// crates
use std::io::{self, Write};
use std::collections::HashMap;

use crate::interpreter::Output;

// modules
mod interpreter;
mod parser;
mod toknizer;

fn main() {

    // Contruction variables
    let mut variables = HashMap::new();

    loop {
        print!(">> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();

        // Take user input.
        io::stdin().read_line(&mut input).unwrap();

        if input.trim() == "exit()" {
            std::process::exit(0);
        } else if input.trim() == "exit" {
            println!("exit keyword must be a function -> 'exit()'");
            continue;
        }

        // Retrive tokens from the tokizer
        let mut tokens = toknizer::tokenize(&input);

        // Parse the given expression
        let parsed = parser::parse_expr(&mut tokens, 1.0);

        // Evaluate parsed expression
        let result = interpreter::eval(parsed, &mut variables);

        // Decode output
        match result {
            Output::Int(n) => println!("{}", n),
            Output::Bool(n) => println!("{}", n),

            _ => {}
        }
    }
}