use crate::compiler::lexer::tokens::Keyword;
use crate::compiler::parser::error::ParseError;
use crate::compiler::parser::exprs::{Expr, ExprStructure, EXPR_SYNC_SET};
use crate::compiler::parser::parser::{Parsed, ParsedBox, Parser};

impl Parser {
    pub fn parse_if_then_else_expr(&mut self) -> Parsed<Expr> {
        self.force_parse_with_default(
            self.default_expr(),
            Parser::try_parse_if_then_else_expr,
            ParseError::expected_expression_not_found,
            EXPR_SYNC_SET.into(),
        )
    }

    pub fn try_parse_if_then_else_expr(&mut self) -> Option<Parsed<Expr>> {
        self.safe_try(|s_self| {
            let p_if = s_self.try_parse_keyword(Keyword::If)?;
            let p_cond = Parsed::box_parsed(s_self.parse_expr());
            let p_then = s_self.parse_keyword(Keyword::Then);
            let p_expr1 = Parsed::box_parsed(s_self.parse_expr());
            let p_else = s_self.parse_keyword(Keyword::Else);
            let p_expr2 = Parsed::box_parsed(s_self.parse_expr());
            Some(Parsed::merge_parsed_3(
                Parsed::merge_parsed_ignore_left(p_if, p_cond),
                Parsed::merge_parsed_ignore_left(p_then, p_expr1),
                Parsed::merge_parsed_ignore_left(p_else, p_expr2),
                |condition, then_expr, else_expr| {
                    Expr::new(ExprStructure::IfThenElse {
                        condition,
                        then_expr,
                        else_expr,
                    })
                },
            ))
        })
    }
}
