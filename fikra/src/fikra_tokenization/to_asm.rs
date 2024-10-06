

pub mod  fikra_tokens_to_asm {

    use crate::fikra_entities::{Token,TokenType,TokenValue};

    pub fn fikra_tokens(tokens_vec: Vec<Token>) -> Result<String, &'static str> {

        let mut asm_string = String::new();
        let mut tokens_iter = tokens_vec.iter().peekable();
    
        asm_string.push_str("global _start\n_start:\n");
    
        while let Some(token) = tokens_iter.next() {
            // Check if the current token is Return
            if token._type == TokenType::Return {
                asm_string.push_str("    MOV rax, 60\n");
    
                // Peek to ensure the next token is an IntLit
                if let Some(next_token) = tokens_iter.peek() {
                    if next_token._type == TokenType::IntLit {
                        // Safely use the reference from peek()
                        if let Some(TokenValue::Integer(int_value)) = &next_token.value {
                            asm_string.push_str(&format!("    MOV rdi, {}\n", int_value));
                        } else {
                            return Err("Expected an integer value for IntLit token.");
                        }
                        
                        // Now advance the iterator
                        tokens_iter.next();
                    } else {
                        return Err("Expected an IntLit token after Return.");
                    }
                } else {
                    return Err("Missing IntLit token after Return.");
                }
    
                // Peek to ensure the next token is a Semi
                if let Some(next_token) = tokens_iter.peek() {
                    if next_token._type == TokenType::Semi {
                        // Safely use the reference from peek()
                        asm_string.push_str("    syscall\n");
    
                        // Now advance the iterator
                        tokens_iter.next();
                    } else {
                        return Err("Expected a Semi token after IntLit.");
                    }
                } else {
                    return Err("Missing Semi token after IntLit.");
                }
            } else {
                return Err("Invalid token sequence. Expected Return -> IntLit -> Semi.");
            }
        }
    
        Ok(asm_string)
    }
    
}
