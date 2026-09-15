use crate::compiler::analyzer::common::route::route::Route;
use crate::compiler::analyzer::shape_def_resolution::description::error::FunctionDescriptionResolutionError;
use crate::compiler::analyzer::shape_def_resolution::description::shape_description::{
    ElementaryShape, ShapeDescription, ShapeDescriptionGut, ShapeVarianceAnnotation,
};
use crate::compiler::analyzer::shape_def_resolution::summary::{
    ClosureDescriptionKey, FunctionDescriptionContextKey, FunctionDescriptionKey,
    GenericParamDescriptionKey, GrandShapeContentSummaryAtWork, InClosureDescriptionContextKey,
    ShapeDescriptionContextKey, ShapeDescriptionKey,
};
use crate::compiler::parser::exprs::identifier::Identifier;
use crate::compiler::parser::exprs::{Expr, FunctionBlock, FunctionDef};
use crate::compiler::parser::parser::{ParsedBox, ParsedPosition};
use crate::compiler::parser::stmts::sequential_stmts::SequentialBlock;
use crate::compiler::parser::types::Type;
use crate::utility::common::vec_option::vo_to_ov;
use std::collections::HashMap;

pub struct FunctionDescriptionContext {
    pub module_route: Route,
    pub generic_params_key: Option<GenericParamDescriptionKey>,
}

pub struct InClosureDescriptionContext {
    pub parent_closure_description_context: Option<InClosureDescriptionContextKey>,
    pub module_route: Route,
    pub current_params: HashMap<String, ShapeDescriptionKey>,
}

pub enum ClosureDescriptionGut {
    HigherOrder(ClosureDescriptionKey),
    Closure {
        block: ParsedBox<SequentialBlock>,
        return_type: ShapeDescriptionKey,
    },
}

pub struct ClosureParameterDescription {
    pub name: String,
    pub shape_description_key: ShapeDescriptionKey,
    pub default: Option<ParsedBox<Expr>>,
}

pub struct ClosureDescription {
    pub params: Vec<ClosureParameterDescription>,
    pub gut: ClosureDescriptionGut,
    pub pos: ParsedPosition,
}

pub struct FunctionDescription {
    pub context_key: FunctionDescriptionContextKey,
    pub function_shape: ShapeDescriptionKey,
    pub closure: ClosureDescriptionKey,
    pub pos: ParsedPosition,
}

