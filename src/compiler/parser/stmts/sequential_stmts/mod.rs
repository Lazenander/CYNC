use crate::compiler::lexer::tokens::TokenGut::{Semicolon, EOF};
use crate::compiler::lexer::tokens::{Keyword, Paren, TokenGut};
use crate::compiler::parser::error::ParseError;
use crate::compiler::parser::parser::{Parsed, ParsedBox, Parser};
pub use crate::compiler::parser::stmts::sequential_stmts::assignment::AssignmentStmt;
pub use crate::compiler::parser::stmts::sequential_stmts::errorhandling::ErrorHandlingStmt;
use crate::compiler::parser::stmts::sequential_stmts::errorhandling::ThrowStmt;
pub use crate::compiler::parser::stmts::sequential_stmts::exprstmt::ExprStmt;
pub use crate::compiler::parser::stmts::sequential_stmts::iflet::IfLetStmt;
pub use crate::compiler::parser::stmts::sequential_stmts::r#for::ForStmt;
pub use crate::compiler::parser::stmts::sequential_stmts::r#if::IfStmt;
pub use crate::compiler::parser::stmts::sequential_stmts::r#let::LetStmt;
pub use crate::compiler::parser::stmts::sequential_stmts::r#match::MatchStmt;
pub use crate::compiler::parser::stmts::sequential_stmts::r#return::ReturnStmt;
pub use crate::compiler::parser::stmts::sequential_stmts::whileletstmt::WhileLetStmt;
pub use crate::compiler::parser::stmts::sequential_stmts::whilestmt::{
    BreakStmt, ContinueStmt, DoWhileStmt, WhileStmt,
};

mod assignment;
mod errorhandling;
mod exprstmt;
mod r#for;
mod r#if;
mod iflet;
mod r#let;
mod r#match;
mod r#return;
mod whileletstmt;
mod whilestmt;

#[derive(Debug, Clone)]
pub struct SequentialBlock {
    pub stmts: Vec<ParsedBox<SequentialStmt>>,
}

#[derive(Debug, Clone)]
pub enum SequentialStmt {
    Empty,
    Assignment(ParsedBox<AssignmentStmt>),
    Throw(ParsedBox<ThrowStmt>),
    ErrorHandling(ParsedBox<ErrorHandlingStmt>),
    Expr(ParsedBox<ExprStmt>),
    For(ParsedBox<ForStmt>),
    IfLet(ParsedBox<IfLetStmt>),
    If(ParsedBox<IfStmt>),
    Let(ParsedBox<LetStmt>),
    Match(ParsedBox<MatchStmt>),
    While(ParsedBox<WhileStmt>),
    DoWhile(ParsedBox<DoWhileStmt>),
    WhileLet(ParsedBox<WhileLetStmt>),
    Continue(ParsedBox<ContinueStmt>),
    Break(ParsedBox<BreakStmt>),
    Return(ParsedBox<ReturnStmt>),
}

const SEQUENTIAL_STMT_SYNC_SET: [TokenGut; 14] = [
    TokenGut::Keyword(Keyword::Let),
    TokenGut::Keyword(Keyword::If),
    TokenGut::Keyword(Keyword::Match),
    TokenGut::Keyword(Keyword::For),
    TokenGut::Keyword(Keyword::While),
    TokenGut::Keyword(Keyword::Do),
    TokenGut::Keyword(Keyword::Continue),
    TokenGut::Keyword(Keyword::Break),
    TokenGut::Keyword(Keyword::Return),
    TokenGut::Keyword(Keyword::Try),
    TokenGut::Keyword(Keyword::Throw),
    TokenGut::Semicolon,
    TokenGut::Paren(Paren::CurlyClose),
    TokenGut::EOF,
];

impl Parser {
    pub fn parse_sequential_stmts(&mut self) -> Parsed<Vec<ParsedBox<SequentialStmt>>> {
        self.parse_star(|c_self| {
            let token = c_self.current_token()?;
            if token.t_gut != TokenGut::Paren(Paren::CurlyClose) && token.t_gut != TokenGut::EOF {
                let p_stmt = c_self.parse_stmt();
                return Some(p_stmt);
            }
            return None;
        })
    }

    pub fn parse_stmt(&mut self) -> Parsed<SequentialStmt> {
        let token = self.current_token().unwrap();
        match token.t_gut {
            TokenGut::Keyword(Keyword::Let) => self.parse_let_stmt_sequential(),
            TokenGut::Keyword(Keyword::If) => match self
                .next_token()
                .unwrap_or(self.current_token().unwrap())
                .t_gut
            {
                TokenGut::Keyword(Keyword::Let) => self.parse_if_let_stmt_sequential(),
                _ => self.parse_if_stmt_sequential(),
            },
            TokenGut::Keyword(Keyword::Match) => self.parse_match_stmt_sequential(),
            TokenGut::Keyword(Keyword::For) => self.parse_for_stmt_sequential(),
            TokenGut::Keyword(Keyword::While) => match self
                .next_token()
                .unwrap_or(self.current_token().unwrap())
                .t_gut
            {
                TokenGut::Keyword(Keyword::Let) => self.parse_while_let_stmt_sequential(),
                _ => self.parse_while_stmt_sequential(),
            },
            TokenGut::Keyword(Keyword::Do) => self.parse_do_while_stmt_sequential(),
            TokenGut::Keyword(Keyword::Continue) => self.parse_continue_stmt_sequential(),
            TokenGut::Keyword(Keyword::Break) => self.parse_break_stmt_sequential(),
            TokenGut::Keyword(Keyword::Return) => self.parse_return_stmt_sequential(),
            TokenGut::Keyword(Keyword::Try) => self.parse_error_handling_stmt_sequential(),
            TokenGut::Keyword(Keyword::Throw) => self.parse_throw_stmt_sequential(),
            TokenGut::Semicolon => self.parse_empty_stmt_sequential(),
            _ => self.chain_try_parses_with_default_parse(
                vec![Parser::try_parse_assignment_stmt_sequential],
                Parser::parse_expr_stmt_sequential,
            ),
        }
    }
}

