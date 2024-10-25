use crate::fikra_entities::{TokenType, TokenValue};
use crate::fikra_parser::node::{ExprVar, NodeBinExprVariant, NodeExpr, NodeProg, NodeStmt, StmtVariant};
use crate::fikra_errors::GeneratorError;
use crate::fikra_entities::TokenValue::Int32;
use std::collections::HashMap;

pub struct Generator<'arena> {
    root: NodeProg<'arena>,
}

impl<'arena> Generator<'arena> {
    pub fn new(root: NodeProg<'arena>) -> Self {
        Generator { root }
    }

    pub fn generate_program(&self) -> Result<String, GeneratorError> {
        
        let mut asm_builder = AsmBuilder::new();
        let mut stack = Stack::new();
        
        for stmt in &self.root.statements {
            self.generate_statement(stmt, &mut asm_builder,&mut stack)?;
        }
        asm_builder.add_instruction("mov rax, 60");
        asm_builder.add_instruction("mov rdi, 0");
        asm_builder.add_instruction("syscall");
        Ok(asm_builder.build())
    }

    fn generate_statement(&self, node_stmt: &NodeStmt, asm: &mut AsmBuilder,stack: &mut Stack) -> Result<(), GeneratorError> {

        match &node_stmt.variant {
            StmtVariant::VariantOne(stmt) => {
                self.generate_expression(&stmt.expr, asm,stack)?;
                asm.add_instruction("mov rax, 60");
                asm.add_instruction(&stack.pop("rdi"));
                asm.add_instruction("syscall");
                Ok(())
            },
            StmtVariant::VariantTwo(stmt) => {
                match (&stmt.ident._type, &stmt.ident.value) {
                    (TokenType::Ident, Some(TokenValue::Identifier(ident_str))) => {
                        if stack.map_variables.contains_key(ident_str) {
                           
                            Err(GeneratorError::InvalidStatement)
                        } else {
                            
                            stack.map_variables.insert(ident_str.to_string(), Var {stack_loc: stack.current_size()});
                            self.generate_expression(&stmt.expr, asm, stack)?;
                            
                            Ok(())
                        }
                    },
                    (TokenType::Ident, _) =>  { Err(GeneratorError::InvalidStatement)},
                    _ =>  Err(GeneratorError::InvalidStatement),
                }
               
            },
            StmtVariant::VariantThree(stmt) =>{

                  stack.begin_scope();
                   for stm in stmt.statements.iter(){

                   }
                   stack.end_scope(asm);

                Ok(())
            },
            _ => Err(GeneratorError::InvalidStatement),
        }
    }

    fn generate_expression(&self, node_expr: &NodeExpr, asm: &mut AsmBuilder,stack: &mut Stack) -> Result<(), GeneratorError> {
        match &node_expr.variant {
            ExprVar::VariantOne(node_expres_int_lit) => {
                if let Some(Int32(n)) = &node_expres_int_lit.int_lit.value {
                    asm.add_instruction(&format!("mov rax, {}", n));
                    asm.add_instruction(&stack.push("rax"));
                    Ok(())
                } else {
                    Err(GeneratorError::InvalidIntegerValue)
                }
            },
            ExprVar::VariantTwo(n) => {

                if let (TokenType::Ident, Some(TokenValue::Identifier(ident_str))) = (&n.ident._type, &n.ident.value){
                    self.generate_identifier_expression(ident_str, asm, stack)
                }else{
                    Err(GeneratorError::InvalidStatement)
                }

            },
            ExprVar::VariantThree(n) => self.generate_binary_expression(&n.variant, asm, stack),
            ExprVar::VariantFour(node_expr_paren) => {
                // For parenthesized expressions, we simply generate code for the inner expression
                self.generate_expression(node_expr_paren.expr, asm, stack)
            },
            _ => Err(GeneratorError::InvalidExpression),
        }
    }

    fn generate_identifier_expression(&self, ident_str: &str, asm: &mut AsmBuilder, stack: &mut Stack) -> Result<(), GeneratorError> {
        if let Some(stack_loc_var) = stack.map_variables.get(ident_str) {
            let offset = ((stack.current_size() - stack_loc_var.stack_loc) - 1) * 8;
            let instruction = format!("QWORD [rsp + {}]", offset);
            asm.add_instruction(&stack.push(&instruction));
            Ok(())
        } else {
            Err(GeneratorError::UndefinedVariable(ident_str.to_string()))
        }
    }

