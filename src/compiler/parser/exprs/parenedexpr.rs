use crate::compiler::lexer::tokens::{Operator, Paren};
use crate::compiler::parser::exprs;
use crate::compiler::parser::exprs::literal::Literal;
use crate::compiler::parser::exprs::{Expr, ExprStructure};
use crate::compiler::parser::parser::{Parsed, ParsedBox, Parser};

impl Parser {
    pub fn parse_parened_expr(&mut self) -> Parsed<Expr> {
        let p_p_ro = self.parse_paren(Paren::RoundOpen);
        let mut p_exprs: Vec<Parsed<Expr>> = vec![];
        if let Some(p_expr) = self.try_parse_expr() {
            p_exprs.push(p_expr);
            while let Some(p_c) = self.try_parse_comma() {
                let p_expr = self.parse_expr();
                p_exprs.push(Parsed::merge_parsed_ignore_left(p_c, p_expr));
            }
        }
        let p_p_rc = self.parse_paren(Paren::RoundClose);
        if p_exprs.is_empty() {
            Parsed::merge_parsed_ignore_left_right(
                p_p_ro,
                self.new_value(Expr::new(ExprStructure::Literal(
                    self.new_parsed_box(Literal::Unit),
                ))),
                p_p_rc,
            )
        } else if p_exprs.len() == 1 {
            Parsed::merge_parsed_ignore_left_right(p_p_ro, p_exprs.pop().unwrap(), p_p_rc)
        } else {
            Parsed::merge_parsed_ignore_left_right(
                p_p_ro,
                Parsed::lift_parsed(self.push_vec(p_exprs), |types| {
                    Expr::new(ExprStructure::TupleExpr {
                        exprs: types.into_iter().map(|e| self.new_parsed_box(e)).collect(),
                    })
                }),
                p_p_rc,
            )
        }
    }
}
