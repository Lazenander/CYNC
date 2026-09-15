use crate::compiler::lexer::tokens::{Operator, Paren};
use crate::compiler::parser::error::ParseError;
use crate::compiler::parser::exprs::{
    Expr, ExprStructure, FunctionBlock, FunctionDef, EXPR_SYNC_SET,
};
use crate::compiler::parser::parser::{Parsed, Parser};
use crate::compiler::parser::stmts::sequential_stmts::SequentialBlock;

impl Parser {
    pub fn parse_construct_expr(&mut self) -> Parsed<Expr> {
        self.force_parse_with_default(
            self.default_expr(),
            Parser::try_parse_construct_expr,
            ParseError::expected_expression_not_found,
            EXPR_SYNC_SET.into(),
        )
    }

    pub fn try_parse_construct_expr(&mut self) -> Option<Parsed<Expr>> {
        self.safe_try(|s_self| {
            let p_id = Parsed::box_parsed(s_self.try_parse_routed_identifier()?);
            let p_co = s_self.try_parse_paren(Paren::CurlyOpen)?;
            let p_elements = s_self
                .try_parse_addition(
                    Parser::try_parse_structs_element,
                    Parser::parse_structs_element,
                    |c_self| c_self.try_parse_operator(Operator::Comma),
                )
                .unwrap_or(s_self.new_value(vec![]));
            let p_cc = s_self.parse_paren(Paren::CurlyClose);
            Some(Parsed::merge_parsed(
                p_id,
                Parsed::merge_parsed_ignore_left_right(p_co, p_elements, p_cc),
                |name, elements| Expr::new(ExprStructure::ConstructExpr { name, elements }),
            ))
        })
    }
}