    fn generate_binary_expression(&self, variant: &NodeBinExprVariant, asm: &mut AsmBuilder, stack: &mut Stack) -> Result<(), GeneratorError> {
        match variant {
            NodeBinExprVariant::VariantOne(node_bin_expr_add) => {
                self.generate_expression(node_bin_expr_add.lhs, asm, stack)?;
                self.generate_expression(node_bin_expr_add.rhs, asm, stack)?;
    
                asm.add_instruction(&stack.pop("rax"));
                asm.add_instruction(&stack.pop("rbx"));
                asm.add_instruction("add rax, rbx");
                asm.add_instruction(&stack.push("rax"));
    
                Ok(())
            },
            NodeBinExprVariant::VariantTwo(node_bin_expr_mul) => {
                self.generate_expression(node_bin_expr_mul.lhs, asm, stack)?;
                self.generate_expression(node_bin_expr_mul.rhs, asm, stack)?;
    
                asm.add_instruction(&stack.pop("rax"));
                asm.add_instruction(&stack.pop("rbx"));
                asm.add_instruction("mul rbx");
                asm.add_instruction(&stack.push("rax"));

                Ok(())
            },
            NodeBinExprVariant::VariantThree(node_bin_expr_sub) => {
                self.generate_expression(node_bin_expr_sub.lhs, asm, stack)?;
                self.generate_expression(node_bin_expr_sub.rhs, asm, stack)?;
                
                asm.add_instruction(&stack.pop("rbx")); // Pop the right operand into rbx
                asm.add_instruction(&stack.pop("rax")); // Pop the left operand into rax
                asm.add_instruction("sub rax, rbx");    // Subtract rbx from rax
                asm.add_instruction(&stack.push("rax")); // Push the result back onto the stack
                Ok(())
            },
            NodeBinExprVariant::VariantFour(node_bin_expr_div) => {
                self.generate_expression(node_bin_expr_div.lhs, asm, stack)?;
                self.generate_expression(node_bin_expr_div.rhs, asm, stack)?;
    
                asm.add_instruction(&stack.pop("rbx"));
                asm.add_instruction(&stack.pop("rax"));
                asm.add_instruction("div rbx");
                asm.add_instruction(&stack.push("rax"));

                Ok(())
            },
            _ => Err(GeneratorError::InvalidExpression)
        }


    }

}

struct AsmBuilder {
    asm_string: String,
}

impl AsmBuilder {
    fn new() -> Self {
        let mut builder = AsmBuilder { asm_string: String::new() };
        builder.add_directive("global _start");
        builder.add_directive("_start:");
        builder
    }

    fn add_directive(&mut self, directive: &str) {
        self.asm_string.push_str(directive);
        self.asm_string.push('\n');
    }

    fn add_instruction(&mut self, instruction: &str) {
        self.asm_string.push_str("    ");
        self.asm_string.push_str(instruction);
        self.asm_string.push('\n');
    }

    fn build(self) -> String {
        self.asm_string
    }
}

struct Var{
    stack_loc: u128,
}
struct Stack {
    index: u128,
    map_variables: HashMap<String, Var>,
    scope_boundaries: Vec<u128>,
}

impl Stack {
    fn new() -> Self {
        Stack { index: 0, map_variables: HashMap::new() ,scope_boundaries: Vec::new(),}
    }

    fn push(&mut self, reg: &str) -> String {
        self.index = self.index.saturating_add(1);
        format!("push {}", reg)
    }

    fn pop(&mut self, reg: &str) -> String {
        self.index = self.index.saturating_sub(1);
        format!("pop {}", reg)
    }

    fn current_size(&self) -> u128 {
        self.index
    }

    fn begin_scope(&mut self){
        self.scope_boundaries.push(self.index);
    }

    fn end_scope(&mut self, asm: &mut AsmBuilder) {
        if let Some(scope_start) = self.scope_boundaries.pop() {
            let variables_to_pop = self.index - scope_start;
            
            // Remove variables from current scope
            self.map_variables.retain(|_, var| var.stack_loc < scope_start);
            
            // Generate pop instruction if needed
            if variables_to_pop > 0 {
                asm.add_instruction(&format!("add rsp, {}", variables_to_pop * 8));
            }
            
            self.index = scope_start;
        }
    }

}