use crate::compiler::analyzer::shape_def_resolution::description::shape_description::ElementaryShape;
use crate::compiler::analyzer::shape_def_resolution::shape_graph::computable_shape::computable_shape::{ComputableShapeGut, ElementaryPShape, UnresolvedPShapeGut};
use crate::compiler::analyzer::shape_def_resolution::shape_graph::subshape::subshape::IsSubShapeResult;
use crate::compiler::analyzer::shape_def_resolution::summary::{ComputableShapeKey, GrandShapeContentSummaryAtWork};
use crate::utility::consts::SUBSHAPE_MAX_COST;
use std::cmp::max;
use std::collections::HashSet;

impl GrandShapeContentSummaryAtWork<'_> {
    fn naive_merge(
        &mut self,
        meet_shapes: Vec<ComputableShapeKey>,
        join_shapes: Vec<ComputableShapeKey>,
    ) -> (Vec<ComputableShapeKey>, Vec<ComputableShapeKey>) {
        let (naive_shapes, complex_shapes) =
            meet_shapes.split(
                |meet_shape| match &self.get_computable_shape(*meet_shape).gut {
                    ComputableShapeGut::Elementary(_)
                    | ComputableShapeGut::Tuple(_)
                    | ComputableShapeGut::Array(_)
                    | ComputableShapeGut::Map(_, _)
                    | ComputableShapeGut::Struct(_) => true,

                    _ => false,
                },
            );

        (meet_shapes, join_shapes)
    }

    pub(super) fn left_meet_right_join_in_computable_shape_is_subshape(
        &mut self,
        meet_shapes: Vec<ComputableShapeKey>,
        join_shapes: Vec<ComputableShapeKey>,
        mut realize_stuck_buffer: HashSet<UnresolvedPShapeGut>,
        mut cost: usize,
    ) -> IsSubShapeResult {
        if cost > SUBSHAPE_MAX_COST {
            return IsSubShapeResult::OverCost;
        }

        if meet_shapes.len() == 0 || join_shapes.len() == 0 {
            unreachable!();
        }

        if meet_shapes.len() == 1 && join_shapes.len() == 1 {
            return self.in_computable_shape_is_subshape(
                *meet_shapes.first().unwrap(),
                *join_shapes.first().unwrap(),
                cost + 1,
            );
        }

        if meet_shapes.iter().any(|left| {
            join_shapes.iter().any(|right| {
                self.in_computable_shape_is_subshape(*left, *right, cost + 1)
                    .is_true()
            })
        }) {
            return IsSubShapeResult::True;
        }

        let mut is_realized = None;

        let shape_realize = |shapes: Vec<ComputableShapeKey>| {
            shapes
                .into_iter()
                .map(|shape| {
                    if let ComputableShapeGut::Unresolved(p_shape) =
                        &self.get_computable_shape(shape).gut
                    {
                        if is_realized.is_none() {
                            is_realized = Some(false);
                        }
                        if !realize_stuck_buffer.contains(&p_shape.gut) {
                            realize_stuck_buffer.insert(p_shape.gut.clone());

                            is_realized = Some(true);

                            if let UnresolvedPShapeGut::GenericVar(generic_var, generic_param_des) =
                                &p_shape.gut
                            {
                                let p_shape = self.substitute_generic(
                                    generic_var.clone(),
                                    generic_param_des.clone(),
                                );
                                return self.substitute_position(
                                    p_shape,
                                    self.get_computable_shape(shape).position.clone(),
                                );
                            } else if let UnresolvedPShapeGut::Specified(route1, generic_args1) =
                                &p_shape.gut
                            {
                                let p_shape = self.substitute_simple_shape(
                                    route1.clone(),
                                    generic_args1.iter().collect(),
                                );
                                return self.substitute_position(
                                    p_shape,
                                    self.get_computable_shape(shape).position.clone(),
                                );
                            }
                        }
                    }

                    shape
                })
                .collect()
        };

        if is_realized == Some(false) {
            return IsSubShapeResult::Stuck;
        }

        let left_shapes: Vec<_> = shape_realize(meet_shapes);
        let right_shapes: Vec<_> = shape_realize(join_shapes);

        if left_shapes
            .iter()
            .all(|left_shape| self.get_computable_shape(*left_shape).is_empty())
            || right_shapes
                .iter()
                .all(|right_shape| self.get_computable_shape(*right_shape).is_any())
        {
            return IsSubShapeResult::True;
        }

        if left_shapes.iter().any(|shape| {
            matches!(
                self.get_computable_shape(*shape).gut,
                ComputableShapeGut::Union(..)
            )
        }) {
            return left_shapes
                .into_iter()
                .fold(vec![], |mut acc: Vec<Vec<_>>, left_shape| {
                    cost += 1;
                    match self.get_computable_shape(left_shape).gut {
                        ComputableShapeGut::Union(shape_key1, shape_key2) => acc
                            .clone()
                            .into_iter()
                            .map(|vec| vec.into_iter().chain(vec![shape_key1]).collect())
                            .chain(
                                acc.into_iter()
                                    .map(|vec| vec.into_iter().chain(vec![shape_key2]).collect()),
                            )
                            .collect(),
                        _ => acc
                            .into_iter()
                            .map(|vec| vec.into_iter().chain(vec![left_shape]).collect())
                            .collect(),
                    }
                })
                .into_iter()
                .fold(IsSubShapeResult::False, |acc, new_left_shapes| {
                    max(
                        acc,
                        self.left_meet_right_join_in_computable_shape_is_subshape(
                            new_left_shapes,
                            right_shapes,
                            realize_stuck_buffer,
                            cost,
                        ),
                    )
                });
        }

        if left_shapes.iter().any(|left_shape| {
            matches!(
                self.get_computable_shape(*left_shape).gut,
                ComputableShapeGut::Intersection(..)
            )
        }) {
            return self.left_meet_right_join_in_computable_shape_is_subshape(
                left_shapes.into_iter().fold(vec![], |acc, left_shape| {
                    match self.get_computable_shape(left_shape).gut {
                        ComputableShapeGut::Intersection(shape_i1, shape_i2) => {
                            acc.into_iter().chain(vec![shape_i1, shape_i2]).collect()
                        }
                        _ => acc.into_iter().chain(vec![left_shape]).collect(),
                    }
                }),
                right_shapes,
                realize_stuck_buffer,
                cost + 1,
            );
        }

        if right_shapes.iter().any(|right_shape| {
            matches!(
                self.get_computable_shape(*right_shape).gut,
                ComputableShapeGut::Union(..)
            )
        }) {
            return right_shapes
                .into_iter()
                .fold(vec![], |mut acc: Vec<Vec<_>>, right_shape| {
                    cost += 1;
                    match self.get_computable_shape(right_shape).gut {
                        ComputableShapeGut::Intersection(shape_key1, shape_key2) => acc
                            .clone()
                            .into_iter()
                            .map(|vec| vec.into_iter().chain(vec![shape_key1]).collect())
                            .chain(
                                acc.into_iter()
                                    .map(|vec| vec.into_iter().chain(vec![shape_key2]).collect()),
                            )
                            .collect(),
                        _ => acc
                            .into_iter()
                            .map(|vec| vec.into_iter().chain(vec![right_shape]).collect())
                            .collect(),
                    }
                })
                .into_iter()
                .fold(IsSubShapeResult::False, |acc, new_right_shapes| {
                    max(
                        acc,
                        self.left_meet_right_join_in_computable_shape_is_subshape(
                            left_shapes,
                            new_right_shapes,
                            realize_stuck_buffer,
                            cost,
                        ),
                    )
                });
        }

        if right_shapes.iter().any(|left_shape| {
            matches!(
                self.get_computable_shape(*left_shape).gut,
                ComputableShapeGut::Intersection(..)
            )
        }) {
            return self.left_meet_right_join_in_computable_shape_is_subshape(
                left_shapes,
                right_shapes.into_iter().fold(vec![], |acc, right_shape| {
                    match self.get_computable_shape(right_shape).gut {
                        ComputableShapeGut::Union(shape_i1, shape_i2) => {
                            acc.into_iter().chain(vec![shape_i1, shape_i2]).collect()
                        }
                        _ => acc.into_iter().chain(vec![right_shape]).collect(),
                    }
                }),
                realize_stuck_buffer,
                cost + 1,
            );
        }

        if left_shapes.into_iter().all(|left_shape| {
            matches!(
                self.get_computable_shape(left_shape).gut,
                ComputableShapeGut::Elementary(_)
            )
        }) && right_shapes.into_iter().all(|right_shape| {
            matches!(
                self.get_computable_shape(right_shape).gut,
                ComputableShapeGut::Elementary(_)
            )
        }) {
            return;
        }

        match (
            computable_subshape1.gut,
            computable_subshape2.gut,
            computable_supershape.gut,
        ) {
            (
                ComputableShapeGut::Elementary(ele_p_shape1),
                ComputableShapeGut::Elementary(ele_p_shape2),
                ComputableShapeGut::Elementary(ele_p_shape3),
            ) => ElementaryPShape::pairwise_is_subset(
                &ElementaryPShape::pairwise_intersection(ele_p_shape1, ele_p_shape2),
                &ele_p_shape3,
            ),

            (
                ComputableShapeGut::Tuple(tuple_shapes1),
                ComputableShapeGut::Tuple(tuple_shapes2),
                ComputableShapeGut::Tuple(tuple_shapes3),
            ) => {
                if tuple_shapes1.len() != tuple_shapes2.len() {
                    true
                } else if tuple_shapes1.len() != tuple_shapes3.len() {
                    false
                } else {
                    tuple_shapes1
                        .into_iter()
                        .zip(tuple_shapes2.into_iter())
                        .zip(tuple_shapes3.into_iter())
                        .all(|((shape1, shape2), shape3)| {
                            self.meet_in_computable_shape_is_subshape(shape1, shape2, shape3)
                        })
                }
            }

            (
                ComputableShapeGut::Array(shape1),
                ComputableShapeGut::Array(shape2),
                ComputableShapeGut::Array(shape3),
            ) => self.meet_in_computable_shape_is_subshape(shape1, shape2, shape3),

            (
                ComputableShapeGut::Map(key1, value1),
                ComputableShapeGut::Map(key2, value2),
                ComputableShapeGut::Map(key3, value3),
            ) => {
                self.meet_in_computable_shape_is_subshape(key1, key2, key3)
                    && self.meet_in_computable_shape_is_subshape(value1, value2, value3)
            }

            (
                ComputableShapeGut::Struct(struct_case1),
                ComputableShapeGut::Struct(struct_case2),
                ComputableShapeGut::Struct(struct_case3),
            ) => struct_case3.into_iter().all(|(field_name, case_shape)| {
                if struct_case1.contains_key(&field_name) && struct_case2.contains_key(&field_name)
                {
                    self.meet_in_computable_shape_is_subshape(
                        *struct_case1.get(&field_name).unwrap(),
                        *struct_case2.get(&field_name).unwrap(),
                        case_shape,
                    )
                } else if struct_case1.contains_key(&field_name) {
                    self.in_computable_shape_is_subshape(
                        *struct_case1.get(&field_name).unwrap(),
                        case_shape,
                    )
                } else if struct_case2.contains_key(&field_name) {
                    self.in_computable_shape_is_subshape(
                        *struct_case2.get(&field_name).unwrap(),
                        case_shape,
                    )
                } else {
                    self.in_computable_shape_is_subshape(
                        self.ask_for_ecs_key(&ElementaryShape::Any).clone(),
                        case_shape,
                    )
                }
            }),

            (
                ComputableShapeGut::Function(args1, ret1),
                ComputableShapeGut::Function(args2, ret2),
                ComputableShapeGut::Function(args3, ret3),
            ) => {
                args1.len() != args2.len()
                    || args1.len() == args3.len()
                        && args1
                            .into_iter()
                            .zip(args2.into_iter())
                            .zip(args3.into_iter())
                            .map(|((shape1, shape2), shape3)| {
                                self.in_computable_shape_is_subshape(shape3, shape1)
                                    && self.in_computable_shape_is_subshape(shape3, shape2)
                            })
                            .all(|x| x)
                        && self.meet_in_computable_shape_is_subshape(ret1, ret2, ret3)
            }

            (ComputableShapeGut::Union(shape1_1, shape1_2), _, _) => {
                self.meet_in_computable_shape_is_subshape(
                    shape1_1,
                    potential_subshape2,
                    potential_supershape,
                ) && self.meet_in_computable_shape_is_subshape(
                    shape1_2,
                    potential_subshape2,
                    potential_supershape,
                )
            }

            (_, ComputableShapeGut::Union(shape2_1, shape2_2), _) => {
                self.meet_in_computable_shape_is_subshape(
                    potential_subshape1,
                    shape2_1,
                    potential_supershape,
                ) && self.meet_in_computable_shape_is_subshape(
                    potential_subshape1,
                    shape2_2,
                    potential_supershape,
                )
            }

            (_, _, ComputableShapeGut::Union(shape3_1, shape3_2)) => {
                self.meet_in_computable_shape_is_subshape(
                    potential_subshape1,
                    potential_subshape2,
                    shape3_1,
                ) || self.meet_in_computable_shape_is_subshape(
                    potential_subshape1,
                    potential_subshape2,
                    shape3_2,
                )
            }

            (ComputableShapeGut::Intersection(shape1_1, shape1_2), _, _) => {
                self.meet_in_computable_shape_is_subshape(
                    shape1_1,
                    potential_subshape2,
                    potential_supershape,
                ) && self.meet_in_computable_shape_is_subshape(
                    shape1_2,
                    potential_subshape2,
                    potential_supershape,
                )
            }

            (_, ComputableShapeGut::Intersection(shape2_1, shape2_2), _) => {
                self.meet_in_computable_shape_is_subshape(
                    potential_subshape1,
                    shape2_1,
                    potential_supershape,
                ) && self.meet_in_computable_shape_is_subshape(
                    potential_subshape1,
                    shape2_2,
                    potential_supershape,
                )
            }

            (_, _, ComputableShapeGut::Intersection(shape3_1, shape3_2)) => {
                self.meet_in_computable_shape_is_subshape(
                    potential_subshape1,
                    potential_subshape2,
                    shape3_1,
                ) && self.meet_in_computable_shape_is_subshape(
                    potential_subshape1,
                    potential_subshape2,
                    shape3_2,
                )
            }
        }
    }
}
