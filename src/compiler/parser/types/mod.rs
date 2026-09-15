pub mod parenedtype;
pub mod squaredtype;
pub mod structtype;
pub mod templated;
pub mod unittype;

use crate::compiler::lexer::tokens::{Operator, Paren, TokenGut};
use crate::compiler::parser::error::ParseError;
use crate::compiler::parser::parser::{Parsed, ParsedBox, Parser};
use crate::compiler::parser::types::parenedtype::TupleType;
use crate::compiler::parser::types::structtype::StructType;
use crate::compiler::parser::types::templated::IdentifierType;

#[derive(Debug, Clone)]
pub enum TypeAnnotation {
    Covariant,
    Contravariant,
    Invariant,
}

impl Parser {
    pub fn parse_type_annotation(&mut self) -> Parsed<TypeAnnotation> {
        if let Some(p_add) = self.try_parse_operator(Operator::Add) {
            Parsed::lift_parsed(p_add, |_| TypeAnnotation::Covariant)
        } else if let Some(p_sub) = self.try_parse_operator(Operator::Sub) {
            Parsed::lift_parsed(p_sub, |_| TypeAnnotation::Contravariant)
        } else if let Some(p_zero) = self.try_parse_zero() {
            Parsed::lift_parsed(p_zero, |_| TypeAnnotation::Invariant)
        } else {
            self.new_value(TypeAnnotation::Covariant)
        }
    }
}

#[derive(Debug, Clone)]
pub enum TypeGut {
    Unit,
    Identifier(IdentifierType),
    ParenSingle(ParsedBox<Type>),
    Tuple(TupleType),
    Array(ParsedBox<Type>),
    Map(ParsedBox<Type>, ParsedBox<Type>),
    Struct(StructType),
    Function(Vec<ParsedBox<Type>>, ParsedBox<Type>),
    Union(ParsedBox<Type>, ParsedBox<Type>),
    Intersection(ParsedBox<Type>, ParsedBox<Type>),
}

#[derive(Debug, Clone)]
pub struct Type {
    pub annotation: TypeAnnotation,
    pub gut: TypeGut,
}

impl Parser {
    pub fn parse_type_with_bp(&mut self, min_bp: u8) -> Parsed<Type> {
        self.force_parse_with_default(
            Type {
                annotation: TypeAnnotation::Invariant,
                gut: TypeGut::Unit,
            },
            |c_self| c_self.try_parse_type_with_bp(min_bp),
            ParseError::expected_type_not_found,
            vec![],
        )
    }

    pub fn try_parse_type_with_bp(&mut self, min_bp: u8) -> Option<Parsed<Type>> {
        self.safe_try(|s_self| {
            let p_annotation = s_self.parse_type_annotation();
            let p_type: Parsed<Type>;
            let token = s_self.current_token()?;
            p_type = Parsed::merge_parsed(
                p_annotation,
                match token.t_gut {
                    TokenGut::Identifier(_) => s_self.parse_identifier_prefix_type_gut(),
                    TokenGut::Paren(Paren::RoundOpen) => s_self.parse_parened_type_gut(),
                    TokenGut::Paren(Paren::SquareOpen) => s_self.parse_squared_type_gut(),
                    TokenGut::Paren(Paren::CurlyOpen) => s_self.parse_curlyed_type_gut(),
                    _ => return None,
                },
                |annotation, gut| Type { annotation, gut },
            );
            let mut p_lhs: Parsed<Type> = p_type;
            while let Some(p_op) = s_self.peek_try(|p_self| {
                p_self.try_parse_between_operators(vec![
                    Operator::BitOr,
                    Operator::BitAnd,
                    Operator::Arrow,
                ])
            }) {
                let (op_l_bp, op_r_bp) = infix_op_bp(p_op.clone().value);
                if op_l_bp < min_bp {
                    break;
                }
                let p_op = s_self
                    .try_parse_between_operators(vec![
                        Operator::BitOr,
                        Operator::BitAnd,
                        Operator::Arrow,
                    ])
                    .unwrap_or_else(|| unreachable!());
                let p_rhs = s_self.parse_type_with_bp(op_r_bp);
                p_lhs = Parsed::merge_parsed_op(p_lhs, p_op, p_rhs, |lhs, op, rhs| match op {
                    Operator::BitOr => Type {
                        annotation: TypeAnnotation::Covariant,
                        gut: TypeGut::Union(s_self.new_parsed_box(lhs), s_self.new_parsed_box(rhs)),
                    },
                    Operator::BitAnd => Type {
                        annotation: TypeAnnotation::Covariant,
                        gut: TypeGut::Intersection(
                            s_self.new_parsed_box(lhs),
                            s_self.new_parsed_box(rhs),
                        ),
                    },
                    Operator::Arrow => Type {
                        annotation: lhs.annotation,
                        gut: match lhs.gut {
                            TypeGut::Unit => TypeGut::Function(vec![], s_self.new_parsed_box(rhs)),
                            TypeGut::Tuple(types) => {
                                TypeGut::Function(types.entries, s_self.new_parsed_box(rhs))
                            }
                            TypeGut::ParenSingle(paran_type) => {
                                TypeGut::Function(vec![paran_type], s_self.new_parsed_box(rhs))
                            }
                            _ => {
                                unreachable!()
                            }
                        },
                    },
                    _ => {
                        unreachable!()
                    }
                });
            }
            Some(p_lhs)
        })
    }

    pub fn parse_type(&mut self) -> Parsed<Type> {
        self.force_parse_with_default(
            Type {
                annotation: TypeAnnotation::Invariant,
                gut: TypeGut::Unit,
            },
            |c_self| c_self.try_parse_type(),
            ParseError::expected_type_not_found,
            vec![],
        )
    }

    pub fn try_parse_type(&mut self) -> Option<Parsed<Type>> {
        self.try_parse_type_with_bp(0)
    }
}

/*
fn prefix_op_bp(op: Operator) -> u8 {
    match op {
        Operator::Exclamation => 0b10001,
        _ => unreachable!()
    }
}
*/

fn infix_op_bp(op: Operator) -> (u8, u8) {
    match op {
        Operator::BitOr => (0b00010, 0b00011),
        Operator::BitAnd => (0b00100, 0b00101),
        Operator::Arrow => (0b01001, 0b01000),
        _ => unreachable!(),
    }
}
