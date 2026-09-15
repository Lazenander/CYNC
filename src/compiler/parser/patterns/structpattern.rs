use crate::compiler::lexer::tokens::Operator::Comma;
use crate::compiler::lexer::tokens::Paren::{CurlyClose, CurlyOpen};
use crate::compiler::lexer::tokens::TokenGut::{Semicolon, EOF};
use crate::compiler::lexer::tokens::{Operator, Paren, TokenGut};
use crate::compiler::parser::error::ParseError;
use crate::compiler::parser::exprs::identifier::Identifier;
use crate::compiler::parser::exprs::{Expr, ExprStructure};
use crate::compiler::parser::parser::{Parsed, ParsedBox, Parser};
use crate::compiler::parser::patterns::{Pattern, PATTERN_SYNC_SET};
use crate::compiler::parser::types::Type;

impl Parser {
    pub fn parse_struct_pattern_element(
        &mut self,
    ) -> Parsed<(ParsedBox<Identifier>, ParsedBox<Pattern>)> {
        self.force_parse_with_default(
            (
                self.new_parsed_box(Default::default()),
                self.new_parsed_box(Pattern::Wildcard),
            ),
            Parser::try_parse_struct_pattern_element,
            ParseError::expected_struct_pattern_element_not_found,
            vec![
                EOF,
                TokenGut::Paren(CurlyClose),
                Semicolon,
                TokenGut::Operator(Comma),
            ],
        )
    }

    fn try_parse_struct_pattern_assign(&mut self) -> Option<Parsed<Pattern>> {
        self.safe_try(|s_self| {
            let p_assign = s_self.try_parse_operator(Operator::Assign)?;
            let p_pattern = s_self.try_parse_pattern()?;
            Some(Parsed::merge_parsed_ignore_left(p_assign, p_pattern))
        })
    }

    pub fn try_parse_struct_pattern_element(
        &mut self,
    ) -> Option<Parsed<(ParsedBox<Identifier>, ParsedBox<Pattern>)>> {
        self.safe_try(|s_self| {
            let p_name = Parsed::box_parsed(s_self.try_parse_identifier()?);
            let o_p_pattern = s_self
                .try_parse_struct_pattern_assign()
                .map(Parsed::box_parsed);
            match o_p_pattern {
                Some(p_pattern) => {
                    Some(Parsed::merge_parsed(p_name, p_pattern, |name, pattern| {
                        (name, pattern)
                    }))
                }
                None => Some(Parsed::lift_parsed(p_name, |name| {
                    (
                        name.clone(),
                        s_self.new_parsed_box(Pattern::Elementary(name)),
                    )
                })),
            }
        })
    }

    pub fn parse_struct_pattern(&mut self) -> Parsed<Pattern> {
        self.force_parse_with_default(
            Pattern::Wildcard,
            Parser::try_parse_struct_pattern,
            ParseError::expected_struct_pattern_element_not_found,
            PATTERN_SYNC_SET.into(),
        )
    }

    pub fn try_parse_struct_pattern(&mut self) -> Option<Parsed<Pattern>> {
        self.safe_try(|s_self| {
            let p_curly_o = s_self.try_parse_paren(Paren::CurlyOpen)?;

            let p_elements = s_self
                .parse_addition(Parser::parse_struct_pattern_element, |c_self| {
                    c_self.try_parse_operator(Operator::Comma)
                });

            let p_curly_c = s_self.try_parse_paren(Paren::CurlyClose)?;

            return Some(Parsed::lift_parsed(
                Parsed::merge_parsed_ignore_left_right(p_curly_o, p_elements, p_curly_c),
                |v| Pattern::Struct(v),
            ));
        })
    }
}
