use crate::compiler::parser::error::ParseError;
use crate::compiler::parser::exprs::identifier::RoutedIdentifier;
use crate::compiler::parser::parser::{Parsed, ParsedBox, Parser};
use crate::compiler::parser::template::TemplateCall;
use crate::compiler::parser::types::TypeGut;

#[derive(Debug, Clone)]
pub struct IdentifierType {
    pub rid: ParsedBox<RoutedIdentifier>,
    pub template_call: Option<ParsedBox<TemplateCall>>,
    // pub paren_type: Option<ParsedBox<TupleType>>,
}

impl Parser {
    pub fn parse_identifier_prefix_type_gut(&mut self) -> Parsed<TypeGut> {
        self.force_parse_with_default(
            TypeGut::Unit,
            Self::try_parse_identifier_prefix_type_gut,
            ParseError::expected_type_not_found,
            vec![],
        )
    }

    pub fn try_parse_identifier_prefix_type_gut(&mut self) -> Option<Parsed<TypeGut>> {
        self.safe_try(|s_self| {
            s_self
                .try_parse_identifier_prefix_type_content()
                .map(|p_id_type| {
                    Parsed::lift_parsed(p_id_type, |id_type| TypeGut::Identifier(id_type))
                })
        })
    }

    pub fn try_parse_identifier_prefix_type_content(&mut self) -> Option<Parsed<IdentifierType>> {
        self.safe_try(|s_self| {
            let p_id = s_self.try_parse_routed_identifier()?;
            let o_p_tc = s_self.try_parse_template_call();
            /*let o_p_tuple = s_self.try_parse_tuple_type();
            Some(match (o_p_tc, o_p_tuple) {
                (Some(p_tc), Some(p_tuple)) => Parsed::merge_parsed_3(
                    p_id,
                    p_tc,
                    p_tuple,
                    |identifier, template_call, p_tuple| IdentifierType {
                        rid: s_self.new_parsed_box(identifier),
                        template_call: Some(s_self.new_parsed_box(template_call)),
                        paren_type: Some(s_self.new_parsed_box(p_tuple)),
                    },
                ),
                (None, Some(p_tuple)) => {
                    Parsed::merge_parsed(p_id, p_tuple, |identifier, p_tuple| IdentifierType {
                        rid: s_self.new_parsed_box(identifier),
                        template_call: None,
                        paren_type: Some(s_self.new_parsed_box(p_tuple)),
                    })
                }
                (Some(p_tc), None) => {
                    Parsed::merge_parsed(p_id, p_tc, |identifier, template_call| IdentifierType {
                        rid: s_self.new_parsed_box(identifier),
                        template_call: Some(s_self.new_parsed_box(template_call)),
                        paren_type: None,
                    })
                }
                (None, None) => Parsed::lift_parsed(p_id, |identifier| IdentifierType {
                    rid: s_self.new_parsed_box(identifier),
                    template_call: None,
                    paren_type: None,
                }),
            })*/
            Some(match o_p_tc {
                Some(p_tc) => {
                    Parsed::merge_parsed(p_id, p_tc, |identifier, template_call| IdentifierType {
                        rid: s_self.new_parsed_box(identifier),
                        template_call: Some(s_self.new_parsed_box(template_call)),
                    })
                }
                None => Parsed::lift_parsed(p_id, |identifier| IdentifierType {
                    rid: s_self.new_parsed_box(identifier),
                    template_call: None,
                }),
            })
        })
    }
}
