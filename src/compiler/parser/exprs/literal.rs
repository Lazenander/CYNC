use crate::compiler::lexer::tokens;
use crate::compiler::lexer::tokens::{Paren, Position, TokenGut};
use crate::compiler::parser::error::ParseError;
use crate::compiler::parser::exprs::{Expr, ExprStructure, EXPR_SYNC_SET};
use crate::compiler::parser::parser::{Parsed, ParsedBox, Parser};

#[derive(Debug, Clone)]
pub enum Literal {
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Character(char),
    String(String),
    Unit,
}

impl Parser {
    pub fn parse_literal_expr(&mut self) -> Parsed<Expr> {
        self.force_parse_with_default(
            self.default_expr(),
            Parser::try_parse_literal_expr,
            ParseError::expected_expression_not_found,
            EXPR_SYNC_SET.into(),
        )
    }

    pub fn try_parse_literal_expr(&mut self) -> Option<Parsed<Expr>> {
        self.safe_try(|s_self| {
            Some(Parsed::lift_parsed(
                s_self.try_parse_literal()?,
                |literal| Expr::new(ExprStructure::Literal(s_self.new_parsed_box(literal))),
            ))
        })
    }

    pub fn parse_literal(&mut self) -> Parsed<Literal> {
        self.force_parse_with_default(
            Literal::Unit,
            Parser::try_parse_literal,
            ParseError::expected_literal_not_found,
            EXPR_SYNC_SET.into(),
        )
    }

    pub fn try_parse_unit(&mut self) -> Option<Parsed<()>> {
        self.safe_try(|s_self| {
            let p_ro = s_self.try_parse_paren(Paren::RoundOpen)?;
            let p_rc = s_self.try_parse_paren(Paren::RoundClose)?;
            Some(Parsed::merge_parsed(p_ro, p_rc, |_, _| ()))
        })
    }

    pub fn try_parse_literal(&mut self) -> Option<Parsed<Literal>> {
        self.safe_try(|s_self| {
            let token = s_self.current_token()?;
            match token.t_gut {
                TokenGut::Literal(tokens::Literal::Integer(value)) => {
                    s_self.next();
                    Some(s_self.new_value(Literal::Integer(value)))
                }
                TokenGut::Literal(tokens::Literal::Float(value)) => {
                    s_self.next();
                    Some(s_self.new_value(Literal::Float(value)))
                }
                TokenGut::Literal(tokens::Literal::Boolean(value)) => {
                    s_self.next();
                    Some(s_self.new_value(Literal::Boolean(value)))
                }
                TokenGut::Literal(tokens::Literal::Character(value)) => {
                    s_self.next();
                    Some(s_self.new_value(Literal::Character(value)))
                }
                TokenGut::Literal(tokens::Literal::String(value)) => {
                    s_self.next();
                    Some(s_self.new_value(Literal::String(value)))
                }
                _ => {
                    if let Some(_) = s_self.try_parse_unit() {
                        Some(s_self.new_value(Literal::Unit))
                    } else {
                        None
                    }
                }
            }
        })
    }
}
