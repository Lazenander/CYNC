use crate::compiler::lexer::tokens::Keyword;
use crate::compiler::parser::exprs::identifier::Identifier;
use crate::compiler::parser::exprs::FunctionDef;
use crate::compiler::parser::parser::{Parsed, ParsedBox, Parser};
use crate::compiler::parser::template::TemplateDef;

#[derive(Debug, Clone)]
pub struct OperationDef {
    pub is_export: bool,
    pub name: ParsedBox<Identifier>,
    pub template_def: Option<ParsedBox<TemplateDef>>,
    pub function_def: ParsedBox<FunctionDef>,
}

impl Parser {
    pub fn parse_operation_def(&mut self) -> Parsed<OperationDef> {
        let o_p_export = self.try_parse_keyword(Keyword::Export);
        let p_o_export = self.push_option(o_p_export);
        let p_operation = self.parse_keyword(Keyword::Operation);
        let p_id = Parsed::box_parsed(self.parse_identifier());
        let o_p_t_def = self.try_parse_template_def(false);
        let p_f_def = Parsed::box_parsed(self.parse_function_def());
        match o_p_t_def {
            Some(p_t_def) => {
                let p_t_def = Parsed::box_parsed(p_t_def);
                Parsed::merge_parsed_3(
                    Parsed::merge_parsed(
                        p_o_export,
                        Parsed::merge_parsed_ignore_left(p_operation, p_id),
                        |export, name| (export, name),
                    ),
                    p_t_def,
                    p_f_def,
                    |(export, name), template_def, function_def| OperationDef {
                        is_export: export.is_some(),
                        name,
                        template_def: Some(template_def),
                        function_def,
                    },
                )
            }
            None => Parsed::merge_parsed_3(
                p_o_export,
                Parsed::merge_parsed_ignore_left(p_operation, p_id),
                p_f_def,
                |export, name, function_def| OperationDef {
                    is_export: export.is_some(),
                    name,
                    template_def: None,
                    function_def,
                },
            ),
        }
    }
}
