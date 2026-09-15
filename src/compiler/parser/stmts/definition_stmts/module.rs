use crate::compiler::lexer::tokens::{Keyword, Paren};
use crate::compiler::parser::exprs::identifier::Identifier;
use crate::compiler::parser::parser::{Parsed, ParsedBox, Parser};
use crate::compiler::parser::stmts::definition_stmts::DefinitionStmts;

#[derive(Debug, Clone)]
pub struct Module {
    pub is_export: bool,
    pub module_name: ParsedBox<Identifier>,
    pub module_stmts: DefinitionStmts,
}

impl Parser {
    pub fn parse_module(&mut self) -> Parsed<Module> {
        let o_p_export = self.try_parse_keyword(Keyword::Export);
        let p_o_export = self.push_option(o_p_export);
        let p_module = self.parse_keyword(Keyword::Module);
        let p_name = Parsed::box_parsed(self.parse_identifier());
        let p_co = self.parse_paren(Paren::CurlyOpen);
        let p_block = self.parse_definition_stmts();
        let p_cc = self.parse_paren(Paren::CurlyClose);
        Parsed::merge_parsed_3(
            p_o_export,
            Parsed::merge_parsed_ignore_left(p_module, p_name),
            Parsed::merge_parsed_ignore_left_right(p_co, p_block, p_cc),
            |export, module_name, module_stmts| Module {
                is_export: export.is_some(),
                module_name,
                module_stmts,
            },
        )
    }
}
