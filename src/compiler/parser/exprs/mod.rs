mod array;
mod constructexpr;
pub mod function_call;
pub mod function_def;
pub mod identifier;
mod ifthenelse;
pub mod literal;
mod map;
mod parenedexpr;
mod r#struct;
pub mod templated;

use crate::compiler::lexer::tokens::{Keyword, Operator, Paren, Position, TokenGut};
use crate::compiler::parser::patterns::Pattern;
use crate::compiler::parser::types::{Type, TypeAnnotation, TypeGut};

use crate::compiler::parser::error::ParseError;
use crate::compiler::parser::exprs::identifier::{Identifier, RoutedIdentifier};
use crate::compiler::parser::exprs::literal::Literal;
use crate::compiler::parser::exprs::templated::TemplateCallRoutedIdentifier;
use crate::compiler::parser::parser::{Parsed, ParsedBox, Parser};
use crate::compiler::parser::template::TemplateCall;
pub use function_def::{FunctionBlock, FunctionDef};

#[derive(Debug, Clone)]
pub struct Expr {
    e_stt: ExprStructure,
}

impl Expr {
    pub fn new(e_stt: ExprStructure) -> Self {
        Expr { e_stt }
    }
}

#[derive(Debug, Clone)]
pub enum ExprStructure {
    Literal(ParsedBox<Literal>),
    Identifier(ParsedBox<RoutedIdentifier>),
    Templated(ParsedBox<TemplateCallRoutedIdentifier>),
    ArrayIndexExpr {
        array: ParsedBox<Expr>,
        index: ParsedBox<Expr>,
    },
    FunctionDef(ParsedBox<FunctionDef>),
    FunctionCall {
        function: ParsedBox<Expr>,
        args1: Vec<ParsedBox<Expr>>,
        args2: Vec<ParsedBox<(ParsedBox<Identifier>, ParsedBox<Expr>)>>,
    },
    ConstructExpr {
        name: ParsedBox<RoutedIdentifier>,
        elements: Vec<ParsedBox<(ParsedBox<Identifier>, ParsedBox<Expr>)>>,
    },
    PrefixMonop(Operator, ParsedBox<Expr>),
    PostfixMonop(ParsedBox<Expr>, Operator),
    Binop(ParsedBox<Expr>, Operator, ParsedBox<Expr>),
    IfThenElse {
        condition: ParsedBox<Expr>,
        then_expr: ParsedBox<Expr>,
        else_expr: ParsedBox<Expr>,
    },
    IsType(ParsedBox<Expr>, ParsedBox<Type>),
    CastType(ParsedBox<Expr>, ParsedBox<Type>),
    TupleExpr {
        exprs: Vec<ParsedBox<Expr>>,
    },
    ArrayExpr {
        exprs: Vec<ParsedBox<Expr>>,
    },
    MapExpr {
        exprs: Vec<ParsedBox<(Expr, Expr)>>,
    },
    StructExpr {
        elements: Vec<ParsedBox<(ParsedBox<Identifier>, ParsedBox<Expr>)>>,
    },
}

