use crate::compiler::lexer::tokens::{Operator, Paren};
use crate::compiler::parser::error::ParseError;
use crate::compiler::parser::exprs::identifier::{Identifier, RoutedIdentifier};
use crate::compiler::parser::exprs::literal::Literal;
use crate::compiler::parser::exprs::{Expr, ExprStructure, EXPR_SYNC_SET};
use crate::compiler::parser::parser::{Parsed, ParsedBox, Parser};

impl Parser {
    pub fn parse_structs_element(&mut self) -> Parsed<(ParsedBox<Identifier>, ParsedBox<Expr>)> {
        self.force_parse_with_default(
            (
                self.new_parsed_box(Default::default()),
                self.new_parsed_box(self.default_expr()),
            ),
            Parser::try_parse_structs_element,
            ParseError::expected_struct_element_not_found,
            EXPR_SYNC_SET.into(),
        )
    }

    fn try_parse_structs_element_assignment(&mut self) -> Option<Parsed<Expr>> {
        self.safe_try(|s_self| {
            let p_assign = s_self.try_parse_operator(Operator::Assign)?;
            let p_expr = s_self.parse_expr();
            Some(Parsed::merge_parsed_ignore_left(p_assign, p_expr))
        })
    }

    pub fn try_parse_structs_element(
        &mut self,
    ) -> Option<Parsed<(ParsedBox<Identifier>, ParsedBox<Expr>)>> {
        self.safe_try(|s_self| {
            let p_id = Parsed::box_parsed(s_self.try_parse_identifier()?);
            let o_p_expr = s_self.try_parse_structs_element_assignment();
            match o_p_expr {
                Some(p_expr) => {
                    let p_expr = Parsed::box_parsed(p_expr);
                    Some(Parsed::merge_parsed(p_id, p_expr, |id, expr| (id, expr)))
                }
                None => Some(Parsed::lift_parsed(p_id, |id| {
                    (
                        id.clone(),
                        s_self.new_parsed_box(Expr::new(ExprStructure::Identifier(
                            s_self.new_parsed_box(RoutedIdentifier {
                                route: vec![],
                                this_id: id,
                            }),
                        ))),
                    )
                })),
            }
        })
    }

    pub fn parse_structs_expr(&mut self) -> Parsed<Expr> {
        self.force_parse_with_default(
            self.default_expr(),
            Parser::try_parse_structs_expr,
            ParseError::expected_expression_not_found,
            EXPR_SYNC_SET.into(),
        )
    }

    pub fn try_parse_structs_expr(&mut self) -> Option<Parsed<Expr>> {
        self.safe_try(|s_self| {
            let p_co = s_self.try_parse_paren(Paren::CurlyOpen)?;
            let p_elements = s_self
                .try_parse_addition(
                    Parser::try_parse_structs_element,
                    Parser::parse_structs_element,
                    |c_self| c_self.try_parse_operator(Operator::Comma),
                )
                .unwrap_or(s_self.new_value(vec![]));
            let p_cc = s_self.try_parse_paren(Paren::CurlyClose)?;
            Some(Parsed::lift_parsed(
                Parsed::merge_parsed_ignore_left_right(p_co, p_elements, p_cc),
                |elements| Expr::new(ExprStructure::StructExpr { elements }),
            ))
        })
    }
}
