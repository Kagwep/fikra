use crate::fikra_entities::{Precedence, Token, TokenType};
use super::node::{ AstArena, ExprVar, NodeBinExpr, NodeBinExprAdd,NodeExprParen, NodeBinExprDiv, NodeBinExprMul, NodeBinExprSub, NodeBinExprVariant, NodeExpr, NodeExprIdent, NodeExprIntLit, NodeProg, NodeStmt, NodeStmtLet, NodeStmtReturn, StmtVariant};
use crate::fikra_errors::{ParseError};

pub struct Parser<'a, 'arena> {
    tokens: &'a [Token],
    ast_arena: &'arena AstArena<'arena>,
}

impl<'a, 'arena> Parser<'a, 'arena> {
    pub fn new(tokens: &'a [Token],ast_arena: &'arena AstArena<'arena>) -> Self {
        Parser { tokens, ast_arena }
    }


    pub fn parse_prog(&mut self) -> Result<NodeProg<'arena>, ParseError> {
        let mut parse_tokens = self.tokens.iter().peekable();
        let mut statements: Vec<&'arena NodeStmt<'arena>> = Vec::new();

        while let Some(_token) = parse_tokens.peek() {
            match self.parse_stmt(&mut parse_tokens) {
                Some(statement) => {
                    statements.push(statement);
                },
                None => return Err(ParseError::InvalidStatement),
            }
        }

        Ok(NodeProg { statements })
    }


    
    fn parse_expr(&self, parse_tokens: &mut std::iter::Peekable<std::slice::Iter<'a, Token>>) -> Option<&'arena NodeExpr<'arena>> {
        self.parse_expr_with_precedence(parse_tokens, Precedence::Lowest)
    }
    
    fn parse_primary(&self, parse_tokens: &mut std::iter::Peekable<std::slice::Iter<'a, Token>>) -> Option<&'arena NodeExpr<'arena>> {
        if let Some(&token) = parse_tokens.peek() {
            let expr = match token._type {
                TokenType::Int32Lit => {
                    parse_tokens.next(); 
                    NodeExpr {
                    
                    variant: ExprVar::VariantOne(NodeExprIntLit { int_lit: token.clone() })
                    }
               },
                TokenType::Ident => {
                    parse_tokens.next(); 
                    NodeExpr {
                        variant: ExprVar::VariantTwo(NodeExprIdent { ident: token.clone() })
                    }
                },
                TokenType::OpenParen =>{
                    parse_tokens.next();
                    let inner_expr = self.parse_expr(parse_tokens)?;
                    if let Some(close_paren) = parse_tokens.next() {
                        if close_paren._type != TokenType::CloseParen {
                            return None; // Mismatched parentheses
                        }
                    }else{
                        return None; // Missing closing parenthesis
                    }
                    let paren_expr = NodeExprParen{expr: inner_expr};
                    let paren_expr_ref = self.ast_arena.paren_expr_arena.alloc(paren_expr);
                    NodeExpr {
                        variant: ExprVar::VariantFour(paren_expr_ref)
                    }
                }
                _ =>{
                    
                    return  None
                }
            };
            Some(self.ast_arena.expr_arena.alloc(expr))
        } else {
            
            None
            
        }
    }

    fn parse_expr_with_precedence(&self, parse_tokens: &mut std::iter::Peekable<std::slice::Iter<'a, Token>>, min_precedence: Precedence) -> Option<&'arena NodeExpr<'arena>> {
        let mut left: &NodeExpr<'arena> = self.parse_primary(parse_tokens)?;
        
        while let Some(&token) = parse_tokens.peek() {
            
            let token_precedence = token._type.get_precedence();

           // Break if it's not an operator or if its precedence is too low
            if token_precedence == Precedence::Lowest || token_precedence < min_precedence {
                break;
            }
            
            parse_tokens.next(); // Consume the operator token
    
            // Parse the right side with higher precedence
            let right = self.parse_expr_with_precedence(parse_tokens, token_precedence.next_higher())?;
    
            // parse a new binary expression node
            left = self.parse_binary_expr(left, token, right)?;
            
    
        }
    
        Some(left)
    }
    
    fn parse_binary_expr(&self, left: &'arena NodeExpr<'arena>, op: &Token, right: &'arena NodeExpr<'arena>) -> Option<&'arena NodeExpr<'arena>> {
        let bin_expr = match op._type {
            TokenType::Plus =>  NodeBinExpr {   
                variant: NodeBinExprVariant::VariantOne(NodeBinExprAdd { lhs: left, rhs: right })
            },
            TokenType::Star => NodeBinExpr {
                variant: NodeBinExprVariant::VariantTwo(NodeBinExprMul { lhs: left, rhs: right })
            },
            TokenType::Minus => NodeBinExpr{
                variant:NodeBinExprVariant::VariantThree(NodeBinExprSub{lhs: left, rhs: right})
            },
            TokenType::Slash => NodeBinExpr{
                variant: NodeBinExprVariant::VariantFour(NodeBinExprDiv{lhs: left, rhs: right})
            },
            // Add other binary operators here...
            _ => return Some(left), // Not a binary operator, return left as is
        };
    
        let expr = NodeExpr {
            variant: ExprVar::VariantThree(self.ast_arena.bin_expr_arena.alloc(bin_expr))
        };
        Some(self.ast_arena.expr_arena.alloc(expr))
    }

    fn parse_stmt(&self, parse_tokens: &mut std::iter::Peekable<std::slice::Iter<'a, Token>>) -> Option<&'arena NodeStmt<'arena>> {
        if let Some(&token) = parse_tokens.peek() {
            match token._type {
                TokenType::Return => {
                    parse_tokens.next(); // Consume the Return token

                    let expr = self.parse_expr(parse_tokens)?;

                    // Check for semicolon
                    if parse_tokens.next()?._type != TokenType::Semi {
                        
                        return None;
                    }

                    let return_stmt = NodeStmtReturn { expr };
                    Some(self.ast_arena.stmt_arena.alloc(NodeStmt {
                        variant: StmtVariant::VariantOne(return_stmt)
                    }))
                },
                TokenType::Let => {
                    parse_tokens.next(); // Consume the Let token

                    // Check for identifier
                    let ident_token = parse_tokens.next()?;
                    if ident_token._type != TokenType::Ident {
                        return None;
                    }

                    // Check for '=' sign
                    if parse_tokens.next()?._type != TokenType::Eq {
                        return None;
                    }


                    let expr = self.parse_expr(parse_tokens)?;

                    // Check for semicolon
                    if parse_tokens.next()?._type != TokenType::Semi {
                        return None;
                    }

                    let let_stmt = NodeStmtLet {
                        ident: ident_token.clone(),
                        expr
                    };


                    Some(self.ast_arena.stmt_arena.alloc(NodeStmt {
                        variant: StmtVariant::VariantTwo(let_stmt)
                    }))
                },
                _ => None
            }
        } else {
           
            None
        }
    }
}