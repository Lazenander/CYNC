use crate::compiler::lexer::tokens::Keyword;
use crate::compiler::parser::error::ParseError;
use crate::compiler::parser::exprs::Expr;
use crate::compiler::parser::parser::{Parsed, ParsedBox, Parser};
use crate::compiler::parser::patterns::Pattern;
use crate::compiler::parser::stmts::sequential_stmts::r#if::IfBlock;
use crate::compiler::parser::stmts::sequential_stmts::{SequentialBlock, SEQUENTIAL_STMT_SYNC_SET};

#[derive(Debug, Clone)]
pub struct ForStmt {
    item: ParsedBox<Pattern>,
    iterable: ParsedBox<Expr>,
    block: ParsedBox<SequentialBlock>,
}

impl Parser {
    pub fn parse_for_stmt(&mut self) -> Parsed<ForStmt> {
        let p_for = self.parse_keyword(Keyword::For);
        let p_item = Parsed::box_parsed(self.parse_pattern());
        let p_in = self.parse_keyword(Keyword::In);
        let p_expr = Parsed::box_parsed(self.parse_expr());
        let p_block = Parsed::box_parsed(self.parse_sequential_block());
        Parsed::merge_parsed_3(
            Parsed::merge_parsed_ignore_left(p_for, p_item),
            Parsed::merge_parsed_ignore_left(p_in, p_expr),
            p_block,
            |item, iterable, block| ForStmt {
                item,
                iterable,
                block,
            },
        )
    }
}
