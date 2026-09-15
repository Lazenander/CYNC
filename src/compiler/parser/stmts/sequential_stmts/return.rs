use crate::compiler::lexer::tokens::Keyword;
use crate::compiler::parser::error::ParseError;
use crate::compiler::parser::exprs::Expr;
use crate::compiler::parser::parser::{Parsed, ParsedBox, Parser};
use crate::compiler::parser::stmts::sequential_stmts::SEQUENTIAL_STMT_SYNC_SET;

#[derive(Debug, Clone)]
pub struct ReturnStmt(ParsedBox<Expr>);

impl Parser {
    pub fn parse_return_stmt(&mut self) -> Parsed<ReturnStmt> {
        let p_return = self.parse_keyword(Keyword::Return);
        let p_expr = Parsed::box_parsed(self.parse_expr());
        let p_sc = self.parse_semicolon();
        Parsed::lift_parsed(
            Parsed::merge_parsed_ignore_left_right(p_return, p_expr, p_sc),
            |expr| ReturnStmt(expr),
        )
    }
}
