
#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Keywords
    Return,     // 'return' keyword for returning from a function
    Let,        // 'let' keyword for variable declaration
    If,         // 'if' keyword for conditional statements
    Else,       // 'else' keyword for alternative in conditional statements
    While,      // 'while' keyword for while loops
    For,        // 'for' keyword for for loops
    
    // Literals
    IntLit,     // Integer literal of unspecified size
    Int8Lit,    // 8-bit signed integer literal
    Int16Lit,   // 16-bit signed integer literal
    Int32Lit,   // 32-bit signed integer literal
    Int64Lit,   // 64-bit signed integer literal
    UInt8Lit,   // 8-bit unsigned integer literal
    UInt16Lit,  // 16-bit unsigned integer literal
    UInt32Lit,  // 32-bit unsigned integer literal
    UInt64Lit,  // 64-bit unsigned integer literal
    FloatLit,   // Floating-point literal
    StringLit,  // String literal
    BoolLit,    // Boolean literal (true or false)

    // Identifiers
    Ident,      // Identifier (variable names, function names, etc.)
    
    // Operators
    Plus,       // Addition operator '+'
    Minus,      // Subtraction operator '-'
    Star,       // Multiplication operator '*'
    Slash,      // Division operator '/'
    Percent,    // Modulo operator '%'
    Eq,         // Assignment operator '='
    EqEq,       // Equality comparison operator '=='
    NotEq,      // Inequality comparison operator '!='
    Lt,         // Less than operator '<'
    LtEq,       // Less than or equal to operator '<='
    Gt,         // Greater than operator '>'
    GtEq,       // Greater than or equal to operator '>='
    And,        // Logical AND operator '&&'
    Or,         // Logical OR operator '||'
    Not,        // Logical NOT operator '!'
    Caret,      // Exponentiation operator '^'
    
    // Delimiters
    Semi,       // Semicolon ';' for statement termination
    Comma,      // Comma ',' for separating items in lists
    Dot,        // Dot '.' for member access
    OpenParen,  // Opening parenthesis '('
    CloseParen, // Closing parenthesis ')'
    OpenBrace,  // Opening brace '{' for blocks
    CloseBrace, // Closing brace '}' for blocks
    OpenBracket, // Opening square bracket '[' for arrays
    CloseBracket, // Closing square bracket ']' for arrays
    
    // Special
    EOF,        // End of file marker
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub _type: TokenType,
    pub value: Option<TokenValue>,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenValue {
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    UInt8(u8),
    UInt16(u16),
    UInt32(u32),
    UInt64(u64),
    Float32(f32),
    Float64(f64),
    Boolean(bool),
    Identifier(String),
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Precedence {
    Lowest,        // Lowest precedence
    Assignment,    // Assignment operators (=)
    LogicalOr,     // Logical OR (||)
    LogicalAnd,    // Logical AND (&&)
    Equality,      // Equality operators (==, !=)
    Comparison,    // Comparison operators (<, <=, >, >=)
    Term,          // Addition and subtraction (+, -)
    Factor,        // Multiplication, division, remainder (*, /, %)
    Exponent,      // Exponentiation (^)
    Unary,         // Unary operators (!, -)
}


impl Token {
    pub fn new(_type: TokenType, value: Option<TokenValue>, line: usize, column: usize) -> Self {
        Token { _type, value, line, column }
    }
}


impl  TokenType {
    pub fn get_precedence(&self) -> Precedence {
        match self {
            TokenType::Eq => Precedence::Assignment,
            TokenType::Or => Precedence::LogicalOr,
            TokenType::And => Precedence::LogicalAnd,
            TokenType::EqEq | TokenType::NotEq => Precedence::Equality,
            TokenType::Lt | TokenType::LtEq | TokenType::Gt | TokenType::GtEq => Precedence::Comparison,
            TokenType::Plus | TokenType::Minus => Precedence::Term,
            TokenType::Star | TokenType::Slash | TokenType::Percent => Precedence::Factor,
            TokenType::Caret => Precedence::Exponent,
            TokenType::Not => Precedence::Unary,
            _ => Precedence::Lowest,
        }
    }

    pub fn evaluation_order(&self) -> u8 {
        match self {
            TokenType::Minus => 1,
            TokenType::Plus => 2,
            TokenType::Star => 3,
            TokenType::Slash => 4,
            TokenType::Percent => 5,
            TokenType::Caret => 6,
            _ => 0,
            
        }
    }

    fn is_binary_op(&self) -> bool {
        matches!(self, TokenType::Plus | TokenType::Star /* Add other binary operators */)
    }
}

impl Precedence {
    // A method to get the numeric precedence value
    pub fn value(&self) -> i32 {
        match self {
            Precedence::Lowest => 0,
            Precedence::Assignment => 1,
            Precedence::LogicalOr => 2,
            Precedence::LogicalAnd => 3,
            Precedence::Equality => 4,
            Precedence::Comparison => 5,
            Precedence::Term => 6,
            Precedence::Factor => 7,
            Precedence::Exponent => 8,
            Precedence::Unary => 9,
        }
    }

    pub fn next_higher(&self) -> Precedence {
        match self {
            Precedence::Lowest => Precedence::Assignment,
            Precedence::Assignment => Precedence::LogicalOr,
            Precedence::LogicalOr => Precedence::LogicalAnd,
            Precedence::LogicalAnd => Precedence::Equality,
            Precedence::Equality => Precedence::Comparison,
            Precedence::Comparison => Precedence::Term,
            Precedence::Term => Precedence::Factor,
            Precedence::Factor => Precedence::Exponent,
            Precedence::Exponent => Precedence::Unary,
            Precedence::Unary => Precedence::Unary, // Highest precedence
        }
    }


}