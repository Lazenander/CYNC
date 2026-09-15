use crate::compiler::lexer::tokens::{
    Keyword, Literal, Operator, Paren, Position, Token, TokenGut,
};
use crate::compiler::parser::error::{ParseError, ParseErrors};
use crate::compiler::parser::parser::{Parsed, ParsedBox, ParsedPosition, Parser};
use crate::utility::vpe::VPE;

impl Parser {
    pub fn parse_keyword(&mut self, expected: Keyword) -> Parsed<Keyword> {
        self.force_parse_with_default(
            expected.clone(),
            |c_self| c_self.try_parse_keyword(expected.clone()),
            |found| {
                ParseError::expected_token_not_found(TokenGut::Keyword(expected.clone()), found)
            },
            vec![],
        )
    }

    pub fn try_parse_keyword(&mut self, expected: Keyword) -> Option<Parsed<Keyword>> {
        self.safe_try(|s_self| {
            let token = s_self.current_token()?;
            let TokenGut::Keyword(k) = token.t_gut else {
                return None;
            };
            if k != expected {
                return None;
            }
            s_self.next();
            Some(s_self.new_value(k))
        })
    }

    pub fn parse_operator(&mut self, expected: Operator) -> Parsed<Operator> {
        self.force_parse_with_default(
            expected.clone(),
            |c_self| c_self.try_parse_operator(expected.clone()),
            |found| {
                ParseError::expected_token_not_found(TokenGut::Operator(expected.clone()), found)
            },
            vec![],
        )
    }

    pub fn try_parse_operator(&mut self, expected: Operator) -> Option<Parsed<Operator>> {
        self.safe_try(|s_self| {
            let token = s_self.current_token()?;
            let TokenGut::Operator(op) = token.t_gut else {
                return None;
            };
            match expected {
                Operator::Increment => {
                    let _ = s_self.try_parse_operator(Operator::Add)?;
                    let _ = s_self.try_parse_operator(Operator::Add)?;
                    Some(s_self.new_value(Operator::Increment))
                }
                Operator::Decrement => {
                    let _ = s_self.try_parse_operator(Operator::Sub)?;
                    let _ = s_self.try_parse_operator(Operator::Sub)?;
                    Some(s_self.new_value(Operator::Decrement))
                }
                Operator::BitShiftRight => {
                    let _ = s_self.try_parse_operator(Operator::Gt)?;
                    let _ = s_self.try_parse_operator(Operator::Gt)?;
                    Some(s_self.new_value(Operator::BitShiftRight))
                }
                Operator::Gte => {
                    let _ = s_self.try_parse_operator(Operator::Gt)?;
                    let _ = s_self.try_parse_operator(Operator::Assign)?;
                    Some(s_self.new_value(Operator::Gte))
                }
                Operator::BitShiftRightAssign => {
                    let _ = s_self.try_parse_operator(Operator::Gt)?;
                    let _ = s_self.try_parse_operator(Operator::Gt)?;
                    let _ = s_self.try_parse_operator(Operator::Assign)?;
                    Some(s_self.new_value(Operator::BitShiftRightAssign))
                }
                _ => {
                    if op != expected {
                        return None;
                    }
                    s_self.next();
                    Some(s_self.new_value(op))
                }
            }
        })
    }

    pub fn try_parse_zero(&mut self) -> Option<Parsed<()>> {
        self.safe_try(|s_self| {
            let token = s_self.current_token()?;
            let TokenGut::Literal(Literal::Integer(0)) = token.t_gut else {
                return None;
            };
            s_self.next();
            Some(s_self.new_value(()))
        })
    }

    pub fn parse_between_operators(&mut self, expecteds: Vec<Operator>) -> Parsed<Operator> {
        self.force_parse_with_default(
            expecteds.first().unwrap().clone(),
            |c_self| c_self.try_parse_between_operators(expecteds.clone()),
            |found| {
                ParseError::expected_token_not_found(
                    TokenGut::Operator(expecteds.first().unwrap().clone()),
                    found,
                )
            },
            vec![],
        )
    }

