use crate::compiler::lexer::tokens::Keyword;
use crate::compiler::parser::error::ParseError;
use crate::compiler::parser::parser::{Parsed, ParsedBox, Parser};
use crate::compiler::parser::stmts::sequential_stmts::r#let::LetElement;
use crate::compiler::parser::stmts::sequential_stmts::{
    LetStmt, SequentialBlock, SEQUENTIAL_STMT_SYNC_SET,
};

#[derive(Debug, Clone)]
pub struct IfLetBlock {
    pub condition: Vec<ParsedBox<LetElement>>,
    pub block: ParsedBox<SequentialBlock>,
}

#[derive(Debug, Clone)]
pub struct IfLetStmt {
    if_block: ParsedBox<IfLetBlock>,
    else_if_blocks: Vec<ParsedBox<IfLetBlock>>,
    else_block: Option<ParsedBox<SequentialBlock>>,
}

impl Parser {
    pub fn parse_if_let_block(&mut self) -> Parsed<IfLetBlock> {
        let p_if = self.parse_keyword(Keyword::If);
        let p_let = self.parse_keyword(Keyword::Let);
        let p_letlements = self.parse_letelements();
        let p_block = Parsed::box_parsed(self.parse_sequential_block());
        Parsed::merge_parsed(
            Parsed::merge_parsed_ignore_left(
                Parsed::merge_parsed_ignore_left(p_if, p_let),
                p_letlements,
            ),
            p_block,
            |condition, block| IfLetBlock { condition, block },
        )
    }

    pub fn try_parse_else_if_let_block(&mut self) -> Option<Parsed<IfLetBlock>> {
        self.safe_try(|s_self| {
            let p_else = s_self.try_parse_keyword(Keyword::Else)?;
            let p_if_let_block = s_self.parse_if_let_block();
            Some(Parsed::merge_parsed_ignore_left(p_else, p_if_let_block))
        })
    }

    pub fn parse_if_let_stmt(&mut self) -> Parsed<IfLetStmt> {
        let p_if_block = Parsed::box_parsed(self.parse_if_let_block());
        let p_else_if_blocks = self.parse_star(Parser::try_parse_else_if_let_block);
        let o_p_else_block = self.try_parse_else_block().map(Parsed::box_parsed);
        match o_p_else_block {
            Some(p_else_block) => Parsed::merge_parsed_3(
                p_if_block,
                p_else_if_blocks,
                p_else_block,
                |if_block, else_if_blocks, else_block| IfLetStmt {
                    if_block,
                    else_if_blocks,
                    else_block: Some(else_block),
                },
            ),
            None => {
                Parsed::merge_parsed(p_if_block, p_else_if_blocks, |if_block, else_if_blocks| {
                    IfLetStmt {
                        if_block,
                        else_if_blocks,
                        else_block: None,
                    }
                })
            }
        }
    }
}
