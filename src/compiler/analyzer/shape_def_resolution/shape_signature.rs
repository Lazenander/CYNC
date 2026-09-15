use crate::compiler::analyzer::common::route::route::Route;
use crate::compiler::analyzer::shape_def_resolution::contents::shape_content::ShapeContentHeadKey;
use crate::compiler::analyzer::shape_def_resolution::description::shape_description::ElementaryShape;
use crate::compiler::analyzer::shape_def_resolution::summary::{
    GrandShapeContentSummaryAtWork, ShapeDescriptionKey,
};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum GenericParamDef {
    Variable(String),
    Guarded(String, ShapeDescriptionKey),
}

impl GenericParamDef {
    pub fn get_name(&self) -> String {
        match self {
            GenericParamDef::Variable(name) => name.clone(),
            GenericParamDef::Guarded(name, _) => name.clone(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum SpecifiedGenericParam {
    Variable(String),
    // Guarded(String, ShapeDescriptionKey),
    Specified(ShapeDescriptionKey),
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum SpecifiedShapeDefSignature {
    Elementary(ElementaryShape),
    Custom {
        route: Route,
        params: Vec<SpecifiedGenericParam>,
    },
}

impl From<SpecifiedShapeDefSignature> for ShapeContentHeadKey {
    fn from(signature: SpecifiedShapeDefSignature) -> ShapeContentHeadKey {
        match signature {
            SpecifiedShapeDefSignature::Elementary(elementary_shape) => {
                ShapeContentHeadKey::Prelude(elementary_shape)
            }
            SpecifiedShapeDefSignature::Custom { route, params: _ } => {
                ShapeContentHeadKey::Custom(route)
            }
        }
    }
}

impl GrandShapeContentSummaryAtWork<'_> {
    fn display_specified_generic_param(
        &self,
        specified_generic_param: &SpecifiedGenericParam,
    ) -> String {
        match specified_generic_param {
            SpecifiedGenericParam::Variable(g_var) => {
                format!("{}", g_var)
            } /*
            SpecifiedGenericParam::Guarded(g_var, guard) => {
            write!(f, "{}: ", g_var)?;
            self.display_shape_definition(f, *guard)
            }*/
            SpecifiedGenericParam::Specified(specified) => self.display_shape_definition(specified),
        }
    }
}

impl GrandShapeContentSummaryAtWork<'_> {
    pub fn display_specified_shape_def_signature(
        &self,
        specified_shape_def_signature: &SpecifiedShapeDefSignature,
    ) -> String {
        match specified_shape_def_signature {
            SpecifiedShapeDefSignature::Elementary(elementary_shape) => {
                format!("{}", elementary_shape)
            }
            SpecifiedShapeDefSignature::Custom { route, params } => {
                if params.is_empty() {
                    format!("{}", route)
                } else {
                    format!(
                        "{}<{}>",
                        route,
                        params
                            .iter()
                            .map(|p| self.display_specified_generic_param(p))
                            .collect::<Vec<_>>()
                            .join(", ")
                    )
                }
            }
        }
    }
}