impl GrandShapeContentSummaryAtWork<'_> {
    pub fn function_description_from_pb_function_def(
        &mut self,
        function_description_context_key: FunctionDescriptionContextKey,
        shape_description_context_key: ShapeDescriptionContextKey,
        pb_function_def: ParsedBox<FunctionDef>,
    ) -> Option<FunctionDescriptionKey> {
        let function_pos = pb_function_def.position.clone();
        self.closure_description_from_pb_function_def(
            function_description_context_key,
            shape_description_context_key.clone(),
            pb_function_def,
        )
        .map(|closure| {
            let closure_description_key = self.new_closure_description(closure);

            let function_shape =
                self.extract_closure_shape(closure_description_key, shape_description_context_key);

            self.new_function_description(FunctionDescription {
                context_key: function_description_context_key,
                function_shape,
                closure: closure_description_key,
                pos: function_pos,
            })
        })
    }

    fn closure_description_from_pb_function_def(
        &mut self,
        function_description_context_key: FunctionDescriptionContextKey,
        shape_description_context_key: ShapeDescriptionContextKey,
        pb_function_def: ParsedBox<FunctionDef>,
    ) -> Option<ClosureDescription> {
        let function_pos = pb_function_def.position.clone();
        let function_def = pb_function_def.owned_value();
        self.function_parameter_from_pb_function_def_args(
            shape_description_context_key,
            function_def.args,
        )
        .and_then(|vec| {
            let params = vec
                .into_iter()
                .map(
                    |(param_name, param_type, param_default)| ClosureParameterDescription {
                        name: param_name,
                        shape_description_key: param_type,
                        default: param_default,
                    },
                )
                .collect::<Vec<_>>();
            let o_gut = match function_def.body.owned_value() {
                FunctionBlock::SequentialBlock(return_type, pb_block) => {
                    Some(ClosureDescriptionGut::Closure {
                        return_type: return_type
                            .and_then(|pb_return_type| {
                                self.shape_description_from_pb_type(
                                    shape_description_context_key,
                                    pb_return_type,
                                )
                            })
                            .unwrap_or_else(|| {
                                self.new_shape_description(ShapeDescription {
                                    context_key: shape_description_context_key,
                                    annotation: ShapeVarianceAnnotation::Covariant,
                                    gut: ShapeDescriptionGut::Elementary(ElementaryShape::Unit),
                                    pos: function_pos.clone(),
                                })
                            }),
                        block: pb_block,
                    })
                }
                FunctionBlock::FunctionDef(pb_function_def) => self
                    .closure_description_from_pb_function_def(
                        function_description_context_key,
                        shape_description_context_key,
                        pb_function_def,
                    )
                    .map(|closure_description| {
                        ClosureDescriptionGut::HigherOrder(
                            self.new_closure_description(closure_description),
                        )
                    }),
            };
            match o_gut {
                Some(gut) => Some(ClosureDescription {
                    params,
                    gut,
                    pos: function_pos,
                }),
                None => None,
            }
        })
    }

    fn function_parameter_from_pb_function_def_args(
        &mut self,
        shape_description_context_key: ShapeDescriptionContextKey,
        args: Vec<
            ParsedBox<(
                ParsedBox<Identifier>,
                ParsedBox<Type>,
                Option<ParsedBox<Expr>>,
            )>,
        >,
    ) -> Option<Vec<(String, ShapeDescriptionKey, Option<ParsedBox<Expr>>)>> {
        vo_to_ov(args.into_iter().fold(vec![], |mut params_acc, pb_arg| {
            let (pb_id, pb_type, pb_default) = pb_arg.owned_value();
            let id_pos = pb_id.position.clone();
            let id = pb_id.owned_value().name.clone();
            params_acc.push(
                if !params_acc
                    .iter()
                    .filter(|opt| match opt {
                        Some((prev_param_name, _, _)) => *prev_param_name == id,
                        None => false,
                    })
                    .collect::<Vec<_>>()
                    .is_empty()
                {
                    self.shape_description_from_pb_type(shape_description_context_key, pb_type)
                        .map(|shape_description| (id, shape_description, pb_default))
                } else {
                    self.add_error(
                        FunctionDescriptionResolutionError::param_name_duplication(id_pos, id)
                            .into(),
                    );

                    None
                },
            );

            params_acc
        }))
    }
}

impl GrandShapeContentSummaryAtWork<'_> {
    pub fn extract_closure_shape(
        &mut self,
        closure_description: ClosureDescriptionKey,
        shape_description_context_key: ShapeDescriptionContextKey,
    ) -> ShapeDescriptionKey {
        let closure_description = self.get_closure_description(closure_description);
        let position = closure_description.pos.clone();
        let params: Vec<ShapeDescriptionKey> = closure_description
            .params
            .iter()
            .map(|param| param.shape_description_key.clone())
            .collect();
        match &closure_description.gut {
            ClosureDescriptionGut::HigherOrder(next_closure_description) => {
                let sub_closure_type = self.extract_closure_shape(
                    next_closure_description.clone(),
                    shape_description_context_key.clone(),
                );
                self.new_shape_description(ShapeDescription {
                    context_key: shape_description_context_key.clone(),
                    annotation: ShapeVarianceAnnotation::Covariant,
                    gut: ShapeDescriptionGut::Function {
                        args: params,
                        ret: sub_closure_type,
                    },
                    pos: position.clone(),
                })
            }
            ClosureDescriptionGut::Closure {
                block: _,
                return_type,
            } => self.new_shape_description(ShapeDescription {
                context_key: shape_description_context_key.clone(),
                annotation: ShapeVarianceAnnotation::Covariant,
                gut: ShapeDescriptionGut::Function {
                    args: params,
                    ret: return_type.clone(),
                },
                pos: position.clone(),
            }),
        }
    }
}
