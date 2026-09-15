use crate::compiler::analyzer::common::route::route::Route;
use crate::compiler::parser::parser::ParsedPosition;
use crate::utility::common::vec_add::AddableVec;
use std::fmt;

#[derive(Debug, Clone)]
pub enum ShapeContentCheckErrorKind {
    GenericParamSizeIncompatible {
        expected: usize,
        found: usize,
        shape_route: Route,
    },
    GenericGuardFails {
        this_content_name: String,
        that_content_name: String,
    },
}

#[derive(Debug, Clone)]
pub struct ShapeContentCheckError {
    pub kind: ShapeContentCheckErrorKind,
    pub position: ParsedPosition,
}

impl fmt::Display for ShapeContentCheckError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            ShapeContentCheckErrorKind::GenericParamSizeIncompatible {
                shape_route,
                expected,
                found,
            } => {
                write!(
                    f,
                    "[{}] The shape {} is defined with {} generic params, but found {}.",
                    self.position.print_path_line_column_span(),
                    shape_route,
                    expected,
                    found,
                )
            }

            ShapeContentCheckErrorKind::GenericGuardFails {
                this_content_name,
                that_content_name,
            } => {
                write!(
                    f,
                    "[{}] The shape {} is not a subtype of shape {} as guarded",
                    self.position.print_path_line_column_span(),
                    this_content_name,
                    that_content_name,
                )
            }
        }
    }
}

impl ShapeContentCheckError {
    pub fn generic_param_size_incompatible(
        position: ParsedPosition,
        shape_route: Route,
        expected: usize,
        found: usize,
    ) -> Self {
        Self {
            kind: ShapeContentCheckErrorKind::GenericParamSizeIncompatible {
                shape_route,
                expected,
                found,
            },
            position,
        }
    }

    pub fn generic_guard_fails(
        position: ParsedPosition,
        this_content_name: String,
        that_content_name: String,
    ) -> Self {
        Self {
            kind: ShapeContentCheckErrorKind::GenericGuardFails {
                this_content_name,
                that_content_name,
            },
            position,
        }
    }
}

pub type ShapeContentCheckErrors = AddableVec<ShapeContentCheckError>;
