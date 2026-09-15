use crate::compiler::lexer::tokens::Keyword;
use crate::compiler::parser::exprs::identifier::{Identifier, RoutedIdentifier};
use crate::compiler::parser::exprs::FunctionDef;
use crate::compiler::parser::parser::{Parsed, ParsedBox, Parser};
use crate::compiler::parser::stmts::definition_stmts::r#use::Use;
use crate::compiler::parser::template::TemplateDef;

impl Parser {
    pub fn parse_export(&mut self) -> Parsed<Use> {
        let p_export = self.parse_keyword(Keyword::Export);
        let p_rid = Parsed::box_parsed(self.parse_routed_identifier());
        let o_p_as = self
            .safe_try(|s_self| {
                let p_as = s_self.try_parse_keyword(Keyword::As)?;
                let p_id = s_self.parse_identifier();
                Some(Parsed::merge_parsed_ignore_left(p_as, p_id))
            })
            .map(|v| Parsed::box_parsed(v));
        match o_p_as {
            Some(p_as) => Parsed::merge_parsed(
                Parsed::merge_parsed_ignore_left(p_export, p_rid),
                p_as,
                |routed_name, name| Use {
                    is_export: true,
                    routed_name,
                    name: Some(name),
                },
            ),
            None => Parsed::lift_parsed(
                Parsed::merge_parsed_ignore_left(p_export, p_rid),
                |routed_name| Use {
                    is_export: true,
                    routed_name,
                    name: None,
                },
            ),
        }
    }
}
