use crate::compiler::lexer::tokens::Keyword;
use crate::compiler::parser::error::ParseError;
use crate::compiler::parser::exprs::{Expr, ExprStructure};
use crate::compiler::parser::parser::{Parsed, ParsedBox, Parser};
use crate::compiler::parser::stmts::sequential_stmts::iflet::IfLetBlock;
use crate::compiler::parser::stmts::sequential_stmts::{SequentialBlock, SEQUENTIAL_STMT_SYNC_SET};

#[derive(Debug, Clone)]
pub struct IfBlock {
    pub condition: ParsedBox<Expr>,
    pub block: ParsedBox<SequentialBlock>,
}

#[derive(Debug, Clone)]
pub struct IfStmt {
    if_block: ParsedBox<IfBlock>,
    else_if_blocks: Vec<ParsedBox<IfBlock>>,
    else_block: Option<ParsedBox<SequentialBlock>>,
}

impl Parser {
    pub fn parse_if_block(&mut self) -> Parsed<IfBlock> {
        let p_if = self.parse_keyword(Keyword::If);
        let p_expr = Parsed::box_parsed(self.parse_expr());
        let p_block = Parsed::box_parsed(self.parse_sequential_block());
        Parsed::merge_parsed(
            Parsed::merge_parsed_ignore_left(p_if, p_expr),
            p_block,
            |condition, block| IfBlock { condition, block },
        )
    }

    pub fn try_parse_else_if_block(&mut self) -> Option<Parsed<IfBlock>> {
        self.safe_try(|s_self| {
            let p_else = s_self.try_parse_keyword(Keyword::Else)?;
            let p_if_block = s_self.parse_if_block();
            Some(Parsed::merge_parsed_ignore_left(p_else, p_if_block))
        })
    }

    pub fn try_parse_else_block(&mut self) -> Option<Parsed<SequentialBlock>> {
        self.safe_try(|s_self| {
            let p_else = s_self.try_parse_keyword(Keyword::Else)?;
            let p_block = s_self.parse_sequential_block();
            Some(Parsed::merge_parsed_ignore_left(p_else, p_block))
        })
    }

    pub fn parse_if_stmt(&mut self) -> Parsed<IfStmt> {
        let p_if_block = Parsed::box_parsed(self.parse_if_block());
        let p_else_if_blocks = self.parse_star(Parser::try_parse_else_if_block);
        let o_p_else_block = self.try_parse_else_block().map(Parsed::box_parsed);
        match o_p_else_block {
            Some(p_else_block) => Parsed::merge_parsed_3(
                p_if_block,
                p_else_if_blocks,
                p_else_block,
                |if_block, else_if_blocks, else_block| IfStmt {
                    if_block,
                    else_if_blocks,
                    else_block: Some(else_block),
                },
            ),
            None => {
                Parsed::merge_parsed(p_if_block, p_else_if_blocks, |if_block, else_if_blocks| {
                    IfStmt {
                        if_block,
                        else_if_blocks,
                        else_block: None,
                    }
                })
            }
        }
    }
}
