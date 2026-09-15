use crate::compiler::analyzer::common::route::route::Route;
use crate::compiler::analyzer::shape_def_resolution::contents::shape_content::ShapeContentHeadKey;
use crate::compiler::analyzer::shape_def_resolution::description::shape_description::ElementaryShape;
use crate::compiler::analyzer::shape_def_resolution::shape_graph::computable_shape::computable_shape::{ComputableShape, ComputableShapeGut, UnresolvedPShape, UnresolvedPShapeGut};
use crate::compiler::analyzer::shape_def_resolution::shape_signature::GenericParamDef;
use crate::compiler::analyzer::shape_def_resolution::summary::{ComputableShapeKey, GenericParamDescriptionKey, GrandShapeContentSummaryAtWork};
use crate::compiler::parser::parser::ParsedPosition;
use std::collections::HashMap;

impl GrandShapeContentSummaryAtWork<'_> {
    pub fn substitute_simple_shape(
        &mut self,
        head_name: Route,
        args: Vec<&ComputableShapeKey>,
    ) -> ComputableShapeKey {
        let shape_content = self
            .get_shape_content_head(&ShapeContentHeadKey::Custom(head_name))
            .clone();
        let shape_description = self
            .get_shape_description(shape_content.description_key)
            .clone();

        let generic_definition_key = self.map_shape_description_context(
            shape_description.context_key,
            |shape_description_context| shape_description_context.generic_param_key,
        );

        let generic_definition = self
            .get_generic_param_description(generic_definition_key.unwrap())
            .clone();

        let mut generic_assignments = HashMap::new();
        args.clone()
            .into_iter()
            .zip(generic_definition.params.clone().into_iter())
            .for_each(|(arg, param)| {
                generic_assignments.insert(param.get_name(), arg);
            });

        let canonical_unresolved_shape_key = self
            .get_computable_shape_key(&shape_content.description_key.clone())
            .unwrap()
            .clone();
        let canonical_unresolved_shape = self.get_computable_shape(canonical_unresolved_shape_key);

        match &canonical_unresolved_shape.gut {
            ComputableShapeGut::Unresolved(UnresolvedPShape {
                gut: UnresolvedPShapeGut::GenericVar(generic_var, _),
                ..
            }) => match generic_assignments.get(generic_var) {
                None => *self.ask_for_ecs_key(&ElementaryShape::Any),
                Some(substitution_shape) => **substitution_shape,
            },
            _ => canonical_unresolved_shape_key,
        }
    }

    pub fn substitute_generic(
        &mut self,
        this_generic_var: String,
        generic_param_key: GenericParamDescriptionKey,
    ) -> ComputableShapeKey {
        let generic_param = self.get_generic_param_description(generic_param_key);
        generic_param
            .clone()
            .params
            .into_iter()
            .find_map(|param| match param {
                GenericParamDef::Variable(generic_var) => {
                    if generic_var == this_generic_var {
                        Some(*self.ask_for_ecs_key(&ElementaryShape::Any))
                    } else {
                        None
                    }
                }
                GenericParamDef::Guarded(generic_var, guard) => {
                    if generic_var == this_generic_var {
                        let key = self.get_computable_shape_key(&guard)?.clone();
                        Some(self.substitute_position(key, None))
                    } else {
                        None
                    }
                }
            })
            .unwrap()
    }

    pub fn substitute_position(
        &mut self,
        shape_key: ComputableShapeKey,
        position: Option<ParsedPosition>,
    ) -> ComputableShapeKey {
        let computable_shape = ComputableShape {
            gut: match self.map_computable_shape(shape_key, |shape| shape.gut.clone()) {
                ComputableShapeGut::Unresolved(UnresolvedPShape {
                    gut: UnresolvedPShapeGut::Specified(route, shapes),
                    ..
                }) => ComputableShapeGut::Unresolved(UnresolvedPShape {
                    gut: UnresolvedPShapeGut::Specified(
                        route.clone(),
                        shapes
                            .into_iter()
                            .map(|shape| self.substitute_position(shape, position.clone()))
                            .collect(),
                    ),
                    position: position.clone(),
                }),
                ComputableShapeGut::Tuple(shapes) => ComputableShapeGut::Tuple(
                    shapes
                        .into_iter()
                        .map(|shape| self.substitute_position(shape, position.clone()))
                        .collect(),
                ),
                ComputableShapeGut::Array(shape) => {
                    ComputableShapeGut::Array(self.substitute_position(shape, position.clone()))
                }
                ComputableShapeGut::Map(shape1, shape2) => ComputableShapeGut::Map(
                    self.substitute_position(shape1, position.clone()),
                    self.substitute_position(shape2, position.clone()),
                ),
                ComputableShapeGut::Struct(struct_cases) => ComputableShapeGut::Struct(
                    struct_cases
                        .into_iter()
                        .map(|(case_field_name, case_shape)| {
                            (
                                case_field_name.clone(),
                                self.substitute_position(case_shape, position.clone()),
                            )
                        })
                        .collect(),
                ),
                ComputableShapeGut::Function(param_shapes, ret_shape) => {
                    ComputableShapeGut::Function(
                        param_shapes
                            .into_iter()
                            .map(|shape| self.substitute_position(shape, position.clone()))
                            .collect(),
                        self.substitute_position(ret_shape, position.clone()),
                    )
                }
                other => other,
            },
            position,
        };

        self.new_computable_shape(computable_shape)
    }
}
