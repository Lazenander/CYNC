use crate::compiler::lexer::tokens::{Keyword, Operator, TokenGut};
use crate::compiler::parser::error::ParseError;
use crate::compiler::parser::exprs::{Expr, ExprStructure};
use crate::compiler::parser::parser::{Parsed, ParsedBox, Parser};
use crate::compiler::parser::patterns::Pattern;
use crate::compiler::parser::stmts::sequential_stmts::{
    MatchStmt, SequentialBlock, SEQUENTIAL_STMT_SYNC_SET,
};

#[derive(Debug, Clone)]
pub struct LetElement {
    pub pattern: ParsedBox<Pattern>,
    pub expr: ParsedBox<Expr>,
}

#[derive(Debug, Clone)]
pub struct LetStmt {
    pub elements: Vec<ParsedBox<LetElement>>,
}

impl Parser {
    pub fn parse_let_stmt(&mut self) -> Parsed<LetStmt> {
        let p_let = self.parse_keyword(Keyword::Let);
        let p_addition = self.parse_letelements();
        let p_sc = self.parse_semicolon();
        Parsed::merge_parsed_ignore_left_right(
            p_let,
            Parsed::lift_parsed(p_addition, |elements| LetStmt { elements }),
            p_sc,
        )
    }

    pub fn parse_letelements(&mut self) -> Parsed<Vec<ParsedBox<LetElement>>> {
        self.parse_addition(Parser::parse_letelement, |c_self| {
            c_self.try_parse_keyword(Keyword::And)
        })
    }

    pub fn parse_letelement(&mut self) -> Parsed<LetElement> {
        let pattern_token = self.parse_pattern();
        let assign_op_token = self.parse_operator(Operator::Assign);
        let assign_expr = self.parse_expr();
        Parsed::merge_parsed(
            Parsed::merge_parsed_ignore_right(pattern_token, assign_op_token),
            assign_expr,
            |pattern, expr| LetElement {
                pattern: self.new_parsed_box(pattern),
                expr: self.new_parsed_box(expr),
            },
        )
    }
}
