use std::collections::HashMap;

use crate::parser::Expr;
use crate::toknizer::Token;

pub fn eval(expr: Expr, variables: &mut HashMap<String, i32>) -> i32 {
    match expr {
        Expr::Int(n) => n,

        Expr::Variable(name) => {
            *variables
                .get(&name)
                .unwrap_or_else(|| panic!("Undefined variable '{}'", name))
        }

        Expr::Binary { left, op, right } => {
            let lhs = eval(*left, variables);
            let rhs = eval(*right, variables);

            match op {
                Token::Add => lhs + rhs,
                Token::Dif => lhs - rhs,
                Token::Mul => lhs * rhs,
                Token::Div => lhs / rhs,

                _ => panic!("Bad operator: {:?}", op),
            }
        }

        Expr::Assignment { name, value } => {
            let result = eval(*value, variables);

            variables.insert(name, result);

            result
        }
    }
}