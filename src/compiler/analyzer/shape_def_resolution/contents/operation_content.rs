use crate::compiler::analyzer::common::route::route::{CanonicalRoute, Route};
use crate::compiler::analyzer::shape_def_resolution::description::function_description::FunctionDescriptionContext;
use crate::compiler::analyzer::shape_def_resolution::description::generic_param_description::GenericParamDescription;
use crate::compiler::analyzer::shape_def_resolution::description::shape_description::ShapeDescriptionContext;
use crate::compiler::analyzer::shape_def_resolution::shape_signature::GenericParamDef;
use crate::compiler::analyzer::shape_def_resolution::summary::{
    FunctionDescriptionContextKey, FunctionDescriptionKey, GenericParamDescriptionKey,
    GrandShapeContentSummaryAtWork, ShapeDescriptionContextKey,
};
use crate::compiler::parser::exprs::FunctionDef;
use crate::compiler::parser::parser::ParsedBox;
use crate::compiler::parser::stmts::definition_stmts::operation::OperationDef;
use crate::compiler::parser::stmts::definition_stmts::shape::ShapeDef;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct OperationContentHeadKey(Route);

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct OperationContent {
    pub key: OperationContentHeadKey,
    pub generic_params_key: Option<GenericParamDescriptionKey>,
    pub shape_def_context_key: ShapeDescriptionContextKey,
    pub function_des_context_key: FunctionDescriptionContextKey,
    pub function_des_key: FunctionDescriptionKey,
}

impl GrandShapeContentSummaryAtWork<'_> {
    pub fn new_operation(
        &mut self,
        module_route: Route,
        pb_parsed_operation: ParsedBox<OperationDef>,
    ) -> Option<OperationContentHeadKey> {
        let parsed_operation = pb_parsed_operation.owned_value();
        let operation_name = parsed_operation.name.owned_value().name;
        let this_operation_route = CanonicalRoute::chain_one(module_route.clone(), operation_name);
        let (generic_param_description_key, shape_context_key) = parsed_operation
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
        let function_des_context_key =
            self.new_function_description_context(FunctionDescriptionContext {
                module_route: module_route.clone(),
                generic_params_key: generic_param_description_key,
            });
        let function_des_key = self.function_description_from_pb_function_def(
            function_des_context_key,
            shape_context_key.clone(),
            parsed_operation.function_def,
        )?;
        let key = OperationContentHeadKey(this_operation_route);
        self.new_operation_content_head(
            key.clone(),
            OperationContent {
                key: key.clone(),
                generic_params_key: generic_param_description_key,
                shape_def_context_key: shape_context_key,
                function_des_context_key,
                function_des_key,
            },
        );

        Some(key)
    }
}
