use crate::compiler::analyzer::common::route::route::Route;
use crate::compiler::analyzer::shape_def_resolution::contents::error::ContentResolutionError;
use crate::compiler::analyzer::shape_def_resolution::description::generic_param_description::GenericParamDescription;
use crate::compiler::analyzer::shape_def_resolution::description::shape_description::ShapeDescriptionContext;
use crate::compiler::analyzer::shape_def_resolution::shape_signature::GenericParamDef;
use crate::compiler::analyzer::shape_def_resolution::summary::{
    GenericParamDescriptionKey, GrandShapeContentSummaryAtWork, ShapeDescriptionContextKey,
};
use crate::compiler::parser::parser::ParsedBox;
use crate::compiler::parser::template::TemplateDef;

impl<'a> GrandShapeContentSummaryAtWork<'a> {
    pub fn unresolved_shape_from_pb_template_def(
        &mut self,
        module_route: Route,
        pb_template_call: ParsedBox<TemplateDef>,
    ) -> Option<(GenericParamDescriptionKey, ShapeDescriptionContextKey)> {
        let mut generic_param_description_key =
            self.new_generic_param_description(GenericParamDescription { params: vec![] });
        let mut shape_description_context_key =
            self.new_shape_description_context(ShapeDescriptionContext {
                module_route: module_route.clone(),
                generic_param_key: Some(generic_param_description_key),
                definition_pattern_variable_types: None,
            });
        let mut is_err = false;
        for generic_arg in pb_template_call.owned_value().args {
            let (pb_id, o_pb_guard) = generic_arg.owned_value();
            let this_var_name_pos = pb_id.position.clone();
            let this_var_name = pb_id.owned_value().name;
            if self.map_generic_param_description(generic_param_description_key, |gp_dp| {
                !gp_dp
                    .params
                    .iter()
                    .filter(|arg| match arg {
                        GenericParamDef::Variable(var_name)
                        | GenericParamDef::Guarded(var_name, _) => var_name == &this_var_name,
                    })
                    .collect::<Vec<_>>()
                    .is_empty()
            }) {
                is_err = true;
                self.add_error(
                    ContentResolutionError::duplicate_generic_variable_definition(
                        this_var_name_pos,
                        this_var_name,
                    )
                    .into(),
                )
            } else {
                let o_guard_type =
                    match o_pb_guard {
                        None => None,
                        Some(pb_guard) => Some(self.shape_description_from_pb_type(
                            shape_description_context_key,
                            pb_guard,
                        )?),
                    };

                generic_param_description_key = self.new_generic_param_description(
                    self.map_generic_param_description(generic_param_description_key, |gp_dp| {
                        GenericParamDescription {
                            params: gp_dp
                                .params
                                .clone()
                                .into_iter()
                                .chain(vec![match o_guard_type {
                                    None => GenericParamDef::Variable(this_var_name),
                                    Some(guard_type) => {
                                        GenericParamDef::Guarded(this_var_name, guard_type)
                                    }
                                }])
                                .collect(),
                        }
                    }),
                );
                shape_description_context_key =
                    self.new_shape_description_context(ShapeDescriptionContext {
                        module_route: module_route.clone(),
                        generic_param_key: Some(generic_param_description_key),
                        definition_pattern_variable_types: None,
                    });
            }
        }
        if is_err {
            None
        } else {
            Some((generic_param_description_key, shape_description_context_key))
        }
    }
}
