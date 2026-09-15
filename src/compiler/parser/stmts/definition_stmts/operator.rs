use crate::compiler::lexer::tokens::{Keyword, Operator};
use crate::compiler::parser::exprs::FunctionDef;
use crate::compiler::parser::parser::{Parsed, ParsedBox, Parser};
use crate::compiler::parser::stmts::definition_stmts::operation::OperationDef;
use crate::compiler::parser::template::TemplateDef;

#[derive(Debug, Clone)]
pub enum OperatorOverloadAnnotation {
    Infix,
    Prefix,
    Postfix,
}

#[derive(Debug, Clone)]
pub struct OperatorOverload {
    pub annotation: OperatorOverloadAnnotation,
    pub operator: ParsedBox<Operator>,
    pub template_def: Option<ParsedBox<TemplateDef>>,
    pub function_def: ParsedBox<FunctionDef>,
}

impl Parser {
    pub fn parse_operator_overload_def(&mut self) -> Parsed<OperatorOverload> {
        let p_annotation = Parsed::lift_parsed(
            self.chain_try_parses_with_default_parse(
                vec![
                    |c_self| c_self.try_parse_keyword(Keyword::Infix),
                    |c_self| c_self.try_parse_keyword(Keyword::Prefix),
                ],
                |c_self| c_self.parse_keyword(Keyword::Postfix),
            ),
            |keyword| match keyword {
                Keyword::Infix => OperatorOverloadAnnotation::Infix,
                Keyword::Prefix => OperatorOverloadAnnotation::Prefix,
                Keyword::Postfix => OperatorOverloadAnnotation::Postfix,
                _ => unreachable!(),
            },
        );
        let p_op = Parsed::box_parsed(self.parse_between_operators(
            match p_annotation.value.clone() {
                OperatorOverloadAnnotation::Infix => vec![
                    Operator::Dot,
                    Operator::Exclamation,
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
                ],
                OperatorOverloadAnnotation::Prefix => vec![
                    Operator::Increment,
                    Operator::Decrement,
                    Operator::Mul,
                    Operator::BitAnd,
                    Operator::Add,
                    Operator::Sub,
                    Operator::Question,
                    Operator::Exclamation,
                    Operator::BitNot,
                ],
                OperatorOverloadAnnotation::Postfix => vec![
                    Operator::Increment,
                    Operator::Decrement,
                    Operator::Question,
                    Operator::Exclamation,
                ],
            },
        ));
        let o_p_t_def = self.try_parse_template_def(false);
        let p_f_def = Parsed::box_parsed(self.parse_function_def());
        match o_p_t_def {
            Some(p_t_def) => {
                let p_t_def = Parsed::box_parsed(p_t_def);
                Parsed::merge_parsed_3(
                    Parsed::merge_parsed(p_annotation, p_op, |annotation, op| (annotation, op)),
                    p_t_def,
                    p_f_def,
                    |(annotation, operator), template_def, function_def| OperatorOverload {
                        annotation,
                        operator,
                        template_def: Some(template_def),
                        function_def,
                    },
                )
            }
            None => Parsed::merge_parsed_3(
                p_annotation,
                p_op,
                p_f_def,
                |annotation, operator, function_def| OperatorOverload {
                    annotation,
                    operator,
                    template_def: None,
                    function_def,
                },
            ),
        }
    }
}
