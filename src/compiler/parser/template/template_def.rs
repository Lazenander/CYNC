use crate::compiler::lexer::tokens::Operator;
use crate::compiler::parser::error::ParseError;
use crate::compiler::parser::exprs::identifier::Identifier;
use crate::compiler::parser::parser::{Parsed, ParsedBox, Parser};
use crate::compiler::parser::types::Type;

#[derive(Debug, Clone)]
pub struct TemplateDef {
    pub args: Vec<ParsedBox<(ParsedBox<Identifier>, Option<ParsedBox<Type>>)>>,
}

impl Parser {
    pub fn parse_template_def_element(
        &mut self,
        need_annotation: bool,
    ) -> Parsed<(ParsedBox<Identifier>, Option<ParsedBox<Type>>)> {
        self.force_parse_with_default(
            (self.new_parsed_box(Default::default()), None),
            |s_self| s_self.try_parse_template_def_element(need_annotation),
            ParseError::expected_template_def_element_not_found,
            vec![],
        )
    }

    pub fn try_parse_template_def_element(
        &mut self,
        need_annotation: bool,
    ) -> Option<Parsed<(ParsedBox<Identifier>, Option<ParsedBox<Type>>)>> {
        self.safe_try(|s_self| {
            let p_id = s_self.try_parse_identifier()?;
            let o_p_type = s_self.safe_try(|ss_self| {
                let p_c = ss_self.try_parse_operator(Operator::Colon)?;
                let p_type = ss_self.try_parse_type()?;
                Some(Parsed::merge_parsed_ignore_left(p_c, p_type))
            });
            match o_p_type {
                Some(p_type) => Some(Parsed::merge_parsed(p_id, p_type, |id, t| {
                    (s_self.new_parsed_box(id), Some(s_self.new_parsed_box(t)))
                })),
                None => Some(Parsed::lift_parsed(p_id, |id| {
                    (s_self.new_parsed_box(id), None)
                })),
            }
        })
    }

    pub fn try_parse_template_def(&mut self, need_annotation: bool) -> Option<Parsed<TemplateDef>> {
        self.safe_try(|s_self| {
            let p_co = s_self.try_parse_operator(Operator::Lt)?;
            let p_types = s_self.parse_addition(
                |ss_self| ss_self.parse_template_def_element(need_annotation),
                |c_self| c_self.try_parse_operator(Operator::Comma),
            );
            let p_cc = s_self.parse_operator(Operator::Gt);
            Some(Parsed::lift_parsed(
                Parsed::merge_parsed_ignore_left_right(p_co, p_types, p_cc),
                |types| TemplateDef { args: types },
            ))
        })
    }
}
