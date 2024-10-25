pub mod tokens {
    use crate::fikra_entities::{Token, TokenType, TokenValue};

    pub fn tokenize(contents: String) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut chars = contents.char_indices().peekable();
        let mut line = 1;
        let mut column = 1;

        while let Some((i, c)) = chars.next() {
            let token = match c {
                c if c.is_whitespace() => {
                    if c == '\n' {
                        line += 1;
                        column = 1;
                    } else {
                        column += 1;
                    }
                    continue;
                }
                c if c.is_alphabetic() => {
                    let word_start = i;
                    let start_column = column;
                    while let Some(&(_, next_c)) = chars.peek() {
                        if !next_c.is_alphanumeric() {
                            break;
                        }
                        chars.next();
                        column += 1;
                    }
                    let word = &contents[word_start..chars.peek().map_or(contents.len(), |&(i, _)| i)];
                    match word {
                        "return" => Token { _type: TokenType::Return, value: None, line, column: start_column },
                        "let" => Token { _type: TokenType::Let, value: None, line, column: start_column },
                        _ => Token { _type: TokenType::Ident, value: Some(TokenValue::Identifier(word.to_string())), line, column: start_column },
                    }
                }
                c if c.is_numeric() => {
                    let num_start = i;
                    let start_column = column;
                    while let Some(&(_, next_c)) = chars.peek() {
                        if !next_c.is_numeric() {
                            break;
                        }
                        chars.next();
                        column += 1;
                    }
                    let number = &contents[num_start..chars.peek().map_or(contents.len(), |&(i, _)| i)];
                    Token {
                        _type: TokenType::Int32Lit,
                        value: Some(TokenValue::Int32(number.parse().unwrap())),
                        line,
                        column: start_column,
                    }
                }
                '(' => Token { _type: TokenType::OpenParen, value: None, line, column },
                ')' => Token { _type: TokenType::CloseParen, value: None, line, column },
                '=' => Token { _type: TokenType::Eq, value: None, line, column },
                '+' => Token { _type: TokenType::Plus, value: None, line, column },
                '*' => Token { _type: TokenType::Star, value: None, line, column },
                '/' => Token { _type: TokenType::Slash, value: None, line, column },
                '-' => Token { _type: TokenType::Minus, value: None, line, column },
                ';' => Token { _type: TokenType::Semi, value: None, line, column },
                '{' => Token { _type: TokenType::OpenBrace, value: None, line, column },
                '}' => Token { _type: TokenType::CloseBrace, value: None, line, column },
                _ => {
                    eprintln!("Unexpected character: {} at line {}, column {}", c, line, column);
                    column += 1;
                    continue;
                }
            };
            tokens.push(token);
            column += 1;
        }

        tokens
    }
}