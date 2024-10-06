
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken(String),
    UnexpectedEndOfInput,
    InvalidStatement,
    InvalidExpression,
    // Add more specific error types as needed
}


// Custom error type
#[derive(Debug)]
pub enum GeneratorError {
    InvalidReturnValue,
    InvalidStatement,
    InvalidExpression,
    InvalidIntegerValue,
    IdentifierAlreadyUsed,
    UndefinedVariable(String),
}


impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::UnexpectedToken(token) => write!(f, "Unexpected token: {}", token),
            ParseError::UnexpectedEndOfInput => write!(f, "Unexpected end of input"),
            ParseError::InvalidStatement => write!(f, "Invalid statement"),
            ParseError::InvalidExpression => write!(f, "Invalid expression"),
            // Add more error messages as needed
        }
    }
}

impl std::error::Error for ParseError {}



impl fmt::Display for GeneratorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GeneratorError::InvalidReturnValue => write!(f, "Invalid or missing return value"),
            GeneratorError::InvalidStatement => write!(f, "Invalid statement or missing statement"),
            GeneratorError::InvalidExpression => write!(f, "Invalid expression or missing expression"),
            GeneratorError::InvalidIntegerValue => write!(f, "Invalid integer or missing integer"),
            GeneratorError::IdentifierAlreadyUsed => write!(f, "Identifier already used"),
            GeneratorError::UndefinedVariable(e) => write!(f, "Variable {} is undefined", e),
        }
    }
}

impl Error for GeneratorError {}

