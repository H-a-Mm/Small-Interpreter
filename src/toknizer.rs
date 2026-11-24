// All token Types
#[derive(Debug, Clone)] // Derive so we can print
pub enum Token {

// -- Operations -- //
    Dif, // Difference
    Add, // Addition
    Div, // Division
    Mul, // Multiplication

    LParen, // Left Parenthesis
    RParen, // Right Parenthesis

// -- Data Types -- //
    
    Int(i32), // Ineteger

// -- Misc -- //

    Eof, // End Of file
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let bytes = input.as_bytes(); // Convert text to bytes for speed.
    
    let mut tokens: Vec<Token> = Vec::new(); // Create a vectored list for output tokens.
    
    let mut i = 0; // Loop index
    
    // Step Through charecters and tokenize, till we reach the end of the code.
    while i < bytes.len() {
        
        // (The match tabe uses ascii values, not string values)
        match bytes[i] {
            b'0'..=b'9' => {                 // Is Digit
                let mut value = 0i32;
                while i < bytes.len() && bytes[i].is_ascii_digit() {
                        value = value * 10 + (bytes[i] - b'0') as i32;
                    i += 1;
                }
                tokens.push(Token::Int(value));
                continue;
            },
            b'-' => tokens.push(Token::Dif), // Subtraction
            b'+' => tokens.push(Token::Add), // Addition
            b'*' => tokens.push(Token::Mul), // Multiplication
            b'/' => tokens.push(Token::Div), // Division

            b'(' => tokens.push(Token::LParen), // Left Parenthesis
            b')' => tokens.push(Token::RParen), // Right Parenthesis
            
            b' ' | b'\r' | b'\n' => {        // Specific Exception arm
                // Ignore
            }
            _ => panic!("unexpected token at position -> {:?} ", bytes[i] as char)
        }
        i += 1;
    }

    tokens.push(Token::Eof); // Denotes the end of the file for the parser.
    
    tokens.reverse(); // Lets us .pop() to see the next token (is faster).
    
    tokens
}

pub fn next(tokens: &mut Vec<Token>) -> Token {
    tokens.pop().unwrap_or(Token::Eof)
}

pub fn peek(tokens: &Vec<Token>) -> Token {
    tokens.last().cloned().unwrap_or(Token::Eof)
}
