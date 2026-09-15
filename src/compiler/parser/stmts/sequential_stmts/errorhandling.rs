use crate::compiler::lexer::tokens::Keyword;
use crate::compiler::parser::exprs::identifier::Identifier;
use crate::compiler::parser::exprs::Expr;
use crate::compiler::parser::parser::{Parsed, ParsedBox, Parser};
use crate::compiler::parser::stmts::sequential_stmts::SequentialBlock;

#[derive(Debug, Clone)]
pub struct ThrowStmt(ParsedBox<Expr>);

#[derive(Debug, Clone)]
pub struct ErrorHandlingStmt {
    try_block: ParsedBox<SequentialBlock>,
    catch_block: Option<(ParsedBox<Identifier>, ParsedBox<SequentialBlock>)>,
    finally_block: Option<ParsedBox<SequentialBlock>>,
}

impl Parser {
    pub fn parse_throw_stmt(&mut self) -> Parsed<ThrowStmt> {
        let p_throw = self.parse_keyword(Keyword::Throw);
        let p_expr = Parsed::box_parsed(self.parse_expr());
        let p_sc = self.parse_semicolon();
        Parsed::lift_parsed(
            Parsed::merge_parsed_ignore_left_right(p_throw, p_expr, p_sc),
            |expr| ThrowStmt(expr),
        )
    }

    fn parse_try_block(&mut self) -> Parsed<SequentialBlock> {
        let p_try = self.parse_keyword(Keyword::Try);
        let p_block = self.parse_sequential_block();
        Parsed::merge_parsed_ignore_left(p_try, p_block)
    }

    fn try_parse_catch_block(
        &mut self,
    ) -> Option<Parsed<(ParsedBox<Identifier>, ParsedBox<SequentialBlock>)>> {
        self.safe_try(|s_self| {
            let p_catch = s_self.try_parse_keyword(Keyword::Catch)?;
            let p_id = Parsed::box_parsed(s_self.parse_identifier());
            let p_block = Parsed::box_parsed(s_self.parse_sequential_block());
            Some(Parsed::merge_parsed(
                Parsed::merge_parsed_ignore_left(p_catch, p_id),
                p_block,
                |id, block| (id, block),
            ))
        })
    }

    fn try_parse_finally_block(&mut self) -> Option<Parsed<SequentialBlock>> {
        self.safe_try(|s_self| {
            let p_final = s_self.try_parse_keyword(Keyword::Finally)?;
            let p_block = s_self.parse_sequential_block();
            Some(Parsed::merge_parsed_ignore_left(p_final, p_block))
        })
    }

    pub fn parse_error_handling_stmt(&mut self) -> Parsed<ErrorHandlingStmt> {
        let p_try_block = Parsed::box_parsed(self.parse_try_block());
        let o_p_catch_block = self.try_parse_catch_block();
        let o_p_finally_block = self.try_parse_finally_block();
        match (o_p_catch_block, o_p_finally_block) {
            (Some(p_catch_block), Some(p_finally_block)) => {
                let p_finally_block = Parsed::box_parsed(p_finally_block);
                Parsed::merge_parsed_3(
                    p_try_block,
                    p_catch_block,
                    p_finally_block,
                    |try_block, catch_block, finally_block| ErrorHandlingStmt {
                        try_block,
                        catch_block: Some(catch_block),
                        finally_block: Some(finally_block),
                    },
                )
            }
            (Some(p_catch_block), None) => {
                Parsed::merge_parsed(p_try_block, p_catch_block, |try_block, catch_block| {
                    ErrorHandlingStmt {
                        try_block,
                        catch_block: Some(catch_block),
                        finally_block: None,
                    }
                })
            }
            (None, Some(p_finally_block)) => {
                let p_finally_block = Parsed::box_parsed(p_finally_block);
                Parsed::merge_parsed(p_try_block, p_finally_block, |try_block, finally_block| {
                    ErrorHandlingStmt {
                        try_block,
                        catch_block: None,
                        finally_block: Some(finally_block),
                    }
                })
            }
            (None, None) => Parsed::lift_parsed(p_try_block, |try_block| ErrorHandlingStmt {
                try_block,
                catch_block: None,
                finally_block: None,
            }),
        }
    }
}
