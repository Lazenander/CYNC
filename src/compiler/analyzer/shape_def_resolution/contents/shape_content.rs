use crate::compiler::analyzer::common::route::route::{CanonicalRoute, Route};
use crate::compiler::analyzer::shape_def_resolution::contents::error::ContentResolutionError;
use crate::compiler::analyzer::shape_def_resolution::description::shape_description::{
    ElementaryShape, ShapeDescriptionContext,
};
use crate::compiler::analyzer::shape_def_resolution::summary::{
    GenericParamDescriptionKey, GrandShapeContentSummaryAtWork, ShapeDescriptionContextKey,
    ShapeDescriptionKey,
};
use crate::compiler::parser::parser::ParsedBox;
use crate::compiler::parser::stmts::definition_stmts::shape::ShapeDef;
use std::collections::HashMap;
use std::fmt;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ShapeContentHeadKey {
    Prelude(ElementaryShape),
    Custom(Route),
}

impl fmt::Display for ShapeContentHeadKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ShapeContentHeadKey::Prelude(shape) => write!(f, "{}", shape),
            ShapeContentHeadKey::Custom(route) => write!(f, "{}", route),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ShapeContent {
    pub key: ShapeContentHeadKey,
    pub generic_params_key: Option<GenericParamDescriptionKey>,
    pub annotated_supertype: Option<ShapeDescriptionKey>,
    pub context_key: ShapeDescriptionContextKey,
    pub description_key: ShapeDescriptionKey,
}

impl GrandShapeContentSummaryAtWork<'_> {
    pub fn new_shape(
        &mut self,
        module_route: Route,
        pb_parsed_shape: ParsedBox<ShapeDef>,
    ) -> Option<ShapeContentHeadKey> {
        let parsed_shape = pb_parsed_shape.owned_value();
        let shape_name = parsed_shape.signature.value.id.owned_value().name.clone();
        let this_shape_route = CanonicalRoute::chain_one(module_route.clone(), shape_name);

        let (generic_param_description_key, shape_context_key) = parsed_shape
            .signature
            .value
            .template_def
            .and_then(|template_def| {
                let (generic_param_description_key_piece, shape_context_key_piece) =
                    self.unresolved_shape_from_pb_template_def(module_route.clone(), template_def)?;
                Some((
                    Some(generic_param_description_key_piece),
                    self.new_shape_description_context(self.map_shape_description_context(
                        shape_context_key_piece,
                        |sd_context| ShapeDescriptionContext {
                            module_route: sd_context.module_route.clone(),
                            generic_param_key: Some(sd_context.generic_param_key.unwrap()),
                            definition_pattern_variable_types: None,
                        },
                    )),
                ))
            })
            .unwrap_or_else(|| {
                (
                    None,
                    self.new_shape_description_context(ShapeDescriptionContext {
                        module_route: module_route.clone(),
                        generic_param_key: None,
                        definition_pattern_variable_types: None,
                    }),
                )
            });
        let annotated_supershape = match parsed_shape.signature.value.annotated_type {
            Some(annotated_type) => {
                Some(self.shape_description_from_pb_type(shape_context_key, annotated_type)?)
            }
            None => None,
        };
        let pattern_variables = parsed_shape.signature.value.args.and_then(|args| {
            let (acc, is_err) = args.owned_value().into_iter().fold(
                (HashMap::new(), false),
                |(mut acc, flag), arg| {
                    let (pb_arg_name, pb_arg_type) = arg.owned_value();
                    let arg_name = pb_arg_name.value.name;
                    let arg_type_key =
                        self.shape_description_from_pb_type(shape_context_key, pb_arg_type);
                    if let Some(arg_type_key) = arg_type_key {
                        if acc.contains_key(&arg_name) {
                            self.add_error(
                                ContentResolutionError::duplicate_pattern_variable_definition(
                                    pb_arg_name.position,
                                    arg_name,
                                )
                                .into(),
                            );
                            (acc, true)
                        } else {
                            acc.insert(arg_name, arg_type_key);
                            (acc, flag)
                        }
                    } else {
                        (acc, true)
                    }
                },
            );
            if is_err {
                None
            } else {
                Some(acc)
            }
        });
        let shape_context_key = self.new_shape_description_context(
            self.map_shape_description_context(shape_context_key, |sd_context| {
                ShapeDescriptionContext {
                    module_route: sd_context.module_route.clone(),
                    generic_param_key: sd_context.generic_param_key.clone(),
                    definition_pattern_variable_types: pattern_variables,
                }
            }),
        );
        let key = ShapeContentHeadKey::Custom(this_shape_route);
        let shape_content = ShapeContent {
            key: key.clone(),
            generic_params_key: generic_param_description_key,
            annotated_supertype: annotated_supershape,
            context_key: shape_context_key,
            description_key: self
                .shape_description_from_pb_type(shape_context_key, parsed_shape.type_expr)?,
        };
        self.new_shape_content_head(key.clone(), shape_content);
        self.gsc_summary
            .bind_name_table
            .insert(key.clone(), HashMap::new());
        Some(key)
    }
}