impl Parser {
    pub fn parse_empty_stmt_sequential(&mut self) -> Parsed<SequentialStmt> {
        let p_sc = self.parse_semicolon();
        Parsed::lift_parsed(p_sc, |_| SequentialStmt::Empty)
    }

    fn try_parse_assignment_stmt_sequential(&mut self) -> Option<Parsed<SequentialStmt>> {
        self.safe_try(|s_self| {
            let p_stmt = s_self.try_parse_assignment_stmt()?;
            Some(Parsed::lift_parsed(Parsed::box_parsed(p_stmt), |stmt| {
                SequentialStmt::Assignment(stmt)
            }))
        })
    }

    fn parse_throw_stmt_sequential(&mut self) -> Parsed<SequentialStmt> {
        Parsed::lift_parsed(Parsed::box_parsed(self.parse_throw_stmt()), |stmt| {
            SequentialStmt::Throw(stmt)
        })
    }

    fn parse_error_handling_stmt_sequential(&mut self) -> Parsed<SequentialStmt> {
        Parsed::lift_parsed(
            Parsed::box_parsed(self.parse_error_handling_stmt()),
            |stmt| SequentialStmt::ErrorHandling(stmt),
        )
    }

    fn parse_expr_stmt_sequential(&mut self) -> Parsed<SequentialStmt> {
        Parsed::lift_parsed(Parsed::box_parsed(self.parse_expr_stmt()), |stmt| {
            SequentialStmt::Expr(stmt)
        })
    }

    fn parse_for_stmt_sequential(&mut self) -> Parsed<SequentialStmt> {
        Parsed::lift_parsed(Parsed::box_parsed(self.parse_for_stmt()), |stmt| {
            SequentialStmt::For(stmt)
        })
    }

    fn parse_if_let_stmt_sequential(&mut self) -> Parsed<SequentialStmt> {
        Parsed::lift_parsed(Parsed::box_parsed(self.parse_if_let_stmt()), |stmt| {
            SequentialStmt::IfLet(stmt)
        })
    }

    fn parse_if_stmt_sequential(&mut self) -> Parsed<SequentialStmt> {
        Parsed::lift_parsed(Parsed::box_parsed(self.parse_if_stmt()), |stmt| {
            SequentialStmt::If(stmt)
        })
    }

    fn parse_let_stmt_sequential(&mut self) -> Parsed<SequentialStmt> {
        Parsed::lift_parsed(Parsed::box_parsed(self.parse_let_stmt()), |stmt| {
            SequentialStmt::Let(stmt)
        })
    }

    fn parse_match_stmt_sequential(&mut self) -> Parsed<SequentialStmt> {
        Parsed::lift_parsed(Parsed::box_parsed(self.parse_match_stmt()), |stmt| {
            SequentialStmt::Match(stmt)
        })
    }

    fn parse_while_stmt_sequential(&mut self) -> Parsed<SequentialStmt> {
        Parsed::lift_parsed(Parsed::box_parsed(self.parse_while_stmt()), |stmt| {
            SequentialStmt::While(stmt)
        })
    }

    fn parse_do_while_stmt_sequential(&mut self) -> Parsed<SequentialStmt> {
        Parsed::lift_parsed(Parsed::box_parsed(self.parse_do_while_stmt()), |stmt| {
            SequentialStmt::DoWhile(stmt)
        })
    }

    fn parse_while_let_stmt_sequential(&mut self) -> Parsed<SequentialStmt> {
        Parsed::lift_parsed(Parsed::box_parsed(self.parse_while_let_stmt()), |stmt| {
            SequentialStmt::WhileLet(stmt)
        })
    }

    fn parse_continue_stmt_sequential(&mut self) -> Parsed<SequentialStmt> {
        Parsed::lift_parsed(Parsed::box_parsed(self.parse_continue_stmt()), |stmt| {
            SequentialStmt::Continue(stmt)
        })
    }

    fn parse_break_stmt_sequential(&mut self) -> Parsed<SequentialStmt> {
        Parsed::lift_parsed(Parsed::box_parsed(self.parse_break_stmt()), |stmt| {
            SequentialStmt::Break(stmt)
        })
    }

    fn parse_return_stmt_sequential(&mut self) -> Parsed<SequentialStmt> {
        Parsed::lift_parsed(Parsed::box_parsed(self.parse_return_stmt()), |stmt| {
            SequentialStmt::Return(stmt)
        })
    }
}

impl Parser {
    pub fn parse_sequential_block(&mut self) -> Parsed<SequentialBlock> {
        self.force_parse_with_default(
            SequentialBlock { stmts: vec![] },
            Self::try_parse_sequential_block,
            ParseError::expected_statement_not_found,
            vec![TokenGut::Paren(Paren::CurlyClose), EOF],
        )
    }

    pub fn try_parse_sequential_block(&mut self) -> Option<Parsed<SequentialBlock>> {
        self.safe_try(|s_self| {
            let p_curly_o = s_self.try_parse_paren(Paren::CurlyOpen)?;
            let p_stmts = s_self.parse_sequential_stmts();
            let p_curly_c = s_self.parse_paren(Paren::CurlyClose);
            Some(Parsed::lift_parsed(
                Parsed::merge_parsed_ignore_left_right(p_curly_o, p_stmts, p_curly_c),
                |stmts| SequentialBlock { stmts },
            ))
        })
    }
}
