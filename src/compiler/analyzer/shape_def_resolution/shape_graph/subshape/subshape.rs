use crate::compiler::analyzer::shape_def_resolution::description::shape_description::ElementaryShape;
use crate::compiler::analyzer::shape_def_resolution::shape_graph::computable_shape::computable_shape::{ComputableShapeGut, ElementaryPShape, UnresolvedPShape, UnresolvedPShapeGut};
use crate::compiler::analyzer::shape_def_resolution::summary::{ComputableShapeKey, GrandShapeContentSummaryAtWork, ShapeDescriptionKey};

#[derive(Eq, PartialEq, Ord, PartialOrd)]
pub enum IsSubShapeResult {
    True,
    Stuck,
    OverCost,
    False,
}

impl IsSubShapeResult {
    pub fn is_true(&self) -> bool {
        match self {
            IsSubShapeResult::True => true,
            _ => false,
        }
    }

    pub fn is_false(&self) -> bool {
        match self {
            IsSubShapeResult::False => true,
            _ => false,
        }
    }
}

impl GrandShapeContentSummaryAtWork<'_> {
    pub fn shape_description_is_subshape(
        &mut self,
        potential_subshape: &ShapeDescriptionKey,
        potential_supershape: &ShapeDescriptionKey,
    ) -> IsSubShapeResult {
        let (potential_computable_subshape, potential_computable_supershape) = (
            self.get_computable_shape_key(&potential_subshape).clone(),
            self.get_computable_shape_key(&potential_supershape).clone(),
        );
        self.computable_shape_is_subshape(
            potential_computable_subshape,
            potential_computable_supershape,
        )
    }

    pub fn computable_shape_is_subshape(
        &mut self,
        potential_subshape: ComputableShapeKey,
        potential_supershape: ComputableShapeKey,
    ) -> IsSubShapeResult {
        self.in_computable_shape_is_subshape(potential_subshape, potential_supershape, 0)
    }

    pub fn in_computable_shape_is_subshape(
        &mut self,
        potential_subshape: ComputableShapeKey,
        potential_supershape: ComputableShapeKey,
        cost: usize,
    ) -> IsSubShapeResult {
        let computable_subshape = self
            .get_computable_shape(potential_subshape.clone())
            .clone();
        let computable_supershape = self
            .get_computable_shape(potential_supershape.clone())
            .clone();
        match (computable_subshape.gut, computable_supershape.gut) {
            (
                ComputableShapeGut::Elementary(ElementaryPShape {
                    gut: ElementaryShape::Empty,
                    position: _,
                }),
                _,
            )
            | (
                _,
                ComputableShapeGut::Elementary(ElementaryPShape {
                    gut: ElementaryShape::Any,
                    position: _,
                }),
            ) => true,

            (
                ComputableShapeGut::Unresolved(UnresolvedPShape {
                    gut: UnresolvedPShapeGut::Specified(route1, generic_args1),
                    ..
                }),
                ComputableShapeGut::Unresolved(UnresolvedPShape {
                    gut: UnresolvedPShapeGut::Specified(route2, generic_args2),
                    ..
                }),
            ) => {
                if self.is_subshape_or_assumption(&potential_subshape, &potential_supershape) {
                    return true;
                }
                if self.is_not_subshape_from_assumption(&potential_subshape, &potential_supershape)
                {
                    return false;
                }

                self.add_assumption(potential_subshape.clone(), potential_supershape.clone());

                let p_shape1 =
                    self.substitute_simple_shape(route1.clone(), generic_args1.iter().collect());
                let p_shape2 =
                    self.substitute_simple_shape(route2.clone(), generic_args2.iter().collect());

                let shape1 = self.substitute_position(
                    p_shape1,
                    self.get_computable_shape(potential_subshape)
                        .position
                        .clone(),
                );
                let shape2 = self.substitute_position(
                    p_shape2,
                    self.get_computable_shape(potential_supershape)
                        .position
                        .clone(),
                );

                let res = self.in_computable_shape_is_subshape(shape1, shape2, cost + 1);

                if res.is_true() {
                    self.realize_assumption_true(potential_subshape, potential_supershape);
                } else if res.is_false() {
                    self.realize_assumption_false(potential_subshape, potential_supershape);
                } else {
                    self.realize_assumption_incomputable(potential_subshape, potential_supershape);
                }

                res
            }

            (
                ComputableShapeGut::Unresolved(UnresolvedPShape {
                    gut: UnresolvedPShapeGut::Specified(route1, generic_args1),
                    ..
                }),
                ComputableShapeGut::Unresolved(UnresolvedPShape {
                    gut: UnresolvedPShapeGut::GenericVar(generic_var2, generic_param_des2),
                    ..
                }),
            ) => {
                let key1 = self.usk_from_csk(potential_subshape.clone());
                let key2 = self.usk_from_csk(potential_supershape.clone());
                if self.is_subshape_or_assumption(&key1, &key2) {
                    return true;
                }
                if self.is_not_subshape_from_assumption(&key1, &key2) {
                    return false;
                }

                self.add_assumption(key1.clone(), key2.clone());

                let p_shape1 =
                    self.substitute_simple_shape(route1.clone(), generic_args1.iter().collect());
                let p_shape2 =
                    self.substitute_generic(generic_var2.clone(), generic_param_des2.clone());

                let shape1 = self.substitute_position(
                    p_shape1,
                    self.get_computable_shape(potential_subshape)
                        .position
                        .clone(),
                );
                let shape2 = self.substitute_position(
                    p_shape2,
                    self.get_computable_shape(potential_supershape)
                        .position
                        .clone(),
                );

                let res = self.in_computable_shape_is_subshape(shape1, shape2);

                if res {
                    self.realize_assumption_true(key1, key2);
                } else {
                    self.realize_assumption_false(key1, key2);
                }

                res
            }

            (
                ComputableShapeGut::Unresolved(UnresolvedPShape {
                    gut: UnresolvedPShapeGut::GenericVar(generic_var1, generic_param_des1),
                    ..
                }),
                ComputableShapeGut::Unresolved(UnresolvedPShape {
                    gut: UnresolvedPShapeGut::Specified(route2, generic_args2),
                    ..
                }),
            ) => {
                let key1 = self.usk_from_csk(potential_subshape.clone());
                let key2 = self.usk_from_csk(potential_supershape.clone());
                if self.is_subshape_or_assumption(&key1, &key2) {
                    return true;
                }
                if self.is_not_subshape_from_assumption(&key1, &key2) {
                    return false;
                }

                self.add_assumption(key1.clone(), key2.clone());

                let p_shape1 =
                    self.substitute_generic(generic_var1.clone(), generic_param_des1.clone());
                let p_shape2 =
                    self.substitute_simple_shape(route2.clone(), generic_args2.iter().collect());

                let shape1 = self.substitute_position(
                    p_shape1,
                    self.get_computable_shape(potential_subshape)
                        .position
                        .clone(),
                );
                let shape2 = self.substitute_position(
                    p_shape2,
                    self.get_computable_shape(potential_supershape)
                        .position
                        .clone(),
                );

                let res = self.in_computable_shape_is_subshape(shape1, shape2);

                if res {
                    self.realize_assumption_true(key1, key2);
                } else {
                    self.realize_assumption_false(key1, key2);
                }

                res
            }

            (
                ComputableShapeGut::Unresolved(UnresolvedPShape {
                    gut: UnresolvedPShapeGut::GenericVar(generic_var1, generic_param_des1),
                    ..
                }),
                ComputableShapeGut::Unresolved(UnresolvedPShape {
                    gut: UnresolvedPShapeGut::GenericVar(generic_var2, generic_param_des2),
                    ..
                }),
            ) => {
                let key1 = self.usk_from_csk(potential_subshape.clone());
                let key2 = self.usk_from_csk(potential_supershape.clone());
                if self.is_subshape_or_assumption(&key1, &key2) {
                    return true;
                }
                if self.is_not_subshape_from_assumption(&key1, &key2) {
                    return false;
                }

                self.add_assumption(key1.clone(), key2.clone());

                let p_shape1 =
                    self.substitute_generic(generic_var1.clone(), generic_param_des1.clone());
                let p_shape2 =
                    self.substitute_generic(generic_var2.clone(), generic_param_des2.clone());

                let shape1 = self.substitute_position(
                    p_shape1,
                    self.get_computable_shape(potential_subshape)
                        .position
                        .clone(),
                );
                let shape2 = self.substitute_position(
                    p_shape2,
                    self.get_computable_shape(potential_supershape)
                        .position
                        .clone(),
                );

                let res = self.in_computable_shape_is_subshape(shape1, shape2);

                if res {
                    self.realize_assumption_true(key1, key2);
                } else {
                    self.realize_assumption_false(key1, key2);
                }

                res
            }

            (
                ComputableShapeGut::Unresolved(UnresolvedPShape {
                    gut: UnresolvedPShapeGut::GenericVar(generic_var1, generic_param_des1),
                    ..
                }),
                _,
            ) => {
                let p_shape1 =
                    self.substitute_generic(generic_var1.clone(), generic_param_des1.clone());
                let shape1 = self.substitute_position(
                    p_shape1,
                    self.get_computable_shape(potential_subshape)
                        .position
                        .clone(),
                );

                self.in_computable_shape_is_subshape(shape1, potential_supershape)
            }

            (
                _,
                ComputableShapeGut::Unresolved(UnresolvedPShape {
                    gut: UnresolvedPShapeGut::GenericVar(generic_var2, generic_param_des2),
                    ..
                }),
            ) => {
                let p_shape2 =
                    self.substitute_generic(generic_var2.clone(), generic_param_des2.clone());
                let shape2 = self.substitute_position(
                    p_shape2,
                    self.get_computable_shape(potential_supershape)
                        .position
                        .clone(),
                );

                self.in_computable_shape_is_subshape(potential_subshape, shape2)
            }

            (
                ComputableShapeGut::Unresolved(UnresolvedPShape {
                    gut: UnresolvedPShapeGut::Specified(route1, generic_args1),
                    ..
                }),
                _,
            ) => {
                let p_shape1 =
                    self.substitute_simple_shape(route1.clone(), generic_args1.iter().collect());
                let shape1 = self.substitute_position(
                    p_shape1,
                    self.get_computable_shape(potential_subshape)
                        .position
                        .clone(),
                );

                self.in_computable_shape_is_subshape(shape1, potential_supershape)
            }

            (
                _,
                ComputableShapeGut::Unresolved(UnresolvedPShape {
                    gut: UnresolvedPShapeGut::Specified(route2, generic_args2),
                    ..
                }),
            ) => {
                let p_shape2 =
                    self.substitute_simple_shape(route2.clone(), generic_args2.iter().collect());
                let shape2 = self.substitute_position(
                    p_shape2,
                    self.get_computable_shape(potential_supershape)
                        .position
                        .clone(),
                );

                self.in_computable_shape_is_subshape(potential_subshape, shape2)
            }

            (
                ComputableShapeGut::Elementary(ele_p_shape1),
                ComputableShapeGut::Elementary(ele_p_shape2),
            ) => ElementaryPShape::pairwise_is_subset(&ele_p_shape1, &ele_p_shape2),

            (
                ComputableShapeGut::Tuple(tuple_shapes1),
                ComputableShapeGut::Tuple(tuple_shapes2),
            ) => {
                tuple_shapes1.len() == tuple_shapes2.len()
                    && tuple_shapes1
                        .into_iter()
                        .zip(tuple_shapes2.into_iter())
                        .map(|(shape1, shape2)| {
                            self.in_computable_shape_is_subshape(shape1, shape2)
                        })
                        .all(|x| x)
            }

            (ComputableShapeGut::Array(shape1), ComputableShapeGut::Array(shape2)) => {
                self.in_computable_shape_is_subshape(shape1, shape2)
            }

            (ComputableShapeGut::Map(key1, value1), ComputableShapeGut::Map(key2, value2)) => {
                self.in_computable_shape_is_subshape(key1, key2)
                    && self.in_computable_shape_is_subshape(value1, value2)
            }

            (
                ComputableShapeGut::Struct(struct_case1),
                ComputableShapeGut::Struct(struct_case2),
            ) => struct_case2.into_iter().all(|(field_name, case_shape)| {
                self.in_computable_shape_is_subshape(
                    if struct_case1.contains_key(&field_name) {
                        struct_case1.get(&field_name).unwrap()
                    } else {
                        self.ask_for_ecs_key(&ElementaryShape::Any)
                    }
                    .clone(),
                    case_shape,
                )
            }),

            (
                ComputableShapeGut::Function(args1, ret1),
                ComputableShapeGut::Function(args2, ret2),
            ) => {
                args1.len() == args2.len()
                    && args1
                        .into_iter()
                        .zip(args2.into_iter())
                        .map(|(shape1, shape2)| {
                            self.in_computable_shape_is_subshape(shape2, shape1)
                        })
                        .all(|x| x)
                    && self.in_computable_shape_is_subshape(ret1, ret2)
            }

            (ComputableShapeGut::Union(shape_u1, shape1_u2), _) => {
                self.in_computable_shape_is_subshape(shape_u1, potential_supershape)
                    && self.in_computable_shape_is_subshape(shape1_u2, potential_supershape)
            }

            (_, ComputableShapeGut::Union(shape_u1, shape1_u2)) => {
                self.in_computable_shape_is_subshape(potential_subshape, shape_u1)
                    || self.in_computable_shape_is_subshape(potential_subshape, shape1_u2)
            }

            (ComputableShapeGut::Intersection(shape_i1, shape_i2), _) => self
                .left_meet_in_computable_shape_is_subshape(
                    vec![shape_i1, shape_i2],
                    potential_supershape,
                ),

            (_, ComputableShapeGut::Intersection(shape_i1, shape1_i2)) => {
                self.in_computable_shape_is_subshape(potential_subshape, shape_i1)
                    && self.in_computable_shape_is_subshape(potential_subshape, shape1_i2)
            }

            (_, _) => false,
        }
    }
}
