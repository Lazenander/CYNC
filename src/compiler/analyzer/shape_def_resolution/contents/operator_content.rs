use crate::compiler::analyzer::common::route::route::Route;
use crate::compiler::analyzer::shape_def_resolution::contents::shape_content::ShapeContentHeadKey;
use crate::compiler::analyzer::shape_def_resolution::description::function_description::FunctionDescriptionContext;
use crate::compiler::analyzer::shape_def_resolution::description::shape_description::{
    ShapeDescriptionContext, ShapeDescriptionGut,
};
use crate::compiler::analyzer::shape_def_resolution::shape_signature::SpecifiedShapeDefSignature;
use crate::compiler::analyzer::shape_def_resolution::summary::{
    FunctionDescriptionContextKey, FunctionDescriptionKey, GenericParamDescriptionKey,
    GrandShapeContentSummaryAtWork, ShapeDescriptionContextKey,
};
use crate::compiler::lexer::tokens::Operator;
use crate::compiler::parser::parser::ParsedBox;
use crate::compiler::parser::stmts::definition_stmts::operator::{
    OperatorOverload, OperatorOverloadAnnotation,
};
use std::fmt;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum PrefixOperatorKind {
    Increment,
    Decrement,
    Add,
    Sub,
    Reference,
    Dereference,
    Question,
    Exclamation,
    BitNot,
}

