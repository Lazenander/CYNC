use crate::compiler::lexer::tokens::{Operator, Paren};
use crate::compiler::parser::parser::{Parsed, ParsedBox, Parser};
use crate::compiler::parser::types::Type;

#[derive(Debug, Clone)]
pub struct TemplateCall {
    pub args: Vec<ParsedBox<Type>>,
}

impl Parser {
    pub fn try_parse_template_call(&mut self) -> Option<Parsed<TemplateCall>> {
        self.safe_try(|s_self| {
            let p_co = s_self.try_parse_operator(Operator::Lt)?;
            let p_types = s_self.parse_addition(Parser::parse_type, |c_self| {
                c_self.try_parse_operator(Operator::Comma)
            });
            let p_cc = s_self.parse_operator(Operator::Gt);
            Some(Parsed::lift_parsed(
                Parsed::merge_parsed_ignore_left_right(p_co, p_types, p_cc),
                |types| TemplateCall { args: types },
            ))
        })
    }
}
