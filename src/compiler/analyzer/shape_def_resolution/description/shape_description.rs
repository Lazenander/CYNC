use crate::compiler::analyzer::common::route::route::Route;
use crate::compiler::analyzer::global_name_resolution::def_signature::SkeletonDefSignatureKind;
use crate::compiler::analyzer::shape_def_resolution::description::error::ShapeDescriptionResolutionError;
use crate::compiler::analyzer::shape_def_resolution::lookup_route::error::LookUpRouteError;
use crate::compiler::analyzer::shape_def_resolution::lookup_route::lookup_route::RidToShapeRouteResult;
use crate::compiler::analyzer::shape_def_resolution::shape_signature::GenericParamDef;
use crate::compiler::analyzer::shape_def_resolution::summary::{
    GenericParamDescriptionKey, GrandShapeContentSummaryAtWork, ShapeDescriptionContextKey,
    ShapeDescriptionKey,
};
use crate::compiler::parser::parser::{ParsedBox, ParsedPosition};
use crate::compiler::parser::types::parenedtype::TupleType;
use crate::compiler::parser::types::structtype::{StructCase, StructType};
use crate::compiler::parser::types::templated::IdentifierType;
use crate::compiler::parser::types::{Type, TypeAnnotation, TypeGut};
use crate::utility::common::vec_option::vo_to_ov;
use phf::phf_map;
use std::collections::{BTreeMap, HashMap};
use std::convert::Into;
use std::iter::{IntoIterator, Iterator};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum IntWidth {
    UnsignedBit8,
    UnsignedBit16,
    UnsignedBit32,
    UnsignedBit64,
    UnsignedBit128,
    SignedBit16,
    SignedBit32,
    SignedBit64,
    SignedBit128,
    SignedBit256,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum FloatWidth {
    Bit16,
    Bit32,
    Bit64,
    Bit128,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum CharWidth {
    Ascii,
    UTF16,
}

#[derive(Debug, Clone, Eq, Hash, PartialEq, Ord, PartialOrd, Copy)]
pub enum ElementaryShape {
    Empty,
    Unit,
    Bool,
    Int(IntWidth),
    Float(FloatWidth),
    Char(CharWidth),
    String(CharWidth),
    Any,
}

#[derive(Debug, Clone, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub enum UnresolvedShape {
    GenericVariable(String),
    Specified(Route, Vec<ShapeDescriptionKey>),
}

#[derive(Debug, Clone)]
pub enum ShapeVarianceAnnotation {
    Covariant,
    Contravariant,
    Invariant,
}

impl From<TypeAnnotation> for ShapeVarianceAnnotation {
    fn from(annotation: TypeAnnotation) -> Self {
        match annotation {
            TypeAnnotation::Covariant => ShapeVarianceAnnotation::Covariant,
            TypeAnnotation::Contravariant => ShapeVarianceAnnotation::Contravariant,
            TypeAnnotation::Invariant => ShapeVarianceAnnotation::Invariant,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ShapeDescription {
    pub context_key: ShapeDescriptionContextKey,
    pub annotation: ShapeVarianceAnnotation,
    pub gut: ShapeDescriptionGut,
    pub pos: ParsedPosition,
}

#[derive(Debug, Clone, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub enum ShapeDescriptionGut {
    Elementary(ElementaryShape),
    Unresolved(UnresolvedShape),
    // Opaque(Route),
    ParenSingle(ShapeDescriptionKey),
    Tuple(Vec<ShapeDescriptionKey>),
    Array(ShapeDescriptionKey),
    Map {
        key: ShapeDescriptionKey,
        value: ShapeDescriptionKey,
    },
    Struct(BTreeMap<String, ShapeDescriptionKey>),
    Function {
        args: Vec<ShapeDescriptionKey>,
        ret: ShapeDescriptionKey,
    },

    Union(ShapeDescriptionKey, ShapeDescriptionKey),
    Intersection(ShapeDescriptionKey, ShapeDescriptionKey),
}

#[derive(Clone)]
pub struct ShapeDescriptionContext {
    pub module_route: Route,
    pub generic_param_key: Option<GenericParamDescriptionKey>,
    pub definition_pattern_variable_types: Option<HashMap<String, ShapeDescriptionKey>>,
}

pub const CORE_TYPE_NAME_MAP: phf::Map<&str, ElementaryShape> = phf_map! {
    "Empty" => ElementaryShape::Empty,
    "Unit" => ElementaryShape::Unit,
    "Bool" => ElementaryShape::Bool,
    "U8" => ElementaryShape::Int(IntWidth::UnsignedBit8),
    "U16" => ElementaryShape::Int(IntWidth::UnsignedBit16),
    "U32" => ElementaryShape::Int(IntWidth::UnsignedBit32),
    "U64" => ElementaryShape::Int(IntWidth::UnsignedBit64),
    "I16" => ElementaryShape::Int(IntWidth::SignedBit16),
    "I32" => ElementaryShape::Int(IntWidth::SignedBit32),
    "I64" => ElementaryShape::Int(IntWidth::SignedBit64),
    "I128" => ElementaryShape::Int(IntWidth::SignedBit128),
    "I256" => ElementaryShape::Int(IntWidth::SignedBit256),
    "F16" => ElementaryShape::Float(FloatWidth::Bit16),
    "F32" => ElementaryShape::Float(FloatWidth::Bit32),
    "F64" => ElementaryShape::Float(FloatWidth::Bit64),
    "F128" => ElementaryShape::Float(FloatWidth::Bit128),
    "ASCIIChar" => ElementaryShape::Char(CharWidth::Ascii),
    "UTF16Char" => ElementaryShape::Char(CharWidth::UTF16),
    "ASCIIString" => ElementaryShape::String(CharWidth::Ascii),
    "UTF16String" => ElementaryShape::String(CharWidth::UTF16),
    "Any" => ElementaryShape::Any,
};

impl GrandShapeContentSummaryAtWork<'_> {
    pub fn shape_description_from_pb_type(
        &mut self,
        shape_description_context_key: ShapeDescriptionContextKey,
        pb_type: ParsedBox<Type>,
    ) -> Option<ShapeDescriptionKey> {
        let shape_description_context: ShapeDescriptionContext = self
            .get_shape_description_context(shape_description_context_key)
            .clone();
        let pb_type_annotation: ShapeVarianceAnnotation = pb_type.value.annotation.clone().into();
        let pb_type_pos = pb_type.position.clone();
        let form_key = |s_self: &mut GrandShapeContentSummaryAtWork, gut| {
            s_self.new_shape_description(ShapeDescription {
                context_key: shape_description_context_key,
                annotation: pb_type_annotation.clone(),
                gut,
                pos: pb_type_pos.clone(),
            })
        };
        match pb_type.owned_value().gut {
            TypeGut::Unit => Some(form_key(
                self,
                ShapeDescriptionGut::Elementary(ElementaryShape::Unit),
            )),
            TypeGut::Identifier(id_type) => {
                let IdentifierType {
                    rid,
                    template_call,
                    //     paren_type,
                } = id_type;
                if rid.value().route.is_empty() && template_call.is_none() {
                    let potential_pattern_variable = rid.value().this_id.value().name.clone();
                    if let Some(ref definition_pattern_variable_types) =
                        shape_description_context.definition_pattern_variable_types
                    {
                        if definition_pattern_variable_types
                            .contains_key(&potential_pattern_variable)
                        {
                            return Some(
                                definition_pattern_variable_types
                                    .get(&potential_pattern_variable)
                                    .unwrap()
                                    .clone(),
                            );
                        }
                    }
                }
                if rid.value().route.is_empty()
                    && shape_description_context.generic_param_key.is_some()
                    && !self
                        .get_generic_param_description(
                            shape_description_context.generic_param_key.unwrap(),
                        )
                        .params
                        .iter()
                        .filter(|generic_variable| match generic_variable {
                            GenericParamDef::Variable(ref existed_generic_variable)
                            | GenericParamDef::Guarded(ref existed_generic_variable, _) => {
                                rid.value().this_id.value().name == *existed_generic_variable
                            }
                        })
                        .collect::<Vec<_>>()
                        .is_empty()
                {
                    return if template_call.is_some() {
                        self.add_error(
                            ShapeDescriptionResolutionError::generic_variable_template_call_detected(pb_type_pos).into(),
                        );
                        None
                    } else {
                        Some(form_key(
                            self,
                            ShapeDescriptionGut::Unresolved(UnresolvedShape::GenericVariable(
                                rid.owned_value().this_id.owned_value().name.clone(),
                            )),
                        ))
                    };
                }
                match self.lookup_def_kind_from_rid(
                    rid.clone(),
                    SkeletonDefSignatureKind::Shape,
                    shape_description_context.module_route.clone(),
                ) {
                    RidToShapeRouteResult::Ok(c_route) => template_call
                        .and_then(|template_call| {
                            self.unresolved_shape_from_pb_template_call(
                                shape_description_context_key,
                                template_call,
                            )
                        })
                        .map(|generic_args| {
                            form_key(
                                self,
                                ShapeDescriptionGut::Unresolved(UnresolvedShape::Specified(
                                    c_route,
                                    generic_args,
                                )),
                            )
                        }),
                    RidToShapeRouteResult::RouteExistsButKindDifferent {
                        route,
                        expected,
                        found,
                    } => {
                        self.add_error(
                            LookUpRouteError::route_exists_but_kind_different(
                                pb_type_pos,
                                route,
                                expected,
                                found,
                            )
                            .into(),
                        );
                        None
                    }
                    RidToShapeRouteResult::RouteNotFound => {
                        self.add_error(LookUpRouteError::route_not_found(pb_type_pos).into());
                        None
                    }
                    RidToShapeRouteResult::UnknownRootRouteFraction => {
                        if rid.value().route.is_empty()
                            && CORE_TYPE_NAME_MAP
                                .contains_key(rid.value().this_id.value().name.as_str())
                        {
                            return if template_call.is_some() {
                                self.add_error(
                                    ShapeDescriptionResolutionError::core_type_template_call_detected(pb_type_pos).into(),
                                );
                                None
                            } else {
                                Some(form_key(
                                    self,
                                    ShapeDescriptionGut::Elementary(
                                        CORE_TYPE_NAME_MAP
                                            .get(rid.value().this_id.value().name.as_str())
                                            .unwrap()
                                            .clone(),
                                    ),
                                ))
                            };
                        }
                        self.add_error(
                            LookUpRouteError::unknown_root_route_fraction(pb_type_pos).into(),
                        );
                        None
                    }
                    RidToShapeRouteResult::SingleCellOrRelativeKeywordRouteNotAllowed => {
                        self.add_error(
                            LookUpRouteError::single_cell_or_relative_keyword_route_not_allowed(
                                pb_type_pos,
                            )
                            .into(),
                        );
                        None
                    }
                }
            }
            TypeGut::ParenSingle(pb_paren_type) => self
                .shape_description_from_pb_type(shape_description_context_key, pb_paren_type)
                .map(|paren_type| form_key(self, ShapeDescriptionGut::ParenSingle(paren_type))),
            TypeGut::Tuple(TupleType { entries }) => vo_to_ov(
                entries
                    .into_iter()
                    .map(|pb_type| {
                        self.shape_description_from_pb_type(shape_description_context_key, pb_type)
                    })
                    .collect(),
            )
            .map(|tuple_shapes| form_key(self, ShapeDescriptionGut::Tuple(tuple_shapes))),
            TypeGut::Array(pb_arr_type) => self
                .shape_description_from_pb_type(shape_description_context_key, pb_arr_type)
                .map(|array_shape| form_key(self, ShapeDescriptionGut::Array(array_shape))),
            TypeGut::Map(pb_key_type, pb_value_type) => {
                let key_shape =
                    self.shape_description_from_pb_type(shape_description_context_key, pb_key_type);
                let value_shape = self
                    .shape_description_from_pb_type(shape_description_context_key, pb_value_type);
                if let (Some(key), Some(value)) = (key_shape, value_shape) {
                    Some(form_key(self, ShapeDescriptionGut::Map { key, value }))
                } else {
                    None
                }
            }
            TypeGut::Struct(StructType { entries }) => {
                let (struct_cases, is_err) = entries
                    .into_iter()
                    .fold((BTreeMap::new(), false), |(mut acc, is_err), pb_struct_case| {
                        let StructCase { id, a_type, default, } = pb_struct_case.owned_value();
                        let field_pos = id.position.clone();
                        let field_name = id.owned_value().name;
                        let mut this_is_err = false;
                        if acc.contains_key(&field_name) {
                            self.add_error(
                                ShapeDescriptionResolutionError::struct_field_name_duplication(field_pos.clone(), field_name.clone()).into(),
                            );
                            this_is_err = true;
                        }
                        match (a_type, default) {
                            (Some(a_type), None) => {
                                if let Some(shape_description) = self.shape_description_from_pb_type(shape_description_context_key, a_type) {
                                    acc.insert(field_name.clone(), shape_description);
                                }
                            }
                            (None, Some(pattern_variable)) => {
                                match shape_description_context.definition_pattern_variable_types {
                                    Some(ref definition_pattern_variable_types) => {
                                        let pattern_variable_name = pattern_variable.owned_value().name;
                                        match definition_pattern_variable_types.get(&pattern_variable_name) {
                                            None => {
                                                self.add_error(
                                                    ShapeDescriptionResolutionError::unknown_pattern_variable(field_pos, pattern_variable_name).into(),
                                                );
                                                this_is_err = true
                                            }
                                            Some(shape_description) => {
                                                acc.insert(field_name.clone(), shape_description.clone());
                                            }
                                        }
                                    }
                                    None => {
                                        self.add_error(
                                            ShapeDescriptionResolutionError::unexpected_assignment_syntax_sugar_used(field_pos).into(),
                                        );
                                        this_is_err = true
                                    }
                                }
                            }
                            _ => unreachable!()
                        }
                        (acc, this_is_err || is_err)
                    });
                if is_err {
                    None
                } else {
                    Some(form_key(self, ShapeDescriptionGut::Struct(struct_cases)))
                }
            }
            TypeGut::Function(arg_types, ret_type) => {
                let arg_shape_defs = vo_to_ov(
                    arg_types
                        .into_iter()
                        .map(|arg_type| {
                            self.shape_description_from_pb_type(
                                shape_description_context_key,
                                arg_type,
                            )
                        })
                        .collect(),
                );

                let ret_shape_def =
                    self.shape_description_from_pb_type(shape_description_context_key, ret_type);
                if let (Some(arg_shape_defs), Some(ret_shape_def)) = (arg_shape_defs, ret_shape_def)
                {
                    Some(form_key(
                        self,
                        ShapeDescriptionGut::Function {
                            args: arg_shape_defs,
                            ret: ret_shape_def,
                        },
                    ))
                } else {
                    None
                }
            }
            TypeGut::Union(pb_type1, pb_type2) => {
                let shape1 =
                    self.shape_description_from_pb_type(shape_description_context_key, pb_type1);
                let shape2 =
                    self.shape_description_from_pb_type(shape_description_context_key, pb_type2);
                if let (Some(shape1), Some(shape2)) = (shape1, shape2) {
                    Some(form_key(self, ShapeDescriptionGut::Union(shape1, shape2)))
                } else {
                    None
                }
            }
            TypeGut::Intersection(pb_type1, pb_type2) => {
                let shape1 =
                    self.shape_description_from_pb_type(shape_description_context_key, pb_type1);
                let shape2 =
                    self.shape_description_from_pb_type(shape_description_context_key, pb_type2);
                if let (Some(shape1), Some(shape2)) = (shape1, shape2) {
                    Some(form_key(
                        self,
                        ShapeDescriptionGut::Intersection(shape1, shape2),
                    ))
                } else {
                    None
                }
            }
        }
    }
}