const EXPR_SYNC_SET: [TokenGut; 71] = [
    TokenGut::Keyword(Keyword::Module),
    TokenGut::Keyword(Keyword::Shape),
    TokenGut::Keyword(Keyword::Operation),
    TokenGut::Keyword(Keyword::BindDef),
    TokenGut::Keyword(Keyword::Infix),
    TokenGut::Keyword(Keyword::Prefix),
    TokenGut::Keyword(Keyword::Postfix),
    TokenGut::Keyword(Keyword::Coerce),
    TokenGut::Keyword(Keyword::Export),
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
    TokenGut::Paren(Paren::RoundOpen),
    TokenGut::Paren(Paren::RoundClose),
    TokenGut::Paren(Paren::SquareOpen),
    TokenGut::Paren(Paren::SquareClose),
    TokenGut::Paren(Paren::CurlyOpen),
    TokenGut::Paren(Paren::CurlyClose),
    TokenGut::EOF,
    TokenGut::Operator(Operator::Add),
    TokenGut::Operator(Operator::Sub),
    TokenGut::Operator(Operator::Mul),
    TokenGut::Operator(Operator::Div),
    TokenGut::Operator(Operator::Mod),
    TokenGut::Operator(Operator::Pow),
    TokenGut::Operator(Operator::IntDiv),
    TokenGut::Operator(Operator::BitAnd),
    TokenGut::Operator(Operator::BitOr),
    TokenGut::Operator(Operator::BitXor),
    TokenGut::Operator(Operator::BitNot),
    TokenGut::Operator(Operator::BitShiftLeft),
    TokenGut::Operator(Operator::BitShiftRight),
    TokenGut::Operator(Operator::Assign),
    TokenGut::Operator(Operator::Eq),
    TokenGut::Operator(Operator::Teq),
    TokenGut::Operator(Operator::Neq),
    TokenGut::Operator(Operator::Gt),
    TokenGut::Operator(Operator::Lt),
    TokenGut::Operator(Operator::Gte),
    TokenGut::Operator(Operator::Lte),
    TokenGut::Operator(Operator::And),
    TokenGut::Operator(Operator::Or),
    TokenGut::Operator(Operator::Question),
    TokenGut::Operator(Operator::Exclamation),
    TokenGut::Operator(Operator::Colon),
    TokenGut::Operator(Operator::DoubleColon),
    TokenGut::Operator(Operator::Dot),
    TokenGut::Operator(Operator::Arrow),
    TokenGut::Operator(Operator::DoubleArrow),
    TokenGut::Operator(Operator::AddAssign),
    TokenGut::Operator(Operator::SubAssign),
    TokenGut::Operator(Operator::MulAssign),
    TokenGut::Operator(Operator::DivAssign),
    TokenGut::Operator(Operator::ModAssign),
    TokenGut::Operator(Operator::BitAndAssign),
    TokenGut::Operator(Operator::BitOrAssign),
    TokenGut::Operator(Operator::BitXorAssign),
    TokenGut::Operator(Operator::BitShiftLeftAssign),
    TokenGut::Operator(Operator::BitShiftRightAssign),
    TokenGut::Operator(Operator::Increment),
    TokenGut::Operator(Operator::Decrement),
    TokenGut::Operator(Operator::Comma),
];

impl Parser {
    pub fn default_expr(&self) -> Expr {
        Expr::new(ExprStructure::Literal(self.new_parsed_box(Literal::Unit)))
    }

    pub fn parse_expr(&mut self) -> Parsed<Expr> {
        self.force_parse_with_default(
            Expr::new(ExprStructure::Identifier(
                self.new_parsed_box(self.default_routed_identifier()),
            )),
            Parser::try_parse_expr,
            ParseError::expected_expression_not_found,
            vec![
                TokenGut::Semicolon,
                TokenGut::Paren(Paren::RoundClose),
                TokenGut::Operator(Operator::Comma),
            ],
        )
    }

    pub fn parse_expr_with_bp(&mut self, min_bp: u16) -> Parsed<Expr> {
        self.force_parse_with_default(
            Expr::new(ExprStructure::Literal(self.new_parsed_box(Literal::Unit))),
            |c_self| c_self.try_parse_expr_with_bp(min_bp),
            ParseError::expected_type_not_found,
            EXPR_SYNC_SET.into(),
        )
    }

    pub fn parse_cast_type(&mut self) -> Parsed<Type> {
        self.force_parse_with_default(
            Type {
                annotation: TypeAnnotation::Covariant,
                gut: TypeGut::Unit,
            },
            Parser::try_parse_cast_type,
            ParseError::expected_expression_not_found,
            EXPR_SYNC_SET.into(),
        )
    }

    pub fn try_parse_cast_type(&mut self) -> Option<Parsed<Type>> {
        self.safe_try(|s_self| {
            let p_is = s_self.try_parse_operator(Operator::Colon)?;
            let p_type = s_self.try_parse_type()?;
            Some(Parsed::merge_parsed_ignore_left(p_is, p_type))
        })
    }

