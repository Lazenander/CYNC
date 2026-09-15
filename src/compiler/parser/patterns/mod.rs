mod annotated;
mod construct;
pub mod elementary;
mod parenedpattern;
mod structpattern;
mod wildcardpattern;

use crate::compiler::lexer::tokens::Operator::{Assign, Colon, Comma};
use crate::compiler::lexer::tokens::{Keyword, Operator, Paren, TokenGut};
use crate::compiler::parser::error::ParseError;
use crate::compiler::parser::exprs::identifier::RoutedIdentifier;
use crate::compiler::parser::exprs::{identifier::Identifier, literal::Literal};
use crate::compiler::parser::parser::{Parsed, ParsedBox, Parser};
use crate::compiler::parser::patterns::annotated::AnnotatedPattern;
use crate::compiler::parser::patterns::construct::ConstructPattern;
use crate::compiler::parser::types::Type;

#[derive(Debug, Clone)]
pub enum Pattern {
    Construct(ConstructPattern),
    Tuple(Vec<ParsedBox<Pattern>>),
    Struct(Vec<ParsedBox<(ParsedBox<Identifier>, ParsedBox<Pattern>)>>),
    Wildcard,
    Literal(ParsedBox<Literal>),
    Elementary(ParsedBox<Identifier>),
    Annotated(ParsedBox<AnnotatedPattern>),
}

const PATTERN_SYNC_SET: [TokenGut; 10] = [
    TokenGut::Operator(Assign),
    TokenGut::Operator(Comma),
    TokenGut::Operator(Colon),
    TokenGut::Operator(Operator::DoubleArrow),
    TokenGut::Semicolon,
    TokenGut::Paren(Paren::RoundOpen),
    TokenGut::Paren(Paren::RoundClose),
    TokenGut::Paren(Paren::CurlyOpen),
    TokenGut::Paren(Paren::CurlyClose),
    TokenGut::EOF,
];

impl Parser {
    pub fn parse_literal_pattern(&mut self) -> Parsed<Pattern> {
        self.force_parse_with_default(
            Pattern::Wildcard,
            Parser::try_parse_literal_pattern,
            ParseError::expected_pattern_not_found,
            PATTERN_SYNC_SET.into(),
        )
    }

    pub fn try_parse_literal_pattern(&mut self) -> Option<Parsed<Pattern>> {
        self.safe_try(|s_self| {
            let p_literal = s_self.try_parse_literal()?;
            Some(Parsed::lift_parsed(
                Parsed::box_parsed(p_literal),
                |b_literal| Pattern::Literal(b_literal),
            ))
        })
    }

    pub fn parse_pattern(&mut self) -> Parsed<Pattern> {
        self.force_parse_with_default(
            Pattern::Wildcard,
            Parser::try_parse_pattern,
            ParseError::expected_pattern_not_found,
            PATTERN_SYNC_SET.into(),
        )
    }

    pub fn try_parse_pattern(&mut self) -> Option<Parsed<Pattern>> {
        self.safe_try(|s_self| {
            let token = s_self.current_token()?;
            match token.t_gut {
                TokenGut::Identifier(_) => Some(s_self.chain_try_parses_with_default_parse(
                    vec![
                        Parser::try_parse_annotated_pattern,
                        Parser::try_parse_construct_pattern,
                    ],
                    Parser::parse_elementary_pattern,
                )),
                TokenGut::Literal(_) => Some(s_self.parse_literal_pattern()),
                TokenGut::WildCard => Some(s_self.parse_wildcard_pattern()),
                TokenGut::Paren(Paren::RoundOpen) => Some(s_self.parse_parened_pattern()),
                TokenGut::Paren(Paren::CurlyOpen) => Some(s_self.parse_struct_pattern()),
                _ => None,
            }
        })
    }
}
