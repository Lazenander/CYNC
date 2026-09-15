use crate::compiler::analyzer::common::route::route::Route;
use crate::compiler::analyzer::global_name_resolution::def_signature::SkeletonDefSignatureKind;
use crate::compiler::parser::parser::ParsedPosition;
use crate::utility::common::vec_add::AddableVec;
use std::fmt;
use std::fmt::write;

#[derive(Debug, Clone)]
pub enum ContentResolutionErrorKind {
    DuplicateGenericVariableDefinition(String),

    InvalidBindType,
    BindOperationTypeMismatch,
    InvalidBindGenericVariable,

    DuplicatePatternVariableDefinition(String),
}

#[derive(Debug, Clone)]
pub struct ContentResolutionError {
    pub kind: ContentResolutionErrorKind,
    pub position: ParsedPosition,
}

impl fmt::Display for ContentResolutionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            ContentResolutionErrorKind::DuplicateGenericVariableDefinition(
                generic_variable_name,
            ) => {
                write!(
                    f,
                    "[{}] Generic variable \"{}\" definition duplication",
                    self.position.print_path_line_column_span(),
                    generic_variable_name,
                )
            }
            ContentResolutionErrorKind::InvalidBindType => {
                write!(
                    f,
                    "[{}] Binding operations only allows named shapes",
                    self.position.print_path_line_column_span()
                )
            }
            ContentResolutionErrorKind::BindOperationTypeMismatch => {
                write!(f, "[{}] Binding operations should be of shape T -> Any, where T is the binding shape", self.position.print_path_line_column_span())
            }
            ContentResolutionErrorKind::InvalidBindGenericVariable => {
                write!(
                    f,
                    "[{}] Generic variable cannot be binded directly",
                    self.position.print_path_line_column_span()
                )
            }
            ContentResolutionErrorKind::DuplicatePatternVariableDefinition(
                pattern_variable_name,
            ) => {
                write!(
                    f,
                    "[{}] Pattern variable \"{}\" definition duplication",
                    self.position.print_path_line_column_span(),
                    pattern_variable_name,
                )
            }
        }
    }
}

impl ContentResolutionError {
    pub fn duplicate_generic_variable_definition(
        position: ParsedPosition,
        generic_variable_name: String,
    ) -> Self {
        Self {
            kind: ContentResolutionErrorKind::DuplicateGenericVariableDefinition(
                generic_variable_name,
            ),
            position,
        }
    }

    pub fn invalid_bind_type(position: ParsedPosition) -> Self {
        Self {
            kind: ContentResolutionErrorKind::InvalidBindType,
            position,
        }
    }

    pub fn bind_operation_type_mismatch(position: ParsedPosition) -> Self {
        Self {
            kind: ContentResolutionErrorKind::BindOperationTypeMismatch,
            position,
        }
    }

    pub fn invalid_bind_generic_variable(position: ParsedPosition) -> Self {
        Self {
            kind: ContentResolutionErrorKind::InvalidBindGenericVariable,
            position,
        }
    }
    pub fn duplicate_pattern_variable_definition(
        position: ParsedPosition,
        pattern_variable_name: String,
    ) -> Self {
        Self {
            kind: ContentResolutionErrorKind::DuplicatePatternVariableDefinition(
                pattern_variable_name,
            ),
            position,
        }
    }
}

pub type ContentResolutionErrors = AddableVec<ContentResolutionError>;
