use crate::compiler::lexer::tokens::Operator;
use crate::compiler::parser::error::ParseError;
use crate::compiler::parser::exprs::identifier::Identifier;
use crate::compiler::parser::parser::{Parsed, ParsedBox, Parser};
use crate::compiler::parser::patterns::{Pattern, PATTERN_SYNC_SET};
use crate::compiler::parser::types::Type;

#[derive(Debug, Clone)]
pub struct AnnotatedPattern {
    id: ParsedBox<Identifier>,
    a_type: ParsedBox<Type>,
}

impl Parser {
    pub fn parse_annotated_pattern(&mut self) -> Parsed<Pattern> {
        self.force_parse_with_default(
            Pattern::Wildcard,
            Self::try_parse_annotated_pattern,
            ParseError::expected_pattern_not_found,
            PATTERN_SYNC_SET.into(),
        )
    }

    pub fn try_parse_annotated_pattern(&mut self) -> Option<Parsed<Pattern>> {
        self.safe_try(|s_self| {
            Some(Parsed::lift_parsed(
                s_self.try_parse_annotated()?,
                |annotated| Pattern::Annotated(s_self.new_parsed_box(annotated)),
            ))
        })
    }

    pub fn try_parse_annotated(&mut self) -> Option<Parsed<AnnotatedPattern>> {
        self.safe_try(|s_self| {
            let p_id = s_self.try_parse_identifier()?;
            let p_c = s_self.try_parse_operator(Operator::Colon)?;
            let p_a_type = s_self.parse_type();
            Some(Parsed::merge_parsed(
                Parsed::merge_parsed_ignore_right(Parsed::box_parsed(p_id), p_c),
                Parsed::box_parsed(p_a_type),
                |id, a_type| AnnotatedPattern { id, a_type },
            ))
        })
    }
}