    pub fn try_parse_between_operators(
        &mut self,
        expecteds: Vec<Operator>,
    ) -> Option<Parsed<Operator>> {
        self.safe_try(|s_self| {
            let prioritized_expecteds: Vec<Operator> = expecteds
                .clone()
                .into_iter()
                .filter(|op| {
                    op == &Operator::BitShiftRight
                        || op == &Operator::BitShiftRightAssign
                        || op == &Operator::Gte
                        || op == &Operator::Increment
                        || op == &Operator::Decrement
                })
                .collect();
            let less_prioritized_expecteds: Vec<Operator> = expecteds
                .clone()
                .into_iter()
                .filter(|op| {
                    op != &Operator::BitShiftRight
                        && op != &Operator::BitShiftRightAssign
                        && op != &Operator::Gte
                        && op != &Operator::Increment
                        && op != &Operator::Decrement
                })
                .collect();
            for expected in prioritized_expecteds {
                let o_p = s_self.try_parse_operator(expected);
                if let Some(p) = o_p {
                    return Some(p);
                }
            }
            for expected in less_prioritized_expecteds {
                let o_p = s_self.try_parse_operator(expected);
                if let Some(p) = o_p {
                    return Some(p);
                }
            }
            return None;
        })
    }

    pub fn parse_paren(&mut self, expected: Paren) -> Parsed<Paren> {
        self.force_parse_with_default(
            expected.clone(),
            |c_self| c_self.try_parse_paren(expected.clone()),
            |found| ParseError::expected_token_not_found(TokenGut::Paren(expected.clone()), found),
            vec![],
        )
    }

    pub fn try_parse_paren(&mut self, expected: Paren) -> Option<Parsed<Paren>> {
        self.safe_try(|s_self| {
            let token = s_self.current_token()?;
            let TokenGut::Paren(paren) = token.t_gut else {
                return None;
            };
            if paren != expected {
                return None;
            }
            s_self.next();
            Some(s_self.new_value(paren))
        })
    }

    pub fn parse_semicolon(&mut self) -> Parsed<()> {
        self.force_parse_with_default(
            (),
            Parser::try_parse_semicolon,
            |found| ParseError::expected_semicolon_not_found(found.position),
            vec![
                TokenGut::Semicolon,
                TokenGut::Paren(Paren::CurlyClose),
                TokenGut::EOF,
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
            ],
        )
    }

    pub fn try_parse_semicolon(&mut self) -> Option<Parsed<()>> {
        self.safe_try(|s_self| {
            let token = s_self.current_token()?;
            let TokenGut::Semicolon = token.t_gut else {
                return None;
            };
            s_self.next();
            Some(s_self.new_value(()))
        })
    }

    pub fn parse_comma(&mut self) -> Parsed<()> {
        self.force_parse_with_default(
            (),
            Parser::try_parse_comma,
            |found| ParseError::expected_comma_not_found(found.position),
            vec![],
        )
    }

    pub fn try_parse_comma(&mut self) -> Option<Parsed<()>> {
        self.safe_try(|s_self| {
            let token = s_self.current_token()?;
            let TokenGut::Operator(Operator::Comma) = token.t_gut else {
                return None;
            };
            s_self.next();
            Some(s_self.new_value(()))
        })
    }

    pub fn parse_eof(&mut self) -> Parsed<()> {
        self.force_parse_with_default(
            (),
            Parser::try_parse_eof,
            |found| ParseError::expected_eof_not_found(found.position),
            vec![],
        )
    }

    pub fn try_parse_eof(&mut self) -> Option<Parsed<()>> {
        self.safe_try(|s_self| {
            let token = s_self.current_token()?;
            let TokenGut::EOF = token.t_gut else {
                return None;
            };
            s_self.next();
            Some(s_self.new_value(()))
        })
    }
}
