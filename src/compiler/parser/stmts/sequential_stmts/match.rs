use crate::compiler::lexer::tokens::{Keyword, Operator, Paren};
use crate::compiler::parser::error::ParseError;
use crate::compiler::parser::exprs::{Expr, ExprStructure};
use crate::compiler::parser::parser::{Parsed, ParsedBox, Parser};
use crate::compiler::parser::patterns::Pattern;
use crate::compiler::parser::stmts::sequential_stmts::{SequentialBlock, SEQUENTIAL_STMT_SYNC_SET};

#[derive(Debug, Clone)]
pub struct MatchCaseBlock {
    pub patterns: Vec<ParsedBox<Pattern>>,
    pub block: ParsedBox<SequentialBlock>,
}

#[derive(Debug, Clone)]
pub struct MatchStmt {
    expr: ParsedBox<Expr>,
    cases: Vec<ParsedBox<MatchCaseBlock>>,
}

impl Parser {
    pub fn try_parse_match_case_block(&mut self) -> Option<Parsed<MatchCaseBlock>> {
        self.safe_try(|s_self| {
            let p_case = s_self.try_parse_keyword(Keyword::Case)?;
            let p_patterns = s_self.parse_addition(Parser::parse_pattern, |c_self| {
                c_self.try_parse_operator(Operator::BitOr)
            });
            let p_op = s_self.parse_operator(Operator::DoubleArrow);
            let p_block = Parsed::box_parsed(s_self.parse_sequential_block());
            Some(Parsed::merge_parsed(
                Parsed::merge_parsed_ignore_left(p_case, p_patterns),
                Parsed::merge_parsed_ignore_left(p_op, p_block),
                |patterns, block| MatchCaseBlock { patterns, block },
            ))
        })
    }

    pub fn parse_match_stmt(&mut self) -> Parsed<MatchStmt> {
        let p_match = self.parse_keyword(Keyword::Match);
        let p_expr = Parsed::box_parsed(self.parse_expr());
        let p_co = self.parse_paren(Paren::CurlyOpen);
        let p_match_cases = self.parse_star(Parser::try_parse_match_case_block);
        let p_cc = self.parse_paren(Paren::CurlyClose);
        Parsed::merge_parsed(
            Parsed::merge_parsed_ignore_left(p_match, p_expr),
            Parsed::merge_parsed_ignore_left_right(p_co, p_match_cases, p_cc),
            |expr, cases| MatchStmt { expr, cases },
        )
    }
}
