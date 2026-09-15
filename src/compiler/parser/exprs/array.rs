use crate::compiler::lexer::tokens::{Operator, Paren};
use crate::compiler::parser::error::ParseError;
use crate::compiler::parser::exprs::{Expr, ExprStructure, EXPR_SYNC_SET};
use crate::compiler::parser::parser::{Parsed, Parser};

impl Parser {
    pub fn parse_array_index(&mut self) -> Parsed<Expr> {
        self.force_parse_with_default(
            self.default_expr(),
            Parser::try_parse_array_index,
            ParseError::expected_expression_not_found,
            EXPR_SYNC_SET.into(),
        )
    }

    pub fn try_parse_array_index(&mut self) -> Option<Parsed<Expr>> {
        self.safe_try(|s_self| {
            let p_so = s_self.try_parse_paren(Paren::SquareOpen)?;
            let p_expr = s_self.parse_expr();
            let p_sc = s_self.parse_paren(Paren::SquareClose);
            Some(Parsed::merge_parsed_ignore_left_right(p_so, p_expr, p_sc))
        })
    }

    pub fn parse_array_expr(&mut self) -> Parsed<Expr> {
        self.force_parse_with_default(
            self.default_expr(),
            Parser::try_parse_array_expr,
            ParseError::expected_expression_not_found,
            EXPR_SYNC_SET.into(),
        )
    }

    pub fn try_parse_array_expr(&mut self) -> Option<Parsed<Expr>> {
        self.safe_try(|s_self| {
            let p_so = s_self.try_parse_paren(Paren::SquareOpen)?;
            let p_exprs = s_self
                .try_parse_addition(Parser::try_parse_expr, Parser::parse_expr, |c_self| {
                    c_self.try_parse_operator(Operator::Comma)
                })
                .unwrap_or(s_self.new_value(vec![]));
            let p_sc = s_self.parse_paren(Paren::SquareClose);
            Some(Parsed::lift_parsed(
                Parsed::merge_parsed_ignore_left_right(p_so, p_exprs, p_sc),
                |exprs| Expr::new(ExprStructure::ArrayExpr { exprs }),
            ))
        })
    }
}
