use crate::compiler::analyzer::shape_def_resolution::shape_content_check::error::ShapeContentCheckError;
use crate::compiler::analyzer::shape_def_resolution::shape_graph::error::ShapeGraphError;
use crate::compiler::analyzer::shape_def_resolution::shape_graph::subshape::subshape::IsSubShapeResult;
use crate::compiler::analyzer::shape_def_resolution::summary::GrandShapeContentSummaryAtWork;

pub mod error;
pub mod task_accumulation;

impl GrandShapeContentSummaryAtWork<'_> {
    pub fn check_shape_content_generic(&mut self) {
        let shape_content_head_keys = self.gsc_summary.shape_content_heads.iter().fold(
            vec![],
            |mut acc, (shape_content_head_key, _)| {
                acc.push(shape_content_head_key.clone());
                acc
            },
        );
        let guard_tasks =
            shape_content_head_keys
                .into_iter()
                .fold(vec![], |acc, shape_content_head_key| {
                    acc.into_iter()
                        .chain(
                            self.does_shape_generic_size_fit(shape_content_head_key.clone())
                                .into_iter(),
                        )
                        .collect()
                });
        guard_tasks.iter().for_each(
            |(
                this_shape_head_key,
                guard_shape_head_key,
                shape_description_key,
                guard_shape_description_key,
            )| {
                let shape_description = self.get_shape_description(*shape_description_key).clone();
                let guard_shape_description = self
                    .get_shape_description(*guard_shape_description_key)
                    .clone();
                match self.shape_description_is_subshape(
                    shape_description_key,
                    guard_shape_description_key,
                ) {
                    IsSubShapeResult::True => (),
                    IsSubShapeResult::False => self.add_error(
                        ShapeContentCheckError::generic_guard_fails(
                            shape_description.pos,
                            this_shape_head_key.to_string(),
                            guard_shape_head_key.to_string(),
                        )
                        .into(),
                    ),
                    IsSubShapeResult::Stuck => self.add_error(
                        ShapeGraphError::is_sub_shape_stuck(
                            guard_shape_description.pos,
                            self.display_shape_definition(shape_description_key),
                            self.display_shape_definition(guard_shape_description_key),
                        )
                        .into(),
                    ),
                    IsSubShapeResult::OverCost => self.add_error(
                        ShapeGraphError::is_sub_shape_over_cost(
                            guard_shape_description.pos,
                            self.display_shape_definition(shape_description_key),
                            self.display_shape_definition(guard_shape_description_key),
                        )
                        .into(),
                    ),
                }
            },
        );
    }
}
