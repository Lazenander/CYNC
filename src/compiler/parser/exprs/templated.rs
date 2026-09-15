use crate::compiler::parser::error::ParseError;
use crate::compiler::parser::exprs::identifier::RoutedIdentifier;
use crate::compiler::parser::exprs::{Expr, ExprStructure, EXPR_SYNC_SET};
use crate::compiler::parser::parser::{Parsed, ParsedBox, Parser};
use crate::compiler::parser::template::TemplateCall;

#[derive(Debug, Clone)]
pub struct TemplateCallRoutedIdentifier {
    rid: ParsedBox<RoutedIdentifier>,
    template_call: Option<ParsedBox<TemplateCall>>,
}

impl Parser {
    pub fn parse_templated_id(&mut self) -> Parsed<TemplateCallRoutedIdentifier> {
        self.force_parse_with_default(
            TemplateCallRoutedIdentifier {
                rid: self.new_parsed_box(self.default_routed_identifier()),
                template_call: None,
            },
            Parser::try_parse_templated_id,
            ParseError::expected_expression_not_found,
            EXPR_SYNC_SET.into(),
        )
    }

    pub fn try_parse_templated_id(&mut self) -> Option<Parsed<TemplateCallRoutedIdentifier>> {
        self.safe_try(|s_self| {
            let p_id = Parsed::box_parsed(s_self.try_parse_routed_identifier()?);
            let o_p_template_call = s_self.try_parse_template_call().map(Parsed::box_parsed);
            match o_p_template_call {
                Some(p_template_call) => Some(Parsed::merge_parsed(
                    p_id,
                    p_template_call,
                    |rid, template_call| TemplateCallRoutedIdentifier {
                        rid,
                        template_call: Some(template_call),
                    },
                )),
                None => Some(Parsed::lift_parsed(p_id, |rid| {
                    TemplateCallRoutedIdentifier {
                        rid,
                        template_call: None,
                    }
                })),
            }
        })
    }

    pub fn parse_templated_expr(&mut self) -> Parsed<Expr> {
        self.force_parse_with_default(
            self.default_expr(),
            Parser::try_parse_templated_expr,
            ParseError::expected_expression_not_found,
            EXPR_SYNC_SET.into(),
        )
    }

    pub fn try_parse_templated_expr(&mut self) -> Option<Parsed<Expr>> {
        self.safe_try(|s_self| {
            let p_id = Parsed::box_parsed(s_self.try_parse_templated_id()?);
            Some(Parsed::lift_parsed(p_id, |id| {
                Expr::new(ExprStructure::Templated(id))
            }))
        })
    }
}
