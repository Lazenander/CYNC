use crate::compiler::parser::parser::{Parsed, ParsedBox, Parser};
use crate::compiler::parser::stmts::definition_stmts::DefinitionStmts;

#[derive(Debug, Clone)]
pub struct Program {
    pub stmts: DefinitionStmts,
}

impl Parser {
    pub fn parse_program(&mut self) -> Parsed<Program> {
        let p_stmts = self.parse_definition_stmts();
        let p_program = Parsed::lift_parsed(p_stmts, |stmts| Program { stmts });
        let p_eof = self.parse_eof();
        Parsed::merge_parsed_ignore_right(p_program, p_eof)
    }
}
