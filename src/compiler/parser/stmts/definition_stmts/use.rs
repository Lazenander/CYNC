use crate::compiler::lexer::tokens::Keyword;
use crate::compiler::parser::exprs::identifier::{Identifier, RoutedIdentifier};
use crate::compiler::parser::exprs::FunctionDef;
use crate::compiler::parser::parser::{Parsed, ParsedBox, Parser};
use crate::compiler::parser::template::TemplateDef;

#[derive(Debug, Clone)]
pub struct Use {
    pub is_export: bool,
    pub routed_name: ParsedBox<RoutedIdentifier>,
    pub name: Option<ParsedBox<Identifier>>,
}

impl Parser {
    pub fn parse_use(&mut self) -> Parsed<Use> {
        let o_p_export = self.try_parse_keyword(Keyword::Export);
        let p_o_export = self.push_option(o_p_export);
        let p_use = self.parse_keyword(Keyword::Use);
        let p_rid = Parsed::box_parsed(self.parse_routed_identifier());
        let o_p_as = self
            .safe_try(|s_self| {
                let p_as = s_self.try_parse_keyword(Keyword::As)?;
                let p_id = s_self.parse_identifier();
                Some(Parsed::merge_parsed_ignore_left(p_as, p_id))
            })
            .map(|v| Parsed::box_parsed(v));
        match o_p_as {
            Some(p_as) => Parsed::merge_parsed_3(
                p_o_export,
                Parsed::merge_parsed_ignore_left(p_use, p_rid),
                p_as,
                |export, routed_name, name| Use {
                    is_export: export.is_some(),
                    routed_name,
                    name: Some(name),
                },
            ),
            None => Parsed::merge_parsed(
                p_o_export,
                Parsed::merge_parsed_ignore_left(p_use, p_rid),
                |export, routed_name| Use {
                    is_export: export.is_some(),
                    routed_name,
                    name: None,
                },
            ),
        }
    }
}
