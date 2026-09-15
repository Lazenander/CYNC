use crate::compiler::lexer::tokens::Keyword;
use crate::compiler::parser::error::ParseError;
use crate::compiler::parser::exprs::Expr;
use crate::compiler::parser::parser::{Parsed, ParsedBox, Parser};
use crate::compiler::parser::stmts::sequential_stmts::{SequentialBlock, SEQUENTIAL_STMT_SYNC_SET};

#[derive(Debug, Clone)]
pub struct WhileStmt {
    condition: ParsedBox<Expr>,
    block: ParsedBox<SequentialBlock>,
}

#[derive(Debug, Clone)]
pub struct DoWhileStmt {
    block: ParsedBox<SequentialBlock>,
    condition: ParsedBox<Expr>,
}

#[derive(Debug, Clone)]
pub struct ContinueStmt();
#[derive(Debug, Clone)]
pub struct BreakStmt();

impl Parser {
    pub fn parse_continue_stmt(&mut self) -> Parsed<ContinueStmt> {
        let p_continue = self.parse_keyword(Keyword::Continue);
        Parsed::lift_parsed(p_continue, |_| ContinueStmt())
    }

    pub fn parse_break_stmt(&mut self) -> Parsed<BreakStmt> {
        let p_break = self.parse_keyword(Keyword::Break);
        Parsed::lift_parsed(p_break, |_| BreakStmt())
    }

    pub fn parse_while_stmt(&mut self) -> Parsed<WhileStmt> {
        let p_while = self.parse_keyword(Keyword::While);
        let p_cond = Parsed::box_parsed(self.parse_expr());
        let p_block = Parsed::box_parsed(self.parse_sequential_block());
        Parsed::merge_parsed(
            Parsed::merge_parsed_ignore_left(p_while, p_cond),
            p_block,
            |condition, block| WhileStmt { condition, block },
        )
    }

    pub fn parse_do_while_stmt(&mut self) -> Parsed<DoWhileStmt> {
        let p_do = self.parse_keyword(Keyword::Do);
        let p_block = Parsed::box_parsed(self.parse_sequential_block());
        let p_while = self.parse_keyword(Keyword::While);
        let p_cond = Parsed::box_parsed(self.parse_expr());
        let p_sc = self.parse_semicolon();
        Parsed::merge_parsed(
            Parsed::merge_parsed_ignore_left(p_do, p_block),
            Parsed::merge_parsed_ignore_left_right(p_while, p_cond, p_sc),
            |block, condition| DoWhileStmt { block, condition },
        )
    }
}
