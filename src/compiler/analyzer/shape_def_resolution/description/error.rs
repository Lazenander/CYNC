use crate::compiler::analyzer::common::route::route::Route;
use crate::compiler::analyzer::global_name_resolution::def_signature::SkeletonDefSignatureKind;
use crate::compiler::parser::parser::ParsedPosition;
use crate::utility::common::vec_add::AddableVec;
use std::fmt;

#[derive(Debug, Clone)]
pub enum ShapeDescriptionResolutionErrorKind {
    StructFieldDuplication(String),
    UnexpectedAssignmentSyntaxSugarUsed,
    UnknownPatternVariable(String),
    GenericVariableTemplateCallDetected,
    CoreTypeTemplateCallDetected,
}

#[derive(Debug, Clone)]
pub struct ShapeDescriptionResolutionError {
    pub kind: ShapeDescriptionResolutionErrorKind,
    pub position: ParsedPosition,
}

impl fmt::Display for ShapeDescriptionResolutionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            ShapeDescriptionResolutionErrorKind::StructFieldDuplication(field_name) => {
                write!(
                    f,
                    "[{}] Struct field name {} duplicates",
                    self.position.print_path_line_column_span(),
                    field_name,
                )
            }
            ShapeDescriptionResolutionErrorKind::UnexpectedAssignmentSyntaxSugarUsed => {
                write!(
                    f,
                    "[{}] Assignment syntax sugar is only for shape definition.",
                    self.position.print_path_line_column_span(),
                )
            }
            ShapeDescriptionResolutionErrorKind::UnknownPatternVariable(pattern_variable) => {
                write!(
                    f,
                    "[{}] Undefined pattern variable {}.",
                    self.position.print_path_line_column_span(),
                    pattern_variable,
                )
            }
            ShapeDescriptionResolutionErrorKind::GenericVariableTemplateCallDetected => {
                write!(
                    f,
                    "[{}] Generic variable is not allowed to have template call",
                    self.position.print_path_line_column_span(),
                )
            }
            ShapeDescriptionResolutionErrorKind::CoreTypeTemplateCallDetected => {
                write!(
                    f,
                    "[{}] Core types do not have template call",
                    self.position.print_path_line_column_span(),
                )
            }
        }
    }
}

impl ShapeDescriptionResolutionError {
    pub fn struct_field_name_duplication(position: ParsedPosition, field_name: String) -> Self {
        Self {
            kind: ShapeDescriptionResolutionErrorKind::StructFieldDuplication(field_name),
            position,
        }
    }

    pub fn unexpected_assignment_syntax_sugar_used(position: ParsedPosition) -> Self {
        Self {
            kind: ShapeDescriptionResolutionErrorKind::UnexpectedAssignmentSyntaxSugarUsed,
            position,
        }
    }

    pub fn unknown_pattern_variable(
        position: ParsedPosition,
        pattern_variable_name: String,
    ) -> Self {
        Self {
            kind: ShapeDescriptionResolutionErrorKind::UnknownPatternVariable(
                pattern_variable_name,
            ),
            position,
        }
    }

    pub fn generic_variable_template_call_detected(position: ParsedPosition) -> Self {
        Self {
            kind: ShapeDescriptionResolutionErrorKind::GenericVariableTemplateCallDetected,
            position,
        }
    }

    pub fn core_type_template_call_detected(position: ParsedPosition) -> Self {
        Self {
            kind: ShapeDescriptionResolutionErrorKind::CoreTypeTemplateCallDetected,
            position,
        }
    }
}

pub type ShapeDescriptionResolutionErrors = AddableVec<ShapeDescriptionResolutionError>;

#[derive(Debug, Clone)]
pub enum FunctionDescriptionResolutionErrorKind {
    ParamNameDuplication(String),
}

#[derive(Debug, Clone)]
pub struct FunctionDescriptionResolutionError {
    pub kind: FunctionDescriptionResolutionErrorKind,
    pub position: ParsedPosition,
}

impl fmt::Display for FunctionDescriptionResolutionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            FunctionDescriptionResolutionErrorKind::ParamNameDuplication(pattern) => {
                write!(
                    f,
                    "[{}] Parameter name \"{}\" duplicates",
                    self.position.print_path_line_column_span(),
                    pattern,
                )
            }
        }
    }
}

impl FunctionDescriptionResolutionError {
    pub fn param_name_duplication(position: ParsedPosition, param_name: String) -> Self {
        Self {
            kind: FunctionDescriptionResolutionErrorKind::ParamNameDuplication(param_name),
            position,
        }
    }
}

pub type FunctionDescriptionResolutionErrors = AddableVec<FunctionDescriptionResolutionError>;
