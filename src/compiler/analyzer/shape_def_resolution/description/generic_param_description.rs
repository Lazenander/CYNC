use crate::compiler::analyzer::shape_def_resolution::shape_signature::GenericParamDef;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct GenericParamDescription {
    pub params: Vec<GenericParamDef>,
}
