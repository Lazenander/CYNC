use crate::compiler::lexer::tokens::Paren;
use crate::compiler::parser::error::ParseError;
use crate::compiler::parser::exprs::literal::Literal;
use crate::compiler::parser::exprs::{Expr, ExprStructure};
use crate::compiler::parser::parser::{Parsed, ParsedBox, Parser};
use crate::compiler::parser::patterns::{Pattern, PATTERN_SYNC_SET};
use crate::compiler::parser::types::Type;

impl Parser {
    pub fn parse_parened_patterns(&mut self) -> Parsed<Vec<Pattern>> {
        self.force_parse_with_default(
            vec![],
            Parser::try_parse_parened_patterns,
            ParseError::expected_struct_pattern_element_not_found,
            PATTERN_SYNC_SET.into(),
        )
    }

    pub fn try_parse_parened_patterns(&mut self) -> Option<Parsed<Vec<Pattern>>> {
        self.safe_try(|s_self| {
            let p_p_ro = s_self.try_parse_paren(Paren::RoundOpen)?;
            let mut p_patterns: Vec<Parsed<Pattern>> = vec![];
            if let Some(p_pattern) = s_self.try_parse_pattern() {
                p_patterns.push(p_pattern);
                while let Some(p_c) = s_self.try_parse_comma() {
                    let p_pattern = s_self.parse_pattern();
                    p_patterns.push(Parsed::merge_parsed_ignore_left(p_c, p_pattern));
                }
            }
            let p_p_rc = s_self.parse_paren(Paren::RoundClose);
            Some(Parsed::merge_parsed_ignore_left_right(
                p_p_ro,
                s_self.push_vec(p_patterns),
                p_p_rc,
            ))
        })
    }

    pub fn parse_parened_pattern(&mut self) -> Parsed<Pattern> {
        self.force_parse_with_default(
            Pattern::Wildcard,
            Parser::try_parse_parened_pattern,
            ParseError::expected_struct_pattern_element_not_found,
            PATTERN_SYNC_SET.into(),
        )
    }

    pub fn try_parse_parened_pattern(&mut self) -> Option<Parsed<Pattern>> {
        self.safe_try(|s_self| {
            let p_patterns = s_self.try_parse_parened_patterns()?;
            Some(Parsed::lift_parsed(p_patterns, |mut p_patterns| {
                if p_patterns.is_empty() {
                    Pattern::Literal((s_self.new_parsed_box(Literal::Unit)))
                } else if p_patterns.len() == 1 {
                    p_patterns.pop().unwrap()
                } else {
                    Pattern::Tuple(
                        p_patterns
                            .into_iter()
                            .map(|p| s_self.new_parsed_box(p))
                            .collect(),
                    )
                }
            }))
        })
    }
}
