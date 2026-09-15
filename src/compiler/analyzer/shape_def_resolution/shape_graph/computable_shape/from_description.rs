use crate::compiler::analyzer::shape_def_resolution::description::shape_description::{ShapeDescriptionGut, UnresolvedShape};
use crate::compiler::analyzer::shape_def_resolution::shape_graph::computable_shape::computable_shape::{ComputableShape, ComputableShapeGut, ElementaryPShape, UnresolvedPShape, UnresolvedPShapeGut};
use crate::compiler::analyzer::shape_def_resolution::summary::{ComputableShapeKey, GrandShapeContentSummaryAtWork, ShapeDescriptionKey};

impl GrandShapeContentSummaryAtWork<'_> {
    pub fn build_computable_shape_from_description(
        &mut self,
        shape_description: &ShapeDescriptionKey,
    ) -> ComputableShapeKey {
        let shape_description = self.get_shape_description(*shape_description).clone();
        let position = shape_description.pos.clone();
        let computable_shape = match &shape_description.gut {
            ShapeDescriptionGut::Elementary(ele_shape) => ComputableShape {
                gut: ComputableShapeGut::Elementary(ElementaryPShape {
                    gut: ele_shape.clone(),
                    position: Some(position.clone()),
                }),
                position: Some(position),
            },
            ShapeDescriptionGut::Unresolved(UnresolvedShape::Specified(route, generic_params)) => {
                ComputableShape {
                    gut: ComputableShapeGut::Unresolved(UnresolvedPShape {
                        gut: UnresolvedPShapeGut::Specified(
                            route.clone(),
                            generic_params
                                .iter()
                                .map(|param| self.build_computable_shape_from_description(param))
                                .collect(),
                        ),
                        position: Some(position.clone()),
                    }),
                    position: Some(position),
                }
            }
            ShapeDescriptionGut::Unresolved(UnresolvedShape::GenericVariable(generic_var_name)) => {
                ComputableShape {
                    gut: ComputableShapeGut::Unresolved(UnresolvedPShape {
                        gut: UnresolvedPShapeGut::GenericVar(
                            generic_var_name.clone(),
                            self.map_shape_description_context(
                                shape_description.context_key,
                                |context| context.generic_param_key.unwrap(),
                            ),
                        ),
                        position: Some(position.clone()),
                    }),
                    position: Some(position),
                }
            }
            ShapeDescriptionGut::ParenSingle(paren_shape) => ComputableShape {
                gut: ComputableShapeGut::ParenSingle(
                    self.build_computable_shape_from_description(paren_shape),
                ),
                position: Some(position),
            },
            ShapeDescriptionGut::Tuple(tuple_shapes) => ComputableShape {
                gut: ComputableShapeGut::Tuple(
                    tuple_shapes
                        .iter()
                        .map(|tuple_shape| {
                            self.build_computable_shape_from_description(tuple_shape)
                        })
                        .collect(),
                ),
                position: Some(position),
            },
            ShapeDescriptionGut::Array(shape) => ComputableShape {
                gut: ComputableShapeGut::Array(self.build_computable_shape_from_description(shape)),
                position: Some(position),
            },
            ShapeDescriptionGut::Map { key, value } => ComputableShape {
                gut: ComputableShapeGut::Map(
                    self.build_computable_shape_from_description(key),
                    self.build_computable_shape_from_description(value),
                ),
                position: Some(position),
            },
            ShapeDescriptionGut::Struct(struct_cases) => ComputableShape {
                gut: ComputableShapeGut::Struct(
                    struct_cases
                        .iter()
                        .map(|(struct_case_field, struct_case_shape)| {
                            (
                                struct_case_field.clone(),
                                self.build_computable_shape_from_description(struct_case_shape),
                            )
                        })
                        .collect(),
                ),
                position: Some(position),
            },
            ShapeDescriptionGut::Function { args, ret } => ComputableShape {
                gut: ComputableShapeGut::Function(
                    args.iter()
                        .map(|arg| self.build_computable_shape_from_description(arg))
                        .collect(),
                    self.build_computable_shape_from_description(ret),
                ),
                position: Some(position),
            },
            ShapeDescriptionGut::Union(oprd1, oprd2) => ComputableShape {
                gut: ComputableShapeGut::Union(
                    self.build_computable_shape_from_description(oprd1),
                    self.build_computable_shape_from_description(oprd2),
                ),
                position: Some(position),
            },
            ShapeDescriptionGut::Intersection(oprd1, oprd2) => ComputableShape {
                gut: ComputableShapeGut::Intersection(
                    self.build_computable_shape_from_description(oprd1),
                    self.build_computable_shape_from_description(oprd2),
                ),
                position: Some(position),
            },
        };
        self.new_computable_shape(computable_shape)
    }
}