    pub fn try_parse_expr_with_bp(&mut self, min_bp: u16) -> Option<Parsed<Expr>> {
        self.safe_try(|s_self| {
            let mut o_p_expr: Option<Parsed<Expr>> = None;
            let token = s_self.current_token()?;
            match token.t_gut {
                TokenGut::Literal(_) => o_p_expr = Some(s_self.parse_literal_expr()),
                TokenGut::Identifier(_) => {
                    o_p_expr = Some(s_self.chain_try_parses_with_default_parse(
                        vec![Parser::try_parse_templated_expr],
                        Parser::parse_identifier_expr,
                    ))
                }
                TokenGut::Keyword(Keyword::If) => o_p_expr = Some(s_self.parse_if_then_else_expr()),
                TokenGut::Paren(Paren::RoundOpen) => {
                    o_p_expr = Some(s_self.chain_try_parses_with_default_parse(
                        vec![Parser::try_parse_function_def_expr],
                        Parser::parse_parened_expr,
                    ))
                }
                TokenGut::Paren(Paren::SquareOpen) => {
                    o_p_expr = Some(s_self.chain_try_parses_with_default_parse(
                        vec![Parser::try_parse_map_expr],
                        Parser::parse_array_expr,
                    ))
                }
                TokenGut::Paren(Paren::CurlyOpen) => o_p_expr = s_self.try_parse_structs_expr(),
                TokenGut::Operator(Operator::Increment)
                | TokenGut::Operator(Operator::Decrement)
                | TokenGut::Operator(Operator::Add)
                | TokenGut::Operator(Operator::Sub)
                | TokenGut::Operator(Operator::Mul)
                | TokenGut::Operator(Operator::BitAnd)
                | TokenGut::Operator(Operator::Exclamation)
                | TokenGut::Operator(Operator::Question)
                | TokenGut::Operator(Operator::BitNot) => {
                    let o_p_op = s_self.try_parse_between_operators(vec![
                        Operator::Increment,
                        Operator::Decrement,
                        Operator::Add,
                        Operator::Sub,
                        Operator::Mul,
                        Operator::BitAnd,
                        Operator::Question,
                        Operator::Exclamation,
                        Operator::BitNot,
                    ]);
                    if let Some(p_op) = o_p_op.clone() {
                        let o_p_rhs =
                            s_self.try_parse_expr_with_bp(prefix_op_bp(p_op.clone().value));
                        if let Some(p_rhs) = o_p_rhs.clone() {
                            let p_rhs = Parsed::box_parsed(p_rhs);
                            o_p_expr = Some(Parsed::merge_parsed(p_op, p_rhs, |op, expr| {
                                Expr::new(ExprStructure::PrefixMonop(op, expr))
                            }))
                        }
                    } else {
                        o_p_expr = None
                    }
                }
                _ => o_p_expr = None,
            }
            let mut p_lhs: Parsed<Expr> = o_p_expr?;
            loop {
                let mut break_flag = true;

                if let Some(p_op) = s_self.peek_try(|p_self| {
                    p_self.try_parse_between_operators(vec![
                        Operator::Increment,
                        Operator::Decrement,
                        Operator::Question,
                        Operator::Exclamation,
                    ])
                }) {
                    let op_bp = postfix_op_bp(p_op.clone().value);
                    if op_bp < min_bp {
                        break;
                    }
                    let p_op = s_self
                        .try_parse_between_operators(vec![
                            Operator::Increment,
                            Operator::Decrement,
                            Operator::Question,
                            Operator::Exclamation,
                        ])
                        .unwrap_or_else(|| unreachable!());
                    p_lhs = Parsed::merge_parsed(p_lhs, p_op, |lhs, op| {
                        Expr::new(ExprStructure::PostfixMonop(s_self.new_parsed_box(lhs), op))
                    });
                    break_flag = false;
                }

                let token = s_self.current_token()?;
                match token.t_gut {
                    TokenGut::Paren(Paren::RoundOpen) => {
                        let p_func_call_args = s_self.parse_function_call_args();
                        p_lhs =
                            Parsed::merge_parsed(p_lhs, p_func_call_args, |lhs, (args1, args2)| {
                                Expr::new(ExprStructure::FunctionCall {
                                    function: s_self.new_parsed_box(lhs),
                                    args1,
                                    args2,
                                })
                            });
                        break_flag = false
                    }
                    TokenGut::Paren(Paren::SquareOpen) => {
                        let p_index = s_self.parse_array_index();
                        p_lhs = Parsed::merge_parsed(p_lhs, p_index, |lhs, index| {
                            Expr::new(ExprStructure::ArrayIndexExpr {
                                array: s_self.new_parsed_box(lhs),
                                index: s_self.new_parsed_box(index),
                            })
                        });
                        break_flag = false
                    }
                    TokenGut::Operator(Operator::Colon) => {
                        let p_type = s_self.parse_cast_type();
                        p_lhs = Parsed::merge_parsed(p_lhs, p_type, |lhs, t| {
                            Expr::new(ExprStructure::CastType(
                                s_self.new_parsed_box(lhs),
                                s_self.new_parsed_box(t),
                            ))
                        });
                        break_flag = false
                    }
                    _ => {}
                }

                if break_flag {
                    break;
                }
            }
            while let Some(p_op) = s_self.peek_try(|p_self| {
                p_self.try_parse_between_operators(vec![
                    Operator::Dot,
                    Operator::Exclamation,
                    Operator::Question,
                    Operator::Pow,
                    Operator::IntDiv,
                    Operator::Mul,
                    Operator::Div,
                    Operator::Mod,
                    Operator::Add,
                    Operator::Sub,
                    Operator::BitShiftLeft,
                    Operator::BitShiftRight,
                    Operator::Gt,
                    Operator::Gte,
                    Operator::Lt,
                    Operator::Lte,
                    Operator::Eq,
                    Operator::Teq,
                    Operator::Neq,
                    Operator::BitAnd,
                    Operator::BitXor,
                    Operator::BitOr,
                    Operator::And,
                    Operator::Or,
                ])
            }) {
                let (op_l_bp, op_r_bp) = infix_op_bp(p_op.clone().value);
                if op_l_bp < min_bp {
                    break;
                }
                let p_op = s_self
                    .try_parse_between_operators(vec![
                        Operator::Dot,
                        Operator::Exclamation,
                        Operator::Question,
                        Operator::Pow,
                        Operator::IntDiv,
                        Operator::Mul,
                        Operator::Div,
                        Operator::Mod,
                        Operator::Add,
                        Operator::Sub,
                        Operator::BitShiftLeft,
                        Operator::BitShiftRight,
                        Operator::Gt,
                        Operator::Gte,
                        Operator::Lt,
                        Operator::Lte,
                        Operator::Eq,
                        Operator::Teq,
                        Operator::Neq,
                        Operator::BitAnd,
                        Operator::BitXor,
                        Operator::BitOr,
                        Operator::And,
                        Operator::Or,
                    ])
                    .unwrap_or_else(|| unreachable!());
                let p_rhs = s_self.parse_expr_with_bp(op_r_bp);
                p_lhs = Parsed::merge_parsed_op(p_lhs, p_op, p_rhs, |lhs, op, rhs| {
                    Expr::new(ExprStructure::Binop(
                        s_self.new_parsed_box(lhs),
                        op,
                        s_self.new_parsed_box(rhs),
                    ))
                })
            }
            Some(p_lhs)
        })
    }

