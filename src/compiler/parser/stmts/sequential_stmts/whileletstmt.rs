use crate::compiler::lexer::tokens::Keyword;
use crate::compiler::parser::error::ParseError;
use crate::compiler::parser::parser::{Parsed, ParsedBox, Parser};
use crate::compiler::parser::stmts::sequential_stmts::r#let::LetElement;
use crate::compiler::parser::stmts::sequential_stmts::{
    DoWhileStmt, SequentialBlock, SEQUENTIAL_STMT_SYNC_SET,
};

#[derive(Debug, Clone)]
pub struct WhileLetStmt {
    condition: Vec<ParsedBox<LetElement>>,
    block: ParsedBox<SequentialBlock>,
}

impl Parser {
    pub fn parse_while_let_stmt(&mut self) -> Parsed<WhileLetStmt> {
        let p_while = self.parse_keyword(Keyword::While);
        let p_let = self.parse_keyword(Keyword::Let);
        let p_cond = self.parse_letelements();
        let p_block = Parsed::box_parsed(self.parse_sequential_block());
        Parsed::merge_parsed(
            Parsed::merge_parsed_ignore_left(
                Parsed::merge_parsed_ignore_left(p_while, p_let),
                p_cond,
            ),
            p_block,
            |condition, block| WhileLetStmt { condition, block },
        )
    }
}
