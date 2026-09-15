use crate::compiler::lexer::tokens::Paren;
use crate::compiler::parser::error::ParseError;
use crate::compiler::parser::parser::{Parsed, ParsedBox, Parser};
use crate::compiler::parser::types::{Type, TypeGut};
use logos::Source;

#[derive(Debug, Clone)]
pub struct TupleType {
    pub entries: Vec<ParsedBox<Type>>,
}

impl Parser {
    pub fn parse_parened_type_gut(&mut self) -> Parsed<TypeGut> {
        self.force_parse_with_default(
            TypeGut::Unit,
            Self::try_parse_parened_type_gut,
            ParseError::expected_type_not_found,
            vec![],
        )
    }

    pub fn try_parse_parened_type_gut(&mut self) -> Option<Parsed<TypeGut>> {
        self.safe_try(|s_self| {
            let p_types = s_self.try_parse_in_paren_types()?;

            Some(Parsed::lift_parsed(p_types, |mut types| {
                if types.is_empty() {
                    TypeGut::Unit
                } else if types.len() == 1 {
                    TypeGut::ParenSingle(s_self.new_parsed_box(types.pop().unwrap()))
                } else {
                    TypeGut::Tuple(TupleType {
                        entries: types
                            .into_iter()
                            .map(|t| s_self.new_parsed_box(t))
                            .collect(),
                    })
                }
            }))
        })
    }

    pub fn try_parse_tuple_type(&mut self) -> Option<Parsed<TupleType>> {
        self.safe_try(|s_self| {
            let p_types = s_self.try_parse_in_paren_types()?;
            Some(Parsed::lift_parsed(p_types, |mut types| TupleType {
                entries: types
                    .into_iter()
                    .map(|t| s_self.new_parsed_box(t))
                    .collect(),
            }))
        })
    }

    pub fn try_parse_in_paren_types(&mut self) -> Option<Parsed<Vec<Type>>> {
        self.safe_try(|s_self| {
            let p_p_ro = s_self.try_parse_paren(Paren::RoundOpen)?;
            let mut p_types: Vec<Parsed<Type>> = vec![];
            if let Some(p_type) = s_self.try_parse_type() {
                p_types.push(p_type);
                while let Some(p_c) = s_self.try_parse_comma() {
                    let p_type = s_self.parse_type();
                    p_types.push(Parsed::merge_parsed_ignore_left(p_c, p_type));
                }
            }
            let p_p_rc = s_self.parse_paren(Paren::RoundClose);
            Some(Parsed::merge_parsed_ignore_left_right(
                p_p_ro,
                s_self.push_vec(p_types),
                p_p_rc,
            ))
        })
    }
}
