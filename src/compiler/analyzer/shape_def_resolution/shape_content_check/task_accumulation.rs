use crate::compiler::analyzer::shape_def_resolution::contents::shape_content::ShapeContentHeadKey;
use crate::compiler::analyzer::shape_def_resolution::description::shape_description::{
    ShapeDescriptionGut, UnresolvedShape,
};
use crate::compiler::analyzer::shape_def_resolution::shape_content_check::error::ShapeContentCheckError;
use crate::compiler::analyzer::shape_def_resolution::shape_signature::GenericParamDef;
use crate::compiler::analyzer::shape_def_resolution::summary::{
    GrandShapeContentSummaryAtWork, ShapeDescriptionKey,
};

impl GrandShapeContentSummaryAtWork<'_> {
    pub fn does_shape_generic_size_fit(
        &mut self,
        shape_content_head_key: ShapeContentHeadKey,
    ) -> Vec<(
        ShapeContentHeadKey,
        ShapeContentHeadKey,
        ShapeDescriptionKey,
        ShapeDescriptionKey,
    )> {
        let shape_content = self.get_shape_content_head(&shape_content_head_key).clone();
        self.does_in_shape_generic_size_fit(&shape_content_head_key, &shape_content.description_key)
    }

    fn does_in_shape_generic_size_fit(
        &mut self,
        shape_content_head_key: &ShapeContentHeadKey,
        shape_description_key: &ShapeDescriptionKey,
    ) -> Vec<(
        ShapeContentHeadKey,
        ShapeContentHeadKey,
        ShapeDescriptionKey,
        ShapeDescriptionKey,
    )> {
        let shape_description = self.get_shape_description(*shape_description_key).clone();

        match shape_description.gut {
            ShapeDescriptionGut::Elementary(_) => vec![],
            ShapeDescriptionGut::Unresolved(unresolved_shape) => match unresolved_shape {
                UnresolvedShape::GenericVariable(_) => vec![],
                UnresolvedShape::Specified(specified_shape, specified_shape_args) => {
                    let specified_shape_content = self
                        .get_shape_content_head(&ShapeContentHeadKey::Custom(
                            specified_shape.clone(),
                        ))
                        .clone();
                    let the_new_slots = specified_shape_content
                        .generic_params_key
                        .map(|generic_params_key| {
                            self.get_generic_param_description(generic_params_key)
                        })
                        .map(|slots| slots.params.clone())
                        .unwrap();
                    if specified_shape_args.len() != the_new_slots.len() {
                        self.add_error(
                            ShapeContentCheckError::generic_param_size_incompatible(
                                shape_description.pos,
                                specified_shape,
                                specified_shape_args.len(),
                                the_new_slots.len(),
                            )
                            .into(),
                        );
                        vec![]
                    } else {
                        specified_shape_args
                            .iter()
                            .for_each(|(specified_shape_arg)| {
                                self.does_in_shape_generic_size_fit(
                                    shape_content_head_key,
                                    specified_shape_arg,
                                );
                            });
                        specified_shape_args
                            .into_iter()
                            .zip(the_new_slots.into_iter())
                            .filter_map(|(specified_shape_key, generic_param_def)| {
                                match generic_param_def {
                                    GenericParamDef::Variable(_) => None,
                                    GenericParamDef::Guarded(_, guard_shape_key) => Some((
                                        shape_content_head_key.clone(),
                                        ShapeContentHeadKey::Custom(specified_shape.clone()),
                                        specified_shape_key,
                                        guard_shape_key,
                                    )),
                                }
                            })
                            .collect()
                    }
                }
            },
            ShapeDescriptionGut::ParenSingle(paren_shape) => {
                self.does_in_shape_generic_size_fit(shape_content_head_key, &paren_shape)
            }
            ShapeDescriptionGut::Tuple(tuple_shape) => {
                tuple_shape.iter().fold(vec![], |acc, tuple_element| {
                    acc.into_iter()
                        .chain(
                            self.does_in_shape_generic_size_fit(
                                shape_content_head_key,
                                tuple_element,
                            ),
                        )
                        .collect()
                })
            }
            ShapeDescriptionGut::Array(array_shape) => {
                self.does_in_shape_generic_size_fit(shape_content_head_key, &array_shape)
            }
            ShapeDescriptionGut::Map { key, value } => self
                .does_in_shape_generic_size_fit(shape_content_head_key, &key)
                .into_iter()
                .chain(
                    self.does_in_shape_generic_size_fit(shape_content_head_key, &value)
                        .into_iter(),
                )
                .collect(),
            ShapeDescriptionGut::Struct(struct_cases) => {
                struct_cases.into_iter().fold(vec![], |acc, (_, value)| {
                    acc.into_iter()
                        .chain(
                            self.does_in_shape_generic_size_fit(shape_content_head_key, &value)
                                .into_iter(),
                        )
                        .collect()
                })
            }
            ShapeDescriptionGut::Function { args, ret } => args
                .into_iter()
                .fold(vec![], |acc, arg| {
                    acc.into_iter()
                        .chain(
                            self.does_in_shape_generic_size_fit(shape_content_head_key, &arg)
                                .into_iter(),
                        )
                        .collect()
                })
                .into_iter()
                .chain(
                    self.does_in_shape_generic_size_fit(shape_content_head_key, &ret)
                        .into_iter(),
                )
                .collect(),
            ShapeDescriptionGut::Union(union1, union2) => self
                .does_in_shape_generic_size_fit(shape_content_head_key, &union1)
                .into_iter()
                .chain(
                    self.does_in_shape_generic_size_fit(shape_content_head_key, &union2)
                        .into_iter(),
                )
                .collect(),
            ShapeDescriptionGut::Intersection(intersection1, intersection2) => self
                .does_in_shape_generic_size_fit(shape_content_head_key, &intersection1)
                .into_iter()
                .chain(
                    self.does_in_shape_generic_size_fit(shape_content_head_key, &intersection2)
                        .into_iter(),
                )
                .collect(),
        }
    }
}
