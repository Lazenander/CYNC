use crate::compiler::lexer::tokens::{Keyword, Operator, Paren};
use crate::compiler::parser::exprs::identifier::Identifier;
use crate::compiler::parser::exprs::Expr;
use crate::compiler::parser::parser::{Parsed, ParsedBox, Parser};
use crate::compiler::parser::template::TemplateDef;
use crate::compiler::parser::types::Type;

#[derive(Debug, Clone)]
pub struct ShapeSignature {
    pub id: ParsedBox<Identifier>,
    pub template_def: Option<ParsedBox<TemplateDef>>,
    pub annotated_type: Option<ParsedBox<Type>>,
    pub args: Option<ParsedBox<Vec<ParsedBox<(ParsedBox<Identifier>, ParsedBox<Type>)>>>>,
}

#[derive(Debug, Clone)]
pub struct ShapeDef {
    pub is_export: bool,
    pub signature: ParsedBox<ShapeSignature>,
    pub type_expr: ParsedBox<Type>,
}

impl Parser {
    pub fn parse_shape_def(&mut self) -> Parsed<ShapeDef> {
        let o_p_export = self.try_parse_keyword(Keyword::Export);
        let p_o_export = self.push_option(o_p_export);
        let p_ktype = self.parse_keyword(Keyword::Shape);
        let p_shape_signature = Parsed::box_parsed(self.parse_shape_signature());
        let p_assign = self.parse_operator(Operator::Assign);
        let p_type = Parsed::box_parsed(self.parse_type());
        Parsed::merge_parsed_3(
            p_o_export,
            Parsed::merge_parsed_ignore_left(p_ktype, p_shape_signature),
            Parsed::merge_parsed_ignore_left(p_assign, p_type),
            |export, signature, type_expr| ShapeDef {
                is_export: export.is_some(),
                signature,
                type_expr,
            },
        )
    }

    pub fn parse_shape_signature(&mut self) -> Parsed<ShapeSignature> {
        let p_id = Parsed::box_parsed(self.parse_identifier());
        let o_p_template_def = self.try_parse_template_def(true);
        let o_p_annotated_type = self.safe_try(|s_self| {
            let p_colon = s_self.try_parse_operator(Operator::Colon)?;
            let p_type = s_self.try_parse_type()?;
            Some(Parsed::merge_parsed_ignore_left(p_colon, p_type))
        });
        let o_p_args = self.try_parse_shape_params();
        match (o_p_template_def, o_p_annotated_type, o_p_args) {
            (None, None, None) => Parsed::lift_parsed(p_id, |id| ShapeSignature {
                id,
                template_def: None,
                annotated_type: None,
                args: None,
            }),
            (Some(p_template_def), None, None) => {
                let p_template_def = Parsed::box_parsed(p_template_def);
                Parsed::merge_parsed(p_id, p_template_def, |id, template_def| ShapeSignature {
                    id,
                    template_def: Some(template_def),
                    annotated_type: None,
                    args: None,
                })
            }
            (None, Some(p_annotated_type), None) => {
                let p_annotated_type = Parsed::box_parsed(p_annotated_type);
                Parsed::merge_parsed(p_id, p_annotated_type, |id, annotated_type| {
                    ShapeSignature {
                        id,
                        template_def: None,
                        annotated_type: Some(annotated_type),
                        args: None,
                    }
                })
            }
            (None, None, Some(p_type_expr)) => {
                let p_type_expr = Parsed::box_parsed(p_type_expr);
                Parsed::merge_parsed(p_id, p_type_expr, |id, args| ShapeSignature {
                    id,
                    template_def: None,
                    annotated_type: None,
                    args: Some(args),
                })
            }
            (Some(p_template_def), Some(p_annotated_type), None) => {
                let p_template_def = Parsed::box_parsed(p_template_def);
                let p_annotated_type = Parsed::box_parsed(p_annotated_type);
                Parsed::merge_parsed_3(
                    p_id,
                    p_template_def,
                    p_annotated_type,
                    |id, template_def, annotated_type| ShapeSignature {
                        id,
                        template_def: Some(template_def),
                        annotated_type: Some(annotated_type),
                        args: None,
                    },
                )
            }
            (Some(p_template_def), None, Some(p_type_expr)) => {
                let p_template_def = Parsed::box_parsed(p_template_def);
                let p_type_expr = Parsed::box_parsed(p_type_expr);
                Parsed::merge_parsed_3(
                    p_id,
                    p_template_def,
                    p_type_expr,
                    |id, template_def, args| ShapeSignature {
                        id,
                        template_def: Some(template_def),
                        annotated_type: None,
                        args: Some(args),
                    },
                )
            }
            _ => Parsed::lift_parsed(p_id, |id| ShapeSignature {
                id,
                template_def: None,
                annotated_type: None,
                args: None,
            }),
        }
    }

    fn try_parse_shape_params(
        &mut self,
    ) -> Option<Parsed<Vec<ParsedBox<(ParsedBox<Identifier>, ParsedBox<Type>)>>>> {
        self.safe_try(|s_self| {
            let p_ro = s_self.try_parse_paren(Paren::RoundOpen)?;
            let mut p_args = s_self.parse_star(|c_self| {
                let p_arg = c_self.try_parse_shape_param()?;
                let p_comma = c_self.try_parse_operator(Operator::Comma)?;
                Some(Parsed::merge_parsed_ignore_right(p_arg, p_comma))
            });
            let o_p_last_arg = s_self.try_parse_shape_param();
            p_args = match o_p_last_arg {
                Some(p_last_arg) => {
                    Parsed::merge_parsed_append(p_args, Parsed::box_parsed(p_last_arg))
                }
                None => p_args,
            };
            let p_rc = s_self.try_parse_paren(Paren::RoundClose)?;
            Some(Parsed::merge_parsed_ignore_left_right(p_ro, p_args, p_rc))
        })
    }

    fn try_parse_shape_param(
        &mut self,
    ) -> Option<Parsed<(ParsedBox<Identifier>, ParsedBox<Type>)>> {
        self.safe_try(|s_self| {
            let p_id = Parsed::box_parsed(s_self.try_parse_identifier()?);
            let p_colon = s_self.try_parse_operator(Operator::Colon)?;
            let p_type = Parsed::box_parsed(s_self.try_parse_type()?);
            Some(Parsed::merge_parsed_ignore_middle(
                p_id,
                p_colon,
                p_type,
                |id, a_type| (id, a_type),
            ))
        })
    }
}
