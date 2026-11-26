use crate::toknizer::{self, tokenize};
use crate::toknizer::Token;

#[derive(Debug)]
pub enum Expr {
    Int(i32),

    Variable(String),

    Macro {
        name: String,
        argument: Box<Expr>,
    },

    Assignment {
        name: String,
        value: Box<Expr>,
    },

    Binary {
        left: Box<Expr>,
        op: Token,
        right: Box<Expr>,
    },
}

pub fn parse_expr(tokens: &mut Vec<Token>, min_bp: f32) -> Expr {
    // Store Left Hand Side expr
    let mut lhs = match toknizer::next(tokens) {
        Token::Int(value) => Expr::Int(value),
        Token::LParen => {
            let expr = parse_expr(tokens, 0.0);

            match toknizer::next(tokens) {
                Token::RParen => {}
                token => panic!("Expected ')', found {:?}", token),
            }
            expr
        }

        Token::Identifier(name) => {
            match toknizer::peek(tokens) {
                Token::Equal => {
                    toknizer::next(tokens);
                
                    let value = parse_expr(tokens, 0.0);
                
                    Expr::Assignment {name, value: Box::new(value)}
                
                }, 

                Token::Macro => {
                    toknizer::next(tokens);
                    
                    match toknizer::next(tokens) {
                        Token::LParen => {}
                        token => panic!("Expected '(' after macro, found {:?}", token),
                    }
                    
                    let argument = parse_expr(tokens, 0.0);
                    
                    match toknizer::next(tokens) {
                        Token::RParen => {}
                        token => panic!("Expected ')' after macro argument, found {:?}", token),
                    }
                    
                    Expr::Macro {
                        name,
                        argument: Box::new(argument),
                    }
                }
              
              _ => panic!("Bad token")  
            }
        }

        _ => panic!("Bad token at location -> {:?}", tokens),
    };
    loop {
        // Store Operator expr
        let op = match toknizer::peek(tokens) {
            Token::Eof => return lhs,
            Token::RParen => return lhs,

            Token::Add
            | Token::Dif
            | Token::Mul
            | Token::EqualToo
            | Token::Greater
            | Token::NotEqualTo
            | Token::Less
            | Token::Div => toknizer::peek(tokens),

            _ => panic!("Bad token {:?}", tokens),
        };

        // Retrive binding power of operator
        let (l_bp, r_bp) = infix_binding_power(&op);
        if l_bp < min_bp {
            break;
        }

        toknizer::next(tokens); // consume operator

        //Store Right Hand side expr
        let rhs = parse_expr(tokens, r_bp);

        lhs = Expr::Binary {
            left: Box::new(lhs),
            op,
            right: Box::new(rhs),
        };
    }
    lhs
}

// Returns the proper binding power for the operation
fn infix_binding_power(token: &Token) -> (f32, f32) {
    match token {
        Token::Add | Token::Dif => (1.0, 1.1),      // left associative
        Token::Mul | Token::Div => (2.0, 2.1),      // left associative
        Token::Greater | Token::Less => (1.0, 1.1), // left associative
        Token::EqualToo | Token::NotEqualTo => (1.0, 1.1),
        _ => panic!("Bad operator, at token {:?}", token),
    }
}
