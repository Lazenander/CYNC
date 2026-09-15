use crate::compiler::analyzer::common::route::route::Route;
use crate::compiler::analyzer::shape_def_resolution::description::shape_description::ElementaryShape;
use crate::compiler::analyzer::shape_def_resolution::summary::{
    ComputableShapeKey, GenericParamDescriptionKey,
};
use crate::compiler::parser::parser::ParsedPosition;
use derivative::Derivative;
use std::collections::BTreeMap;

#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ElementaryPShape {
    pub gut: ElementaryShape,
    #[derivative(
        PartialEq = "ignore",
        PartialOrd = "ignore",
        Ord = "ignore",
        Hash = "ignore"
    )]
    pub position: Option<ParsedPosition>,
}

#[derive(Debug, Clone, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub enum UnresolvedPShapeGut {
    Specified(Route, Vec<ComputableShapeKey>),
    GenericVar(String, GenericParamDescriptionKey),
}

#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UnresolvedPShape {
    pub gut: UnresolvedPShapeGut,
    #[derivative(
        PartialEq = "ignore",
        PartialOrd = "ignore",
        Ord = "ignore",
        Hash = "ignore"
    )]
    pub position: Option<ParsedPosition>,
}

#[derive(Debug, Clone, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub enum ComputableShapeGut {
    Elementary(ElementaryPShape),
    Unresolved(UnresolvedPShape),
    ParenSingle(ComputableShapeKey),
    Tuple(Vec<ComputableShapeKey>),
    Array(ComputableShapeKey),
    Map(ComputableShapeKey, ComputableShapeKey),
    Struct(BTreeMap<String, ComputableShapeKey>),
    Function(Vec<ComputableShapeKey>, ComputableShapeKey),
    Union(ComputableShapeKey, ComputableShapeKey),
    Intersection(ComputableShapeKey, ComputableShapeKey),
}

#[derive(Derivative, Debug, Clone)]
#[derivative(PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ComputableShape {
    pub gut: ComputableShapeGut,
    #[derivative(
        PartialEq = "ignore",
        PartialOrd = "ignore",
        Ord = "ignore",
        Hash = "ignore"
    )]
    pub position: Option<ParsedPosition>,
}

impl ComputableShape {
    pub fn is_any(&self) -> bool {
        matches!(
            self,
            ComputableShape {
                gut: ComputableShapeGut::Elementary(ElementaryPShape {
                    gut: ElementaryShape::Any,
                    ..
                }),
                ..
            }
        )
    }

    pub fn is_empty(&self) -> bool {
        matches!(
            self,
            ComputableShape {
                gut: ComputableShapeGut::Elementary(ElementaryPShape {
                    gut: ElementaryShape::Empty,
                    ..
                }),
                ..
            }
        )
    }
}

impl ComputableShape {
    pub fn any() -> Self {
        ComputableShape {
            gut: ComputableShapeGut::Elementary(ElementaryPShape {
                gut: ElementaryShape::Any,
                position: None,
            }),
            position: None,
        }
    }

    pub fn empty() -> Self {
        ComputableShape {
            gut: ComputableShapeGut::Elementary(ElementaryPShape {
                gut: ElementaryShape::Empty,
                position: None,
            }),
            position: None,
        }
    }
}
