use crate::compiler::lexer::tokens::Operator;
use crate::compiler::parser::exprs::Expr;
use crate::compiler::parser::parser::{Parsed, ParsedBox, Parser};
use crate::compiler::parser::patterns::Pattern;

#[derive(Debug, Clone)]
pub struct AssignmentStmt {
    pattern: ParsedBox<Pattern>,
    assign_op: ParsedBox<Operator>,
    expr: ParsedBox<Expr>,
}

impl Parser {
    pub fn try_parse_assignment_stmt(&mut self) -> Option<Parsed<AssignmentStmt>> {
        self.safe_try(|s_self| {
            let p_pattern = Parsed::box_parsed(s_self.try_parse_pattern()?);
            let p_assign_op = Parsed::box_parsed(s_self.try_parse_between_operators(vec![
                Operator::Assign,
                Operator::AddAssign,
                Operator::SubAssign,
                Operator::MulAssign,
                Operator::DivAssign,
                Operator::ModAssign,
                Operator::BitAndAssign,
                Operator::BitOrAssign,
                Operator::BitXorAssign,
                Operator::BitShiftLeftAssign,
                Operator::BitShiftRightAssign,
            ])?);
            let p_expr = Parsed::box_parsed(s_self.parse_expr());
            Some(Parsed::merge_parsed_3(
                p_pattern,
                p_assign_op,
                p_expr,
                |pattern, assign_op, expr| AssignmentStmt {
                    pattern,
                    assign_op,
                    expr,
                },
            ))
        })
    }
}
