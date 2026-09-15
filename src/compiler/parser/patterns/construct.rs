use crate::compiler::lexer::tokens::Paren;
use crate::compiler::parser::error::ParseError;
use crate::compiler::parser::exprs::identifier::RoutedIdentifier;
use crate::compiler::parser::parser::{Parsed, ParsedBox, Parser};
use crate::compiler::parser::patterns::{Pattern, PATTERN_SYNC_SET};
use crate::compiler::parser::template::TemplateCall;

#[derive(Debug, Clone)]
pub struct ConstructPattern {
    pub rid: ParsedBox<RoutedIdentifier>,
    pub template_call: Option<ParsedBox<TemplateCall>>,
    pub pattern: Vec<ParsedBox<Pattern>>,
}

impl Parser {
    pub fn parse_construct_patter(&mut self) -> Parsed<Pattern> {
        self.force_parse_with_default(
            Pattern::Wildcard,
            Self::try_parse_construct_pattern,
            ParseError::expected_pattern_not_found,
            PATTERN_SYNC_SET.into(),
        )
    }

    pub fn try_parse_construct_pattern(&mut self) -> Option<Parsed<Pattern>> {
        self.safe_try(|s_self| {
            let p_content = s_self.try_parse_construct_pattern_content()?;
            Some(Parsed::lift_parsed(p_content, |content| {
                Pattern::Construct(content)
            }))
        })
    }

    pub fn try_parse_construct_pattern_content(&mut self) -> Option<Parsed<ConstructPattern>> {
        self.safe_try(|s_self| {
            let p_id = Parsed::box_parsed(s_self.try_parse_routed_identifier()?);
            let o_p_template_call = s_self.try_parse_template_call();
            let p_patterns =
                Parsed::lift_parsed(s_self.try_parse_parened_patterns()?, |patterns| {
                    patterns
                        .into_iter()
                        .map(|p| s_self.new_parsed_box(p))
                        .collect()
                });
            match o_p_template_call {
                Some(p_template_call) => {
                    let p_template_call = Parsed::box_parsed(p_template_call);
                    Some(Parsed::merge_parsed_3(
                        p_id,
                        p_template_call,
                        p_patterns,
                        |rid, template_call, pattern| ConstructPattern {
                            rid,
                            template_call: Some(template_call),
                            pattern,
                        },
                    ))
                }
                None => Some(Parsed::merge_parsed(p_id, p_patterns, |rid, pattern| {
                    ConstructPattern {
                        rid,
                        template_call: None,
                        pattern,
                    }
                })),
            }
        })
    }
}
