use std::collections::HashMap;

use crate::parser::Expr;
use crate::toknizer::Token;

#[derive(Debug)]
pub enum Output {
    Int(i32),
    Bool(bool)
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
                        Token::Greater => Output::Bool(l>r),
                        Token::Less => Output::Bool(l<r),

                        _ => panic!("Bad operator: {:?}", op),
                    }
                }

                _ => panic!("Type mismatch"),
            }
        }

        Expr::Assignment { name, value } => {
            let result = eval(*value, variables);

            match result {
                Output::Int(value) => {
                    variables.insert(name, value);
                    Output::Int(value)
                }

                Output::Bool(_) => {
                    panic!("Cannot assign boolean to variable")
                }
            }
        }
    }
}