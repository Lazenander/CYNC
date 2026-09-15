use crate::compiler::analyzer::common::route::route::{CanonicalRoute, Route};
use crate::compiler::analyzer::shape_def_resolution::contents::error::ContentResolutionError;
use crate::compiler::analyzer::shape_def_resolution::contents::shape_content::ShapeContentHeadKey;
use crate::compiler::analyzer::shape_def_resolution::description::function_description::FunctionDescriptionContext;
use crate::compiler::analyzer::shape_def_resolution::description::shape_description::{
    ShapeDescriptionContext, ShapeDescriptionGut, UnresolvedShape,
};
use crate::compiler::analyzer::shape_def_resolution::shape_signature::{
    GenericParamDef, SpecifiedGenericParam, SpecifiedShapeDefSignature,
};
use crate::compiler::analyzer::shape_def_resolution::summary::{
    FunctionDescriptionContextKey, FunctionDescriptionKey, GenericParamDescriptionKey,
    GrandShapeContentSummaryAtWork, ShapeDescriptionContextKey, ShapeDescriptionKey,
};
use crate::compiler::parser::parser::ParsedBox;
use crate::compiler::parser::stmts::definition_stmts::bind::BindDef;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct BindContentHeadKey(Route);

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct BindContent {
    pub key: BindContentHeadKey,
    pub generic_params_key: Option<GenericParamDescriptionKey>,
    pub shape_def_context_key: ShapeDescriptionContextKey,
    pub function_des_context_key: FunctionDescriptionContextKey,
    pub function_des_key: FunctionDescriptionKey,
    pub binded_shape: SpecifiedShapeDefSignature,
}

impl GrandShapeContentSummaryAtWork<'_> {
    pub fn new_bind(
        &mut self,
        module_route: Route,
        pb_parsed_bind: ParsedBox<BindDef>,
    ) -> Option<BindContentHeadKey> {
        let parsed_bind = pb_parsed_bind.owned_value();
        let template_def = parsed_bind.template_def;
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
        let key = BindContentHeadKey(CanonicalRoute::chain_one(
            module_route.clone(),
            parsed_bind.name.owned_value().name.clone(),
        ));
        let function_des_context_key =
            self.new_function_description_context(FunctionDescriptionContext {
                module_route: module_route.clone(),
                generic_params_key,
            });
        let function_des_key = self.function_description_from_pb_function_def(
            function_des_context_key,
            shape_def_context_key,
            parsed_bind.function_def,
        )?;
        let binded_shape = self.extract_binded_shape(
            self.map_function_description(function_des_key, |function_des| {
                function_des.function_shape.clone()
            }),
        )?;

        self.gsc_summary
            .bind_name_table
            .get_mut(&match &binded_shape {
                SpecifiedShapeDefSignature::Elementary(ele_shape) => {
                    ShapeContentHeadKey::Prelude(ele_shape.clone())
                }
                SpecifiedShapeDefSignature::Custom { route, .. } => {
                    ShapeContentHeadKey::Custom(route.clone())
                }
            })
            .unwrap()
            .entry(key.0.get_last_name())
            .or_insert(vec![])
            .push(key.clone());

        self.new_bind_content_head(
            key.clone(),
            BindContent {
                key: key.clone(),
                generic_params_key,
                shape_def_context_key,
                function_des_context_key,
                function_des_key,
                binded_shape,
            },
        );

        Some(key)
    }

    pub fn extract_binded_shape(
        &mut self,
        closure_shape_des_key: ShapeDescriptionKey,
    ) -> Option<SpecifiedShapeDefSignature> {
        let closure_shape_des = self.get_shape_description(closure_shape_des_key);
        let shape_des_context = self.get_shape_description_context(closure_shape_des.context_key);
        match &closure_shape_des.gut {
            ShapeDescriptionGut::Function { args, ret: _ } => {
                if args.len() == 1 {
                    self.extract_specified_shape(args.first().unwrap().clone())
                } else {
                    self.add_error(
                        ContentResolutionError::bind_operation_type_mismatch(
                            closure_shape_des.pos.clone(),
                        )
                        .into(),
                    );
                    None
                }
            }
            _ => unreachable!(),
        }
    }

    pub fn extract_specified_shape(
        &mut self,
        shape_des_key: ShapeDescriptionKey,
    ) -> Option<SpecifiedShapeDefSignature> {
        let shape_description = self.get_shape_description(shape_des_key);
        let shape_des_context = self.get_shape_description_context(shape_description.context_key);
        match &shape_description.gut {
            ShapeDescriptionGut::Elementary(elementary_shape) => Some(
                SpecifiedShapeDefSignature::Elementary(elementary_shape.clone()),
            ),
            ShapeDescriptionGut::Unresolved(unresolved_shape) => {
                match unresolved_shape {
                    UnresolvedShape::GenericVariable(_) => {
                        self.add_error(
                            ContentResolutionError::invalid_bind_generic_variable(
                                shape_description.pos.clone(),
                            )
                            .into(),
                        );
                        None
                    }
                    UnresolvedShape::Specified(route, generics_shapes) => {
                        let o_specified_generics_shapes = generics_shapes.iter().fold(
                        Some(vec![]),
                        |mut acc, generic_param_call| {
                            let shape_description = self.get_shape_description(*generic_param_call);
                            match &shape_description.gut {
                                ShapeDescriptionGut::Unresolved(UnresolvedShape::GenericVariable(gen_var_name)) => {
                                    self.map_generic_param_description(shape_des_context.generic_param_key.unwrap(),
                                                                       |generic_param_desc| {
                                                                           let res = generic_param_desc.params.iter().find_map(|generic_param_def| {
                                                                               match generic_param_def {
                                                                                   GenericParamDef::Variable(var_name) => {
                                                                                       if var_name == gen_var_name {
                                                                                           Some(SpecifiedGenericParam::Variable(var_name.clone()))
                                                                                       } else {
                                                                                           None
                                                                                       }
                                                                                   }
                                                                                   GenericParamDef::Guarded(var_name, guard) => {
                                                                                       if var_name == gen_var_name {
                                                                                           // Some(SpecifiedGenericParam::Guarded(var_name.clone(), *guard))
                                                                                           Some(SpecifiedGenericParam::Specified(*guard))
                                                                                       } else {
                                                                                           None
                                                                                       }
                                                                                   }
                                                                               }
                                                                           });
                                                                           Some(res.unwrap())
                                                                       }
                                    ).unwrap();
                                }
                                _ => {
                                    if let Some(acc_vec) = acc.as_mut() {
                                        acc_vec.push(SpecifiedGenericParam::Specified(*generic_param_call));
                                    }
                                }
                            }
                            acc
                        }
                    );
                        o_specified_generics_shapes.map(|specified_generics_shapes| {
                            SpecifiedShapeDefSignature::Custom {
                                route: route.clone(),
                                params: specified_generics_shapes,
                            }
                        })
                    }
                }
            }
            _ => {
                self.add_error(
                    ContentResolutionError::invalid_bind_type(shape_description.pos.clone()).into(),
                );
                None
            }
        }
    }
}
