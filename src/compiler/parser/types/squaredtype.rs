use crate::compiler::lexer::tokens::{Operator, Paren};
use crate::compiler::parser::error::ParseError;
use crate::compiler::parser::parser::{Parsed, Parser};
use crate::compiler::parser::types::TypeGut;

impl Parser {
    pub fn parse_squared_type_gut(&mut self) -> Parsed<TypeGut> {
        self.force_parse_with_default(
            TypeGut::Unit,
            Self::try_parse_squared_type_gut,
            ParseError::expected_type_not_found,
            vec![],
        )
    }

    pub fn try_parse_squared_type_gut(&mut self) -> Option<Parsed<TypeGut>> {
        self.safe_try(|s_self| {
            let p_sqro = s_self.try_parse_paren(Paren::SquareOpen)?;
            let p_type = s_self.parse_type();
            let o_p_mapped_type = s_self.safe_try(|ss_self| {
                let p_arrow = ss_self.try_parse_operator(Operator::DoubleArrow)?;
                let p_type = ss_self.parse_type();
                Some(Parsed::merge_parsed_ignore_right(p_type, p_arrow))
            });
            let p_sqrc = s_self.parse_paren(Paren::SquareClose);
            match o_p_mapped_type {
                Some(p_mapped_type) => Some(Parsed::merge_parsed(
                    Parsed::merge_parsed_ignore_left(p_sqro, p_type),
                    Parsed::merge_parsed_ignore_right(p_mapped_type, p_sqrc),
                    |kt, vt| TypeGut::Map(s_self.new_parsed_box(kt), s_self.new_parsed_box(vt)),
                )),
                None => Some(Parsed::lift_parsed(
                    Parsed::merge_parsed_ignore_left_right(p_sqro, p_type, p_sqrc),
                    |t| TypeGut::Array(s_self.new_parsed_box(t)),
                )),
            }
        })
    }
}
