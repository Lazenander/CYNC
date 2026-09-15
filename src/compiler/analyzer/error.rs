use crate::compiler::analyzer::common::route::route::Route;
use crate::compiler::analyzer::global_name_resolution::error::{
    GlobalNameAliasResolutionError, GlobalNameAliasResolutionErrors,
};
use crate::compiler::analyzer::module_merge::error::{ModuleMergeError, ModuleMergeErrors};
use crate::compiler::lexer::tokens::{Token, TokenGut};
use crate::compiler::parser::error::{ErrorKind, ParseError};
use crate::compiler::parser::parser::ParsedPosition;
use crate::utility::common::vec_add::AddableVec;
use std::fmt;
use std::fmt::{write, Formatter};

#[derive(Debug, Clone)]
pub enum AnalyzeError {
    ModuleMergeError(ModuleMergeError),
    GlobalNameAliasResolutionError(GlobalNameAliasResolutionError),
}

impl From<ModuleMergeError> for AnalyzeError {
    fn from(value: ModuleMergeError) -> Self {
        Self::ModuleMergeError(value)
    }
}

impl From<GlobalNameAliasResolutionError> for AnalyzeError {
    fn from(value: GlobalNameAliasResolutionError) -> Self {
        Self::GlobalNameAliasResolutionError(value)
    }
}

impl fmt::Display for AnalyzeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self {
            AnalyzeError::ModuleMergeError(err) => {
                write!(f, "{}", err)
            }
            AnalyzeError::GlobalNameAliasResolutionError(err) => {
                write!(f, "{}", err)
            }
        }
    }
}

pub type AnalyzeErrors = AddableVec<AnalyzeError>;

impl From<ModuleMergeErrors> for AnalyzeErrors {
    fn from(value: ModuleMergeErrors) -> Self {
        value
            .0
            .into_iter()
            .map(AnalyzeError::from)
            .collect::<Vec<_>>()
            .into()
    }
}

impl From<GlobalNameAliasResolutionErrors> for AnalyzeErrors {
    fn from(value: GlobalNameAliasResolutionErrors) -> Self {
        value
            .0
            .into_iter()
            .map(AnalyzeError::from)
            .collect::<Vec<_>>()
            .into()
    }
}
