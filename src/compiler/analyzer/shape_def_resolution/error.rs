use crate::compiler::analyzer::shape_def_resolution::contents::error::ContentResolutionError;
use crate::compiler::analyzer::shape_def_resolution::description::error::{
    FunctionDescriptionResolutionError, ShapeDescriptionResolutionError,
};
use crate::compiler::analyzer::shape_def_resolution::lookup_route::error::LookUpRouteError;
use crate::compiler::analyzer::shape_def_resolution::shape_content_check::error::ShapeContentCheckError;
use crate::compiler::analyzer::shape_def_resolution::shape_graph::error::ShapeGraphError;
use crate::compiler::parser::parser::ParsedPosition;
use crate::utility::common::vec_add::AddableVec;
use std::fmt;

#[derive(Debug, Clone)]
pub enum ShapeDefinitionResolutionErrorKind {
    LookUpRoute(LookUpRouteError),
    ShapeDescription(ShapeDescriptionResolutionError),
    ContentResolution(ContentResolutionError),
    FunctionDescription(FunctionDescriptionResolutionError),
    ShapeContentCheck(ShapeContentCheckError),
    ShapeGraph(ShapeGraphError),
}

#[derive(Debug, Clone)]
pub struct ShapeDefinitionResolutionError {
    pub kind: ShapeDefinitionResolutionErrorKind,
    pub position: ParsedPosition,
}

impl fmt::Display for ShapeDefinitionResolutionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            ShapeDefinitionResolutionErrorKind::LookUpRoute(error) => {
                write!(f, "{}", error)
            }
            ShapeDefinitionResolutionErrorKind::ShapeDescription(error) => {
                write!(f, "{}", error)
            }
            ShapeDefinitionResolutionErrorKind::ContentResolution(error) => {
                write!(f, "{}", error)
            }
            ShapeDefinitionResolutionErrorKind::FunctionDescription(error) => {
                write!(f, "{}", error)
            }
            ShapeDefinitionResolutionErrorKind::ShapeGraph(error) => {
                write!(f, "{}", error)
            }
            ShapeDefinitionResolutionErrorKind::ShapeContentCheck(error) => {
                write!(f, "{}", error)
            }
        }
    }
}

impl From<LookUpRouteError> for ShapeDefinitionResolutionError {
    fn from(error: LookUpRouteError) -> Self {
        ShapeDefinitionResolutionError {
            position: error.position.clone(),
            kind: ShapeDefinitionResolutionErrorKind::LookUpRoute(error),
        }
    }
}

impl From<ContentResolutionError> for ShapeDefinitionResolutionError {
    fn from(error: ContentResolutionError) -> Self {
        ShapeDefinitionResolutionError {
            position: error.position.clone(),
            kind: ShapeDefinitionResolutionErrorKind::ContentResolution(error),
        }
    }
}

impl From<ShapeDescriptionResolutionError> for ShapeDefinitionResolutionError {
    fn from(error: ShapeDescriptionResolutionError) -> Self {
        ShapeDefinitionResolutionError {
            position: error.position.clone(),
            kind: ShapeDefinitionResolutionErrorKind::ShapeDescription(error),
        }
    }
}

impl From<FunctionDescriptionResolutionError> for ShapeDefinitionResolutionError {
    fn from(error: FunctionDescriptionResolutionError) -> Self {
        ShapeDefinitionResolutionError {
            position: error.position.clone(),
            kind: ShapeDefinitionResolutionErrorKind::FunctionDescription(error),
        }
    }
}

impl From<ShapeContentCheckError> for ShapeDefinitionResolutionError {
    fn from(error: ShapeContentCheckError) -> Self {
        ShapeDefinitionResolutionError {
            position: error.position.clone(),
            kind: ShapeDefinitionResolutionErrorKind::ShapeContentCheck(error),
        }
    }
}

impl From<ShapeGraphError> for ShapeDefinitionResolutionError {
    fn from(error: ShapeGraphError) -> Self {
        ShapeDefinitionResolutionError {
            position: error.position.clone(),
            kind: ShapeDefinitionResolutionErrorKind::ShapeGraph(error),
        }
    }
}

pub type ShapeDefinitionResolutionErrors = AddableVec<ShapeDefinitionResolutionError>;
