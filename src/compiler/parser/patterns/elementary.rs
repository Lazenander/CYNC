use crate::compiler::parser::error::ParseError;
use crate::compiler::parser::exprs::identifier::Identifier;
use crate::compiler::parser::parser::{Parsed, ParsedBox, Parser};
use crate::compiler::parser::patterns::{Pattern, PATTERN_SYNC_SET};
use log::error;

impl Parser {
    pub fn parse_elementary_pattern(&mut self) -> Parsed<Pattern> {
        self.force_parse_with_default(
            Pattern::Wildcard,
            Self::try_parse_elementary_pattern,
            ParseError::expected_pattern_not_found,
            PATTERN_SYNC_SET.into(),
        )
    }

    pub fn try_parse_elementary_pattern(&mut self) -> Option<Parsed<Pattern>> {
        self.safe_try(|s_self| {
            let p_id = s_self.try_parse_identifier()?;
            Some(Parsed::lift_parsed(p_id, |id| {
                Pattern::Elementary(s_self.new_parsed_box(id))
            }))
        })
    }
}
