use std::collections::HashMap;
use std::fmt;

use crate::parser::Expr;
use crate::toknizer::Token;

#[derive(Debug)]
pub enum Output {
    Int(i32),
    String(String),
    Bool(bool),
    None(),
}

impl fmt::Display for Output {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Output::Int(v) => write!(f, "{v}"),
            Output::String(v) => write!(f, "{v}"),
            Output::Bool(v) => write!(f, "{v}"),
            Output::None() => write!(f, ""),
        }
    }
}

pub fn eval(expr: Expr, variables: &mut HashMap<String, i32>) -> Output {
    match expr {
        Expr::Int(n) => Output::Int(n),

        Expr::Variable(name) => {
            Output::Int(
                *variables
                    .get(&name)
                    .unwrap_or_else(|| panic!("Undefined variable '{}'", name))
            )
        }

        Expr::Binary { left, op, right } => {
            let lhs = eval(*left, variables);
            let rhs = eval(*right, variables);

            match (lhs, rhs) {
                (Output::Int(l), Output::Int(r)) => {
                    match op {
                        Token::Add => Output::Int(l + r),
                        Token::Dif => Output::Int(l - r),
                        Token::Mul => Output::Int(l * r),
                        Token::Div => Output::Int(l / r),
                        Token::EqualToo => Output::Bool(l == r),
                        Token::Greater => Output::Bool(l > r),
                        Token::Less => Output::Bool(l < r),
                        Token::NotEqualTo => Output::Bool(l != r),

                        _ => panic!("Bad operator: {:?}", op),
                    }
                }

                _ => panic!("Type mismatch"),
            }
        }

        Expr::Assignment { name, value } => {

            // Edge cases
            match value.as_ref() {
                Expr::Macro { .. } => {
                    panic!("cannot assign a macro result to a variable");
                }
                _ => {}
            }
            
            let result = eval(*value, variables);

            match result {
                Output::Int(value) => {
                    variables.insert(name, value);
                    Output::Int(value)
                }

                Output::Bool(_) => {
                    panic!("Cannot assign boolean to variable")
                }
                _ => Output::None()
            }
        }

        // Hard coded interpreter macros
        Expr::Macro { name, argument } => {
            let arg = eval(*argument, variables);
        
            match name.as_str() {
                "print" => {
                    print!("{} \n", arg);
                    Output::None()
                }
        
                "println" => {
                    println!("{} \n", arg);
                    Output::None()
                }
        
                "panic" => {
                    panic!("{} \n", arg);
                }
        
                _ => panic!("Unknown macro: {}", name),
            }
        }
    }
}