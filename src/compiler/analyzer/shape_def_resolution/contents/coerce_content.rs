use crate::compiler::analyzer::common::route::route::Route;
use crate::compiler::analyzer::shape_def_resolution::contents::shape_content::ShapeContentHeadKey;
use crate::compiler::analyzer::shape_def_resolution::description::function_description::FunctionDescriptionContext;
use crate::compiler::analyzer::shape_def_resolution::description::shape_description::{
    ShapeDescriptionContext, ShapeDescriptionGut,
};
use crate::compiler::analyzer::shape_def_resolution::shape_signature::SpecifiedShapeDefSignature;
use crate::compiler::analyzer::shape_def_resolution::summary::{
    FunctionDescriptionContextKey, FunctionDescriptionKey, GenericParamDescriptionKey,
    GrandShapeContentSummaryAtWork, ShapeDescriptionContextKey,
};
use crate::compiler::parser::parser::ParsedBox;
use crate::compiler::parser::stmts::definition_stmts::coerce::Coerce;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct CoerceContentHeadKey {
    pub from_shape: ShapeContentHeadKey,
    pub to_shape: ShapeContentHeadKey,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct CoerceDefSignature {
    pub from_shape: SpecifiedShapeDefSignature,
    pub to_shape: SpecifiedShapeDefSignature,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct CoerceContent {
    pub signature: CoerceDefSignature,
    pub generic_params_key: Option<GenericParamDescriptionKey>,
    pub shape_def_context_key: ShapeDescriptionContextKey,
    pub function_des_context_key: FunctionDescriptionContextKey,
    pub function_des_key: FunctionDescriptionKey,
}

impl GrandShapeContentSummaryAtWork<'_> {
    pub fn display_coerce_def_signature(
        &self,
        coerce_def_signature: &CoerceDefSignature,
    ) -> String {
        format!(
            "{} --> {}",
            self.display_specified_shape_def_signature(&coerce_def_signature.from_shape),
            self.display_specified_shape_def_signature(&coerce_def_signature.to_shape)
        )
    }
}

impl GrandShapeContentSummaryAtWork<'_> {
    pub fn new_coerce(
        &mut self,
        module_route: Route,
        pb_parsed_coerce: ParsedBox<Coerce>,
    ) -> Option<CoerceContentHeadKey> {
        let parsed_coerce = pb_parsed_coerce.owned_value();
        let template_def = parsed_coerce.template_def;
        let generic_params_key;
        let shape_def_context_key;
        match template_def.and_then(|template_def| {
            self.unresolved_shape_from_pb_template_def(module_route.clone(), template_def)
        }) {
            Some((generic_params_key_piece, shape_def_context_key_piece)) => {
                generic_params_key = Some(generic_params_key_piece);
                shape_def_context_key = shape_def_context_key_piece;
            }
            None => {
                generic_params_key = None;
                shape_def_context_key =
                    self.new_shape_description_context(ShapeDescriptionContext {
                        module_route: module_route.clone(),
                        generic_param_key: None,
                        definition_pattern_variable_types: None,
                    })
            }
        }
        let function_des_context_key =
            self.new_function_description_context(FunctionDescriptionContext {
                module_route: module_route.clone(),
                generic_params_key,
            });
        let function_des_key = self.function_description_from_pb_function_def(
            function_des_context_key,
            shape_def_context_key,
            parsed_coerce.function_def,
        )?;
        let from_shape = self.extract_binded_shape(
            self.map_function_description(function_des_key, |function_des| {
                function_des.function_shape.clone()
            }),
        )?;
        let to_shape_key = self.map_function_description(function_des_key, |function_des| {
            self.map_shape_description(function_des.function_shape, |shape_description| {
                if let ShapeDescriptionGut::Function { args: _, ret } = shape_description.gut {
                    Some(ret)
                } else {
                    None
                }
            })
        })?;
        let to_shape = self.extract_specified_shape(to_shape_key)?;
        let signature = CoerceDefSignature {
            from_shape: from_shape.clone(),
            to_shape: to_shape.clone(),
        };
        let key = CoerceContentHeadKey {
            from_shape: from_shape.into(),
            to_shape: to_shape.into(),
        };

        self.insert_coerce_content_head(
            key.clone(),
            CoerceContent {
                signature,
                generic_params_key,
                shape_def_context_key,
                function_des_context_key,
                function_des_key,
            },
        );

        Some(key)
    }
}
