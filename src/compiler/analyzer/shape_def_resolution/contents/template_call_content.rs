use crate::compiler::analyzer::common::route::route::Route;
use crate::compiler::analyzer::shape_def_resolution::description::shape_description::{
    ShapeDescription, ShapeDescriptionContext,
};
use crate::compiler::analyzer::shape_def_resolution::summary::{
    GrandShapeContentSummaryAtWork, ShapeDescriptionContextKey, ShapeDescriptionKey,
};
use crate::compiler::parser::parser::ParsedBox;
use crate::compiler::parser::template::TemplateCall;
use crate::utility::common::vec_option::vo_to_ov;

impl<'a> GrandShapeContentSummaryAtWork<'a> {
    pub fn unresolved_shape_from_pb_template_call(
        &mut self,
        shape_description_context_key: ShapeDescriptionContextKey,
        pb_template_call: ParsedBox<TemplateCall>,
    ) -> Option<Vec<ShapeDescriptionKey>> {
        vo_to_ov(
            pb_template_call
                .owned_value()
                .args
                .into_iter()
                .map(|arg| self.shape_description_from_pb_type(shape_description_context_key, arg))
                .collect(),
        )
    }
}
