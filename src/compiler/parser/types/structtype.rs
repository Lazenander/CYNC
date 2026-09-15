use crate::compiler::lexer::tokens::{Operator, Paren, TokenGut};
use crate::compiler::parser::error::ParseError;
use crate::compiler::parser::exprs::identifier::Identifier;
use crate::compiler::parser::parser::{Parsed, ParsedBox, Parser};
use crate::compiler::parser::types::{Type, TypeGut};

#[derive(Debug, Clone)]
pub struct StructCase {
    pub id: ParsedBox<Identifier>,
    pub a_type: Option<ParsedBox<Type>>,
    pub default: Option<ParsedBox<Identifier>>,
}

#[derive(Debug, Clone)]
pub struct StructType {
    pub entries: Vec<ParsedBox<StructCase>>,
}

impl Parser {
    pub fn parse_struct_type_element(&mut self) -> Parsed<StructCase> {
        self.force_parse_with_default(
            StructCase {
                id: self.new_parsed_box(Identifier {
                    name: "".to_string(),
                }),
                a_type: None,
                default: None,
            },
            Parser::try_parse_struct_type_element,
            ParseError::expected_struct_type_element_not_found,
            vec![
                TokenGut::Operator(Operator::Comma),
                TokenGut::Paren(Paren::CurlyClose),
            ],
        )
    }

    pub fn try_parse_struct_type_element(&mut self) -> Option<Parsed<StructCase>> {
        self.safe_try(|s_self| {
            let p_name = Parsed::box_parsed(s_self.try_parse_identifier()?);
            let o_p_type: Option<Parsed<ParsedBox<Type>>> = s_self.safe_try(|ss_self| {
                let p_c = ss_self.try_parse_operator(Operator::Colon)?;
                let p_type = ss_self.parse_type();
                let p_type = Parsed::box_parsed(p_type);
                Some(Parsed::merge_parsed_ignore_left(p_c, p_type))
            });
            let o_p_default: Option<Parsed<ParsedBox<Identifier>>> = s_self.safe_try(|ss_self| {
                let p_assign = ss_self.try_parse_operator(Operator::Assign)?;
                let p_expr = ss_self.parse_identifier();
                let p_expr = Parsed::box_parsed(p_expr);
                Some(Parsed::merge_parsed_ignore_left(p_assign, p_expr))
            });
            match (o_p_type, o_p_default) {
                (None, Some(p_default)) => {
                    Some(Parsed::merge_parsed(p_name, p_default, |id, default| {
                        StructCase {
                            id,
                            a_type: None,
                            default: Some(default),
                        }
                    }))
                }
                (Some(p_type), None) => Some(Parsed::merge_parsed(p_name, p_type, |id, a_type| {
                    StructCase {
                        id,
                        a_type: Some(a_type),
                        default: None,
                    }
                })),
                _ => None,
            }
        })
    }

    pub fn try_parse_struct_type_content(&mut self) -> Option<Parsed<StructType>> {
        self.safe_try(|s_self| {
            let p_curly_o = s_self.try_parse_paren(Paren::CurlyOpen)?;

            let p_elements = s_self.parse_addition(Parser::parse_struct_type_element, |c_self| {
                c_self.try_parse_operator(Operator::Comma)
            });

            let p_curly_c = s_self.parse_paren(Paren::CurlyClose);

            return Some(Parsed::lift_parsed(
                Parsed::merge_parsed_ignore_left_right(p_curly_o, p_elements, p_curly_c),
                |v| StructType { entries: v },
            ));
        })
    }

    pub fn parse_curlyed_type_gut(&mut self) -> Parsed<TypeGut> {
        self.force_parse_with_default(
            TypeGut::Unit,
            Self::try_parse_curlyed_type_gut,
            ParseError::expected_type_not_found,
            vec![],
        )
    }

    pub fn try_parse_curlyed_type_gut(&mut self) -> Option<Parsed<TypeGut>> {
        self.safe_try(|s_self| {
            let curlyed_type = s_self.try_parse_struct_type_content()?;
            Some(Parsed::lift_parsed(curlyed_type, |v| TypeGut::Struct(v)))
        })
    }
}