impl From<Operator> for PrefixOperatorKind {
    fn from(operator: Operator) -> Self {
        match operator {
            Operator::Increment => PrefixOperatorKind::Increment,
            Operator::Decrement => PrefixOperatorKind::Decrement,
            Operator::Add => PrefixOperatorKind::Add,
            Operator::Sub => PrefixOperatorKind::Sub,
            Operator::Mul => PrefixOperatorKind::Reference,
            Operator::BitAnd => PrefixOperatorKind::Dereference,
            Operator::Question => PrefixOperatorKind::Question,
            Operator::Exclamation => PrefixOperatorKind::Exclamation,
            Operator::BitNot => PrefixOperatorKind::BitNot,
            _ => unreachable!(),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum InfixOperatorKind {
    Dot,
    Exclamation,
    Question,
    Pow,
    IntDiv,
    Mul,
    Div,
    Mod,
    Add,
    Sub,
    BitShiftLeft,
    BitShiftRight,
    Gt,
    Gte,
    Lt,
    Lte,
    Eq,
    Teq,
    Neq,
    BitAnd,
    BitXor,
    BitOr,
    And,
    Or,
}

impl From<Operator> for InfixOperatorKind {
    fn from(operator: Operator) -> Self {
        match operator {
            Operator::Dot => InfixOperatorKind::Dot,
            Operator::Exclamation => InfixOperatorKind::Exclamation,
            Operator::Question => InfixOperatorKind::Question,
            Operator::Pow => InfixOperatorKind::Pow,
            Operator::IntDiv => InfixOperatorKind::IntDiv,
            Operator::Mul => InfixOperatorKind::Mul,
            Operator::Div => InfixOperatorKind::Div,
            Operator::Mod => InfixOperatorKind::Mod,
            Operator::Add => InfixOperatorKind::Add,
            Operator::Sub => InfixOperatorKind::Sub,
            Operator::BitShiftLeft => InfixOperatorKind::BitShiftLeft,
            Operator::BitShiftRight => InfixOperatorKind::BitShiftRight,
            Operator::Gt => InfixOperatorKind::Gt,
            Operator::Gte => InfixOperatorKind::Gte,
            Operator::Lt => InfixOperatorKind::Lt,
            Operator::Lte => InfixOperatorKind::Lte,
            Operator::Eq => InfixOperatorKind::Eq,
            Operator::Teq => InfixOperatorKind::Teq,
            Operator::Neq => InfixOperatorKind::Neq,
            Operator::BitAnd => InfixOperatorKind::BitAnd,
            Operator::BitXor => InfixOperatorKind::BitXor,
            Operator::BitOr => InfixOperatorKind::BitOr,
            Operator::And => InfixOperatorKind::And,
            Operator::Or => InfixOperatorKind::Or,
            _ => unreachable!(),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum PostfixOperatorKind {
    Question,
    Exclamation,
    Increment,
    Decrement,
}

impl From<Operator> for PostfixOperatorKind {
    fn from(operator: Operator) -> Self {
        match operator {
            Operator::Question => PostfixOperatorKind::Question,
            Operator::Exclamation => PostfixOperatorKind::Exclamation,
            Operator::Increment => PostfixOperatorKind::Increment,
            Operator::Decrement => PostfixOperatorKind::Decrement,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum OperatorDefSignature {
    Prefix {
        prefix_op_kind: PrefixOperatorKind,
        operand: SpecifiedShapeDefSignature,
    },
    Infix {
        operand1: SpecifiedShapeDefSignature,
        infix_op_kind: InfixOperatorKind,
        operand2: SpecifiedShapeDefSignature,
    },
    Postfix {
        operand: SpecifiedShapeDefSignature,
        postfix_op_kind: PostfixOperatorKind,
    },
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum OperatorContentHeadKey {
    Prefix {
        prefix_op_kind: PrefixOperatorKind,
        operand: ShapeContentHeadKey,
    },
    Infix {
        operand1: ShapeContentHeadKey,
        infix_op_kind: InfixOperatorKind,
        operand2: ShapeContentHeadKey,
    },
    Postfix {
        operand: ShapeContentHeadKey,
        postfix_op_kind: PostfixOperatorKind,
    },
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct OperatorContent {
    pub signature: OperatorDefSignature,
    pub generic_params_key: Option<GenericParamDescriptionKey>,
    pub shape_def_context_key: ShapeDescriptionContextKey,
    pub function_des_context_key: FunctionDescriptionContextKey,
    pub function_des_key: FunctionDescriptionKey,
}

impl fmt::Display for PrefixOperatorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PrefixOperatorKind::Increment => write!(f, "++"),
            PrefixOperatorKind::Decrement => write!(f, "--"),
            PrefixOperatorKind::Add => write!(f, "+"),
            PrefixOperatorKind::Sub => write!(f, "-"),
            PrefixOperatorKind::Reference => write!(f, "*"),
            PrefixOperatorKind::Dereference => write!(f, "&"),
            PrefixOperatorKind::Question => write!(f, "?"),
            PrefixOperatorKind::Exclamation => write!(f, "!"),
            PrefixOperatorKind::BitNot => write!(f, "~"),
        }
    }
}

impl fmt::Display for InfixOperatorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InfixOperatorKind::Dot => write!(f, "."),
            InfixOperatorKind::Exclamation => write!(f, "!"),
            InfixOperatorKind::Question => write!(f, "?"),
            InfixOperatorKind::Pow => write!(f, "**"),
            InfixOperatorKind::IntDiv => write!(f, "//"),
            InfixOperatorKind::Mul => write!(f, "*"),
            InfixOperatorKind::Div => write!(f, "/"),
            InfixOperatorKind::Mod => write!(f, "%"),
            InfixOperatorKind::Add => write!(f, "+"),
            InfixOperatorKind::Sub => write!(f, "-"),
            InfixOperatorKind::BitShiftLeft => write!(f, "<<"),
            InfixOperatorKind::BitShiftRight => write!(f, ">>"),
            InfixOperatorKind::Gt => write!(f, ">"),
            InfixOperatorKind::Gte => write!(f, ">="),
            InfixOperatorKind::Lt => write!(f, "<"),
            InfixOperatorKind::Lte => write!(f, "<="),
            InfixOperatorKind::Eq => write!(f, "=="),
            InfixOperatorKind::Teq => write!(f, "==="),
            InfixOperatorKind::Neq => write!(f, "!="),
            InfixOperatorKind::BitAnd => write!(f, "&"),
            InfixOperatorKind::BitXor => write!(f, "^"),
            InfixOperatorKind::BitOr => write!(f, "|"),
            InfixOperatorKind::And => write!(f, "&&"),
            InfixOperatorKind::Or => write!(f, "||"),
        }
    }
}

impl fmt::Display for PostfixOperatorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PostfixOperatorKind::Question => write!(f, "?"),
            PostfixOperatorKind::Exclamation => write!(f, "!"),
            PostfixOperatorKind::Increment => write!(f, "++"),
            PostfixOperatorKind::Decrement => write!(f, "--"),
        }
    }
}

impl GrandShapeContentSummaryAtWork<'_> {
    pub fn display_operator_def_signature(
        &self,
        operator_def_signature: &OperatorDefSignature,
    ) -> String {
        match operator_def_signature {
            OperatorDefSignature::Prefix {
                prefix_op_kind,
                operand,
            } => {
                format!(
                    "({}) {}",
                    prefix_op_kind,
                    self.display_specified_shape_def_signature(operand)
                )
            }
            OperatorDefSignature::Infix {
                operand1,
                infix_op_kind,
                operand2,
            } => {
                format!(
                    "{} ({}) {}",
                    self.display_specified_shape_def_signature(operand1),
                    infix_op_kind,
                    self.display_specified_shape_def_signature(operand2)
                )
            }
            OperatorDefSignature::Postfix {
                operand,
                postfix_op_kind,
            } => {
                format!(
                    "{} ({})",
                    self.display_specified_shape_def_signature(operand),
                    postfix_op_kind,
                )
            }
        }
    }
}

impl GrandShapeContentSummaryAtWork<'_> {
    pub fn new_operator(
        &mut self,
        module_route: Route,
        pb_parsed_operator: ParsedBox<OperatorOverload>,
    ) -> Option<OperatorContentHeadKey> {
        let parsed_operator = pb_parsed_operator.owned_value();
        let template_def = parsed_operator.template_def;
        let generic_params_key;
        let shape_def_context_key;
        match template_def.and_then(|template_def| {
            self.unresolved_shape_from_pb_template_def(module_route.clone(), template_def)
        }) {
            Some((generic_params_key_piece, shape_def_context_key_piece)) => {
                generic_params_key = Some(generic_params_key_piece);
                shape_def_context_key = shape_def_context_key_piece;
            }
            None => {
                generic_params_key = None;
                shape_def_context_key =
                    self.new_shape_description_context(ShapeDescriptionContext {
                        module_route: module_route.clone(),
                        generic_param_key: None,
                        definition_pattern_variable_types: None,
                    })
            }
        }

        let (function_des_context_key, function_des_key, operator_def_signature, key) =
            match parsed_operator.annotation {
                OperatorOverloadAnnotation::Infix => {
                    let operator_kind: InfixOperatorKind =
                        parsed_operator.operator.owned_value().into();
                    let function_des_context_key =
                        self.new_function_description_context(FunctionDescriptionContext {
                            module_route: module_route.clone(),
                            generic_params_key,
                        });
                    let function_des_key = self.function_description_from_pb_function_def(
                        function_des_context_key,
                        shape_def_context_key,
                        parsed_operator.function_def,
                    )?;
                    let oprand_shape_key1 = self
                        .map_function_description(function_des_key, |function_def| {
                            function_def.function_shape
                        });
                    let operand_shape1 = self.extract_binded_shape(oprand_shape_key1)?;
                    let oprand_shape_key2 =
                        self.map_shape_description(oprand_shape_key1, |shape_des| {
                            if let ShapeDescriptionGut::Function { args: _, ret } = shape_des.gut {
                                Some(ret)
                            } else {
                                None
                            }
                        })?;
                    let operand_shape2 = self.extract_binded_shape(oprand_shape_key2)?;
                    let signature = OperatorDefSignature::Infix {
                        operand1: operand_shape1.clone(),
                        infix_op_kind: operator_kind.clone(),
                        operand2: operand_shape2.clone(),
                    };
                    let key = OperatorContentHeadKey::Infix {
                        operand1: operand_shape1.into(),
                        infix_op_kind: operator_kind,
                        operand2: operand_shape2.into(),
                    };
                    (function_des_context_key, function_des_key, signature, key)
                }
                OperatorOverloadAnnotation::Prefix => {
                    let operator_kind: PrefixOperatorKind =
                        parsed_operator.operator.owned_value().into();
                    let function_des_context_key =
                        self.new_function_description_context(FunctionDescriptionContext {
                            module_route: module_route.clone(),
                            generic_params_key,
                        });
                    let function_des_key = self.function_description_from_pb_function_def(
                        function_des_context_key,
                        shape_def_context_key,
                        parsed_operator.function_def,
                    )?;
                    let operand_shape = self.extract_binded_shape(
                        self.map_function_description(function_des_key, |function_def| {
                            function_def.function_shape
                        }),
                    )?;
                    let signature = OperatorDefSignature::Prefix {
                        prefix_op_kind: operator_kind,
                        operand: operand_shape.clone(),
                    };
                    let key = OperatorContentHeadKey::Prefix {
                        prefix_op_kind: operator_kind,
                        operand: operand_shape.into(),
                    };
                    (function_des_context_key, function_des_key, signature, key)
                }
                OperatorOverloadAnnotation::Postfix => {
                    let operator_kind: PostfixOperatorKind =
                        parsed_operator.operator.owned_value().into();
                    let function_des_context_key =
                        self.new_function_description_context(FunctionDescriptionContext {
                            module_route: module_route.clone(),
                            generic_params_key,
                        });
                    let function_des_key = self.function_description_from_pb_function_def(
                        function_des_context_key,
                        shape_def_context_key,
                        parsed_operator.function_def,
                    )?;
                    let operand_shape = self.extract_binded_shape(
                        self.map_function_description(function_des_key, |function_def| {
                            function_def.function_shape
                        }),
                    )?;
                    let signature = OperatorDefSignature::Postfix {
                        postfix_op_kind: operator_kind,
                        operand: operand_shape.clone(),
                    };
                    let key = OperatorContentHeadKey::Postfix {
                        postfix_op_kind: operator_kind,
                        operand: operand_shape.into(),
                    };
                    (function_des_context_key, function_des_key, signature, key)
                }
            };

        self.insert_operator_content_head(
            key.clone(),
            OperatorContent {
                signature: operator_def_signature,
                generic_params_key,
                shape_def_context_key,
                function_des_context_key,
                function_des_key,
            },
        );

        Some(key)
    }
}
