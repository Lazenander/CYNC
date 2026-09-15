use crate::compiler::parser::exprs::Expr;
use crate::compiler::parser::parser::{Parsed, ParsedBox, Parser};
use crate::compiler::parser::stmts::sequential_stmts::SequentialBlock;
#[derive(Debug, Clone)]
pub struct ExprStmt(ParsedBox<Expr>);

impl Parser {
    pub fn parse_expr_stmt(&mut self) -> Parsed<ExprStmt> {
        let p_expr = self.parse_expr();
        let p_sc = self.parse_semicolon();
        Parsed::lift_parsed(Parsed::merge_parsed_ignore_right(p_expr, p_sc), |expr| {
            ExprStmt(self.new_parsed_box(expr))
        })
    }
}
