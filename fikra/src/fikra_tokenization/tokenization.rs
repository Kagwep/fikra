

pub mod tokens {

    use crate::fikra_entities::{Token,TokenType,TokenValue};
    
    pub fn tokenize(contents: String) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut chars = contents.chars().peekable();
        let mut line = 1;
        let mut column = 1;
    
        while let Some(&c) = chars.peek() {
            if c.is_whitespace() {
                chars.next();
            } else if c.is_alphabetic() {
                let mut word = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_alphanumeric() {
                        word.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                if word == "return" {
                    tokens.push(Token { _type: TokenType::Return, value: None,line: 0, column: 0});
                }else if word == "let" {
                    tokens.push(Token { _type: TokenType::Let, value: None,line: 0, column: 0 });
                } else {
                    tokens.push(Token { _type: TokenType::Ident, value: Some(TokenValue::Identifier(word)),line: 0, column: 1 });
                    
                }

            } else if c.is_numeric() {
                let mut number = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_numeric() {
                        number.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token {
                    _type: TokenType::Int32Lit,
                    value: Some(TokenValue::Int32(number.parse::<i32>().unwrap())),
                    line: 0, column: 0
                });
                
            } else if c == '(' {
                tokens.push(Token { _type: TokenType::OpenParen, value: None, line: 0, column: 0});
                chars.next();
            }else if c == ')' {
                tokens.push(Token { _type: TokenType::CloseParen, value: None,line: 0, column: 0 });
                chars.next();
            }
            else if c == '=' {
                tokens.push(Token { _type: TokenType::Eq, value: None,line: 0, column: 0 });
                chars.next();
            }
            else if c == '+' {
                tokens.push(Token { _type: TokenType::Plus, value: None,line: 0, column: 0 });
                chars.next();
            }
            else if c == '*' {
                tokens.push(Token { _type: TokenType::Star, value: None,line: 0, column: 0 });
                chars.next();
            }
            else if c == ';' {
                tokens.push(Token { _type: TokenType::Semi, value: None,line: 0, column: 0 });
                chars.next();
                
            } else {
                println!("Unexpected character: {}", c);
                chars.next();
            }
        }
    
        tokens
    }

    
}