    pub fn try_parse_expr(&mut self) -> Option<Parsed<Expr>> {
        self.try_parse_expr_with_bp(0)
    }
}

fn postfix_op_bp(op: Operator) -> u16 {
    match op {
        Operator::Question | Operator::Exclamation => 0b0010000000000000,
        Operator::Increment | Operator::Decrement => 0b0001000000000001,
        _ => unreachable!(),
    }
}

fn prefix_op_bp(op: Operator) -> u16 {
    match op {
        Operator::Increment
        | Operator::Decrement
        | Operator::Add
        | Operator::Sub
        | Operator::Mul
        | Operator::BitAnd
        | Operator::Question
        | Operator::Exclamation
        | Operator::BitNot => 0b0001000000000000,
        _ => unreachable!(),
    }
}

fn infix_op_bp(op: Operator) -> (u16, u16) {
    match op {
        Operator::Dot => (0b1000000000000000, 0b1000000000000001),
        Operator::Exclamation | Operator::Question => (0b0010000000000000, 0b0010000000000001),
        Operator::Pow | Operator::IntDiv => (0b0000100000000000, 0b0000100000000001),
        Operator::Mul | Operator::Div | Operator::Mod => (0b0000010000000000, 0b0000010000000001),
        Operator::Add | Operator::Sub => (0b0000001000000000, 0b0000001000000001),
        Operator::BitShiftLeft | Operator::BitShiftRight => {
            (0b0000000100000000, 0b0000000100000001)
        }
        Operator::Gt | Operator::Gte | Operator::Lt | Operator::Lte => {
            (0b0000000010000000, 0b0000000010000001)
        }
        Operator::Eq | Operator::Teq | Operator::Neq => (0b0000000001000000, 0b0000000001000001),
        Operator::BitAnd => (0b0000000000100000, 0b0000000000100001),
        Operator::BitXor => (0b0000000000010000, 0b0000000000010001),
        Operator::BitOr => (0b0000000000001000, 0b0000000000001001),
        Operator::And => (0b0000000000000100, 0b0000000000000101),
        Operator::Or => (0b0000000000000010, 0b0000000000000011),
        // Operator::Comma                                             => (0b0000000000000000, 0b0000000000000001),
        _ => {
            print!("{:?}", op);
            unreachable!()
        }
    }
}
