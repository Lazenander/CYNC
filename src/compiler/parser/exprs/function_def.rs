use super::{identifier::Identifier, Expr, ExprStructure, EXPR_SYNC_SET};
use crate::compiler::lexer::tokens::Paren::CurlyOpen;
use crate::compiler::lexer::tokens::{Operator, Paren, TokenGut};
use crate::compiler::parser::error::ParseError;
use crate::compiler::parser::parser::{Parsed, ParsedBox, Parser};
use crate::compiler::parser::stmts::sequential_stmts::SequentialBlock;
use crate::compiler::parser::types::Type;

#[derive(Debug, Clone)]
pub enum FunctionBlock {
    SequentialBlock(Option<ParsedBox<Type>>, ParsedBox<SequentialBlock>),
    FunctionDef(ParsedBox<FunctionDef>),
}

#[derive(Debug, Clone)]
pub struct FunctionDef {
    pub args: Vec<
        ParsedBox<(
            ParsedBox<Identifier>,
            ParsedBox<Type>,
            Option<ParsedBox<Expr>>,
        )>,
    >,
    pub body: ParsedBox<FunctionBlock>,
}

impl Parser {
    fn try_parse_function_arg_def(
        &mut self,
    ) -> Option<
        Parsed<(
            ParsedBox<Identifier>,
            ParsedBox<Type>,
            Option<ParsedBox<Expr>>,
        )>,
    > {
        self.safe_try(|s_self| {
            let p_id = Parsed::box_parsed(s_self.try_parse_identifier()?);
            let p_c = s_self.try_parse_operator(Operator::Colon)?;
            let p_type = Parsed::box_parsed(s_self.try_parse_type()?);
            let o_p_assign = s_self.try_parse_operator(Operator::Assign);
            let o_p_default = s_self.try_parse_expr();
            if let Some(p_assign) = o_p_assign.clone() {
                if let Some(p_default) = o_p_default.clone() {
                    let p_default = Parsed::box_parsed(p_default);
                    return Some(Parsed::merge_parsed_3(
                        p_id,
                        Parsed::merge_parsed_ignore_left_right(p_c, p_type, p_assign),
                        p_default,
                        |id, t, default| (id, t, Some(default)),
                    ));
                }
            }
            if o_p_assign.is_none() && o_p_default.is_none() {
                return Some(Parsed::merge_parsed_ignore_middle(
                    p_id,
                    p_c,
                    p_type,
                    |id, t| (id, t, None),
                ));
            }
            return None;
        })
    }

    fn try_parse_typed_pure_function_block(&mut self) -> Option<Parsed<FunctionBlock>> {
        self.safe_try(|s_self| {
            if let Some(tok) = s_self.current_token() {
                if tok.t_gut == TokenGut::Paren(CurlyOpen) {
                    return None;
                }
            }
            let p_type = s_self.try_parse_type()?;
            let p_block = s_self.parse_sequential_block();
            Some(Parsed::merge_parsed(p_type, p_block, |t, block| {
                FunctionBlock::SequentialBlock(
                    Some(s_self.new_parsed_box(t)),
                    s_self.new_parsed_box(block),
                )
            }))
        })
    }

    fn parse_untyped_pure_function_block(&mut self) -> Parsed<FunctionBlock> {
        let p_block = self.parse_sequential_block();
        Parsed::lift_parsed(p_block, |block| {
            FunctionBlock::SequentialBlock(None, self.new_parsed_box(block))
        })
    }

    fn parse_pure_function_block(&mut self) -> Parsed<FunctionBlock> {
        self.chain_try_parses_with_default_parse(
            vec![Parser::try_parse_typed_pure_function_block],
            Parser::parse_untyped_pure_function_block,
        )
    }

    pub fn try_parse_non_pure_function_head(
        &mut self,
    ) -> Option<
        Parsed<
            Vec<
                ParsedBox<(
                    ParsedBox<Identifier>,
                    ParsedBox<Type>,
                    Option<ParsedBox<Expr>>,
                )>,
            >,
        >,
    > {
        self.safe_try(|s_self| {
            let p_p_ro = s_self.try_parse_paren(Paren::RoundOpen)?;
            let mut p_args = s_self.parse_star(|c_self| {
                let p_arg = c_self.try_parse_function_arg_def()?;
                let p_comma = c_self.try_parse_operator(Operator::Comma)?;
                Some(Parsed::merge_parsed_ignore_right(p_arg, p_comma))
            });
            let o_p_last_arg = s_self.try_parse_function_arg_def();
            p_args = match o_p_last_arg {
                Some(p_last_arg) => {
                    Parsed::merge_parsed_append(p_args, Parsed::box_parsed(p_last_arg))
                }
                None => p_args,
            };
            let p_p_rc = s_self.try_parse_paren(Paren::RoundClose)?;
            let p_arrow = s_self.try_parse_operator(Operator::Arrow)?;
            Some(Parsed::merge_parsed_ignore_right(
                Parsed::merge_parsed_ignore_left_right(p_p_ro, p_args, p_p_rc),
                p_arrow,
            ))
        })
    }

    pub fn parse_function_block(&mut self) -> Parsed<FunctionBlock> {
        let o_p_f_head = self.try_parse_non_pure_function_head();
        match o_p_f_head {
            Some(p_f_head) => {
                let p_f_block = Parsed::box_parsed(self.parse_function_block());
                Parsed::merge_parsed(p_f_head, p_f_block, |f_head, f_block| {
                    (FunctionBlock::FunctionDef(self.new_parsed_box(FunctionDef {
                        args: f_head,
                        body: f_block,
                    })))
                })
            }
            None => self.parse_pure_function_block(),
        }
    }

    pub fn parse_function_def(&mut self) -> Parsed<FunctionDef> {
        self.force_parse_with_default(
            FunctionDef {
                args: vec![],
                body: self.new_parsed_box(FunctionBlock::SequentialBlock(
                    None,
                    self.new_parsed_box(SequentialBlock { stmts: vec![] }),
                )),
            },
            Parser::try_parse_function_def,
            ParseError::expected_expression_not_found,
            EXPR_SYNC_SET.into(),
        )
    }

    pub fn try_parse_function_def(&mut self) -> Option<Parsed<FunctionDef>> {
        self.safe_try(|s_self| {
            let p_f_head = s_self.try_parse_non_pure_function_head()?;
            let p_f_block = s_self.parse_function_block();
            Some(Parsed::merge_parsed(
                p_f_head,
                p_f_block,
                |f_head, f_block| FunctionDef {
                    args: f_head,
                    body: s_self.new_parsed_box(f_block),
                },
            ))
        })
    }

    pub fn parse_function_def_expr(&mut self) -> Parsed<Expr> {
        self.force_parse_with_default(
            self.default_expr(),
            Parser::try_parse_function_def_expr,
            ParseError::expected_expression_not_found,
            EXPR_SYNC_SET.into(),
        )
    }

    pub fn try_parse_function_def_expr(&mut self) -> Option<Parsed<Expr>> {
        self.safe_try(|s_self| {
            let p_f_def = Parsed::box_parsed(s_self.try_parse_function_def()?);
            Some(Parsed::lift_parsed(p_f_def, |f_def| {
                Expr::new(ExprStructure::FunctionDef(f_def))
            }))
        })
    }
}
