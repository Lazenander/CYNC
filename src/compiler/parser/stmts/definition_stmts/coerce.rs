use crate::compiler::lexer::tokens::Keyword;
use crate::compiler::parser::exprs::identifier::Identifier;
use crate::compiler::parser::exprs::FunctionDef;
use crate::compiler::parser::parser::{Parsed, ParsedBox, Parser};
use crate::compiler::parser::template::TemplateDef;

#[derive(Debug, Clone)]
pub struct Coerce {
    pub template_def: Option<ParsedBox<TemplateDef>>,
    pub function_def: ParsedBox<FunctionDef>,
}

impl Parser {
    pub fn parse_coerce_def(&mut self) -> Parsed<Coerce> {
        let p_coerce = self.parse_keyword(Keyword::Coerce);
        let o_p_t_def = self.try_parse_template_def(false);
        let p_f_def = Parsed::box_parsed(self.parse_function_def());
        match o_p_t_def {
            Some(p_t_def) => {
                let p_t_def = Parsed::box_parsed(p_t_def);
                Parsed::merge_parsed(
                    Parsed::merge_parsed_ignore_left(p_coerce, p_t_def),
                    p_f_def,
                    |template_def, function_def| Coerce {
                        template_def: Some(template_def),
                        function_def,
                    },
                )
            }
            None => Parsed::lift_parsed(
                Parsed::merge_parsed_ignore_left(p_coerce, p_f_def),
                |function_def| Coerce {
                    template_def: None,
                    function_def,
                },
            ),
        }
    }
}
