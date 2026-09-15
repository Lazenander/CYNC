use crate::compiler::lexer::tokens::{Operator, Paren, TokenGut};
use crate::compiler::parser::error::ParseError;
use crate::compiler::parser::exprs::identifier::Identifier;
use crate::compiler::parser::exprs::{Expr, ExprStructure, EXPR_SYNC_SET};
use crate::compiler::parser::parser::{Parsed, ParsedBox, Parser};

impl Parser {
    pub fn try_parse_arg1(&mut self) -> Option<Parsed<Vec<ParsedBox<Expr>>>> {
        self.safe_try(|s_self| {
            let mut p_args1: Vec<Parsed<ParsedBox<Expr>>> = vec![];
            if let Some(p_arg1) = s_self.try_parse_expr() {
                p_args1.push(Parsed::box_parsed(p_arg1));
                let p_p_args1 = s_self.parse_star(|c_self| {
                    let p_c = c_self.try_parse_comma()?;
                    let p_arg1 = c_self.try_parse_expr()?;
                    if let Some(_) =
                        c_self.peek_try(|cc_self| cc_self.try_parse_operator(Operator::Assign))
                    {
                        None
                    } else {
                        Some(Parsed::merge_parsed_ignore_left(p_c, p_arg1))
                    }
                });
                Some(Parsed::merge_parsed(
                    s_self.push_vec(p_args1),
                    p_p_args1,
                    |args1_1, args1_2| {
                        args1_1.clone().extend(args1_2);
                        args1_1
                    },
                ))
            } else {
                Some(s_self.new_value(vec![]))
            }
        })
    }

    pub fn try_parse_arg2(
        &mut self,
    ) -> Option<Parsed<Vec<ParsedBox<(ParsedBox<Identifier>, ParsedBox<Expr>)>>>> {
        self.safe_try(|s_self| {
            let mut p_args2: Vec<Parsed<ParsedBox<(ParsedBox<Identifier>, ParsedBox<Expr>)>>> =
                vec![];
            if let Some(p_id) = s_self.try_parse_identifier() {
                let p_id = Parsed::box_parsed(p_id);
                let Some(p_assign) = s_self.try_parse_operator(Operator::Assign) else {
                    return Some(s_self.new_value(vec![]));
                };
                let Some(p_arg2) = s_self.try_parse_expr() else {
                    return Some(s_self.new_value(vec![]));
                };
                let p_arg2 = Parsed::box_parsed(p_arg2);
                p_args2.push(Parsed::merge_parsed(
                    p_id,
                    Parsed::merge_parsed_ignore_left(p_assign, p_arg2),
                    |id, arg2| s_self.new_parsed_box((id, arg2)),
                ));
                let p_p_args2 = s_self.parse_star(|c_self| {
                    let p_c = c_self.try_parse_comma()?;
                    let p_id = Parsed::box_parsed(c_self.try_parse_identifier()?);
                    let p_assign = c_self.try_parse_operator(Operator::Assign)?;
                    let p_arg2 = Parsed::box_parsed(c_self.try_parse_expr()?);
                    Some(Parsed::merge_parsed(
                        Parsed::merge_parsed_ignore_left(p_c, p_id),
                        Parsed::merge_parsed_ignore_left(p_assign, p_arg2),
                        |id, arg2| ((id, arg2)),
                    ))
                });
                Some(Parsed::merge_parsed(
                    s_self.push_vec(p_args2),
                    p_p_args2,
                    |args2_1, args2_2| {
                        args2_1.clone().extend(args2_2);
                        args2_1
                    },
                ))
            } else {
                Some(s_self.new_value(vec![]))
            }
        })
    }

    pub fn parse_function_call_args(
        &mut self,
    ) -> Parsed<(
        Vec<ParsedBox<Expr>>,
        Vec<ParsedBox<(ParsedBox<Identifier>, ParsedBox<Expr>)>>,
    )> {
        self.force_parse_with_default(
            (vec![], vec![]),
            Parser::try_parse_function_call_args,
            ParseError::expected_expression_not_found,
            vec![
                TokenGut::Paren(Paren::RoundClose),
                TokenGut::Operator(Operator::Comma),
                TokenGut::Paren(Paren::CurlyClose),
                TokenGut::EOF,
            ],
        )
    }

    pub fn try_parse_function_call_args(
        &mut self,
    ) -> Option<
        Parsed<(
            Vec<ParsedBox<Expr>>,
            Vec<ParsedBox<(ParsedBox<Identifier>, ParsedBox<Expr>)>>,
        )>,
    > {
        self.safe_try(|s_self| {
            let p_p_ro = s_self.try_parse_paren(Paren::RoundOpen)?;
            let p_args1 = s_self.try_parse_arg1()?;
            let mut p_args2: Parsed<Vec<ParsedBox<(ParsedBox<Identifier>, ParsedBox<Expr>)>>> =
                s_self.new_value(vec![]);
            if p_args1.value.is_empty() {
                if let Some(s_p_args2) = s_self.try_parse_arg2() {
                    p_args2 = s_p_args2;
                }
            } else {
                if let Some(p_c) = s_self.try_parse_operator(Operator::Comma) {
                    if let Some(s_p_args2) = s_self.try_parse_arg2() {
                        p_args2 = Parsed::merge_parsed_ignore_left(p_c, s_p_args2);
                    }
                }
            }
            let p_p_rc = s_self.parse_paren(Paren::RoundClose);
            Some(Parsed::merge_parsed(
                Parsed::merge_parsed_ignore_left(p_p_ro, p_args1),
                Parsed::merge_parsed_ignore_right(p_args2, p_p_rc),
                |args1, args2| (args1, args2),
            ))
        })
    }
}
