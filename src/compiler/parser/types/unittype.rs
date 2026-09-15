use crate::compiler::parser::parser::{Parsed, Parser};
use crate::compiler::parser::types::TypeGut;

impl Parser {
    pub fn try_parse_unit_type_gut(&mut self) -> Option<Parsed<TypeGut>> {
        self.safe_try(|s_self| {
            if let Some(_) = s_self.try_parse_unit() {
                Some(s_self.new_value(TypeGut::Unit))
            } else {
                None
            }
        })
    }
}
