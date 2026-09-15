use crate::compiler::lexer::tokens::TokenGut;
use crate::compiler::parser::error::ParseError;
use crate::compiler::parser::parser::{Parsed, Parser};
use crate::compiler::parser::patterns::{Pattern, PATTERN_SYNC_SET};

impl Parser {
    pub fn parse_wildcard_pattern(&mut self) -> Parsed<Pattern> {
        self.force_parse_with_default(
            Pattern::Wildcard,
            Self::try_parse_wildcard_pattern,
            ParseError::expected_pattern_not_found,
            PATTERN_SYNC_SET.into(),
        )
    }

    pub fn try_parse_wildcard_pattern(&mut self) -> Option<Parsed<Pattern>> {
        self.safe_try(|s_self| match s_self.current_token()?.t_gut {
            TokenGut::WildCard => {
                s_self.next();
                Some(s_self.new_value(Pattern::Wildcard))
            }
            _ => None,
        })
    }
}
