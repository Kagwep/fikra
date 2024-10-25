use typed_arena::Arena;
use crate::fikra_entities::Token;

pub struct AstArena<'arena> {
    pub expr_arena: Arena<NodeExpr<'arena>>,
    pub stmt_arena: Arena<NodeStmt<'arena>>,
    pub bin_expr_arena: Arena<NodeBinExpr<'arena>>,
    pub paren_expr_arena: Arena<NodeExprParen<'arena>>,
}

pub struct NodeExprIntLit {
    pub int_lit: Token,
}

pub struct NodeExprIdent {
   pub ident: Token,
}

pub struct NodeExprParen<'arena> {
    pub expr: &'arena NodeExpr<'arena>,
}

pub enum ExprVar<'arena> {
    VariantOne(NodeExprIntLit),
    VariantTwo(NodeExprIdent),
    VariantThree(&'arena NodeBinExpr<'arena>),
    VariantFour(&'arena NodeExprParen<'arena>)
}

pub struct NodeStmtReturn<'arena> {
    pub expr: &'arena NodeExpr<'arena>
}

pub struct NodeStmtLet<'arena> {
    pub ident: Token,
    pub expr: &'arena NodeExpr<'arena>,
}

pub struct  NodeStmtScope<'arena>{
    pub statements: Vec<&'arena NodeStmt<'arena>>
}

pub enum StmtVariant<'arena> {
    VariantOne(NodeStmtReturn<'arena>),
    VariantTwo(NodeStmtLet<'arena>),  
    VariantThree(NodeStmtScope<'arena>)
}

pub enum NodeBinExprVariant<'arena> {
    VariantOne(NodeBinExprAdd<'arena>),
    VariantTwo(NodeBinExprMul<'arena>),
    VariantThree(NodeBinExprSub<'arena>),
    VariantFour(NodeBinExprDiv<'arena>)
}

pub struct NodeBinExprAdd<'arena> {
    pub lhs: &'arena NodeExpr<'arena>,
    pub rhs: &'arena NodeExpr<'arena>
}

pub struct NodeBinExprMul<'arena> {
    pub lhs: &'arena NodeExpr<'arena>,
    pub rhs: &'arena NodeExpr<'arena>
}


pub struct NodeBinExprSub<'arena> {
    pub lhs: &'arena NodeExpr<'arena>,
    pub rhs: &'arena NodeExpr<'arena>
}

pub struct NodeBinExprDiv<'arena> {
    pub lhs: &'arena NodeExpr<'arena>,
    pub rhs: &'arena NodeExpr<'arena>
}

pub struct NodeBinExpr<'arena> {
    pub variant: NodeBinExprVariant<'arena>
}

pub struct NodeExpr<'arena> {
    pub variant: ExprVar<'arena>,
}

pub struct NodeStmt<'arena> {
    pub variant: StmtVariant<'arena>,
}

pub struct NodeProg<'arena> {
    pub statements: Vec<&'arena NodeStmt<'arena>>
}

impl<'arena> AstArena<'arena> {
    pub fn new() -> Self {
        AstArena {
            expr_arena: Arena::new(),
            stmt_arena: Arena::new(),
            bin_expr_arena: Arena::new(),
            paren_expr_arena: Arena::new(),
        }
    }

    pub fn new_expr(&'arena self, variant: ExprVar<'arena>) -> &'arena NodeExpr<'arena> {
        self.expr_arena.alloc(NodeExpr { variant })
    }

    pub fn new_stmt(&'arena self, variant: StmtVariant<'arena>) -> &'arena NodeStmt<'arena> {
        self.stmt_arena.alloc(NodeStmt { variant })
    }

   
}

pub enum ParseError {
    StatementError,
   
}