use crate::parser::Expr;
use crate::toknizer::Token;

pub fn eval(expr: Expr) -> i32 {
    match expr {
        Expr::Int(n) => n,

        Expr::Binary { left, op, right } => {
            let lhs = eval(*left);
            let rhs = eval(*right);

            match op {
                Token::Add => lhs + rhs,
                Token::Dif => lhs - rhs,
                Token::Mul => lhs * rhs,
                Token::Div => lhs / rhs,

                _ => panic!("Bad operator: {:?}", op),
            }
        }
    }
}