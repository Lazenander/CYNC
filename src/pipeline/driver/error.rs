use crate::compiler::analyzer::error::{AnalyzeError, AnalyzeErrors};
use crate::compiler::parser::error::{ParseError, ParseErrors};
use crate::pipeline::driver::dependency_resolution::error::{
    DependencyResolutionError, DependencyResolutionErrors,
};
use crate::pipeline::driver::driver::CompilerDriver;
use crate::pipeline::driver::registry_parser::error::{
    CellInstallError, CellInstallErrors, RegistryParseError, RegistryParseErrors,
};
use crate::utility::common::vec_add::AddableVec;
use std::ops::Add;

#[derive(Debug, Clone)]
pub enum PipelineError {
    CellRegistryError(RegistryParseError),
    CellInstallError(CellInstallError),
    ParseError(ParseError),
    DependencyResolutionError(DependencyResolutionError),
    AnalyzeError(AnalyzeError),
}

impl From<RegistryParseError> for PipelineError {
    fn from(value: RegistryParseError) -> Self {
        Self::CellRegistryError(value)
    }
}

impl From<CellInstallError> for PipelineError {
    fn from(value: CellInstallError) -> Self {
        Self::CellInstallError(value)
    }
}

impl From<ParseError> for PipelineError {
    fn from(value: ParseError) -> Self {
        Self::ParseError(value)
    }
}

impl From<DependencyResolutionError> for PipelineError {
    fn from(value: DependencyResolutionError) -> Self {
        Self::DependencyResolutionError(value)
    }
}

impl From<AnalyzeError> for PipelineError {
    fn from(value: AnalyzeError) -> Self {
        Self::AnalyzeError(value)
    }
}

pub type PipelineErrors = AddableVec<PipelineError>;

impl From<RegistryParseErrors> for PipelineErrors {
    fn from(value: RegistryParseErrors) -> Self {
        value
            .0
            .into_iter()
            .map(PipelineError::from)
            .collect::<Vec<_>>()
            .into()
    }
}

impl From<CellInstallErrors> for PipelineErrors {
    fn from(value: CellInstallErrors) -> Self {
        value
            .0
            .into_iter()
            .map(PipelineError::from)
            .collect::<Vec<_>>()
            .into()
    }
}

impl From<ParseErrors> for PipelineErrors {
    fn from(value: ParseErrors) -> Self {
        value
            .0
            .into_iter()
            .map(PipelineError::from)
            .collect::<Vec<_>>()
            .into()
    }
}

impl From<DependencyResolutionErrors> for PipelineErrors {
    fn from(value: DependencyResolutionErrors) -> Self {
        value
            .0
            .into_iter()
            .map(PipelineError::from)
            .collect::<Vec<_>>()
            .into()
    }
}

impl From<AnalyzeErrors> for PipelineErrors {
    fn from(value: AnalyzeErrors) -> Self {
        value
            .0
            .into_iter()
            .map(PipelineError::from)
            .collect::<Vec<_>>()
            .into()
    }
}

impl CompilerDriver {
    pub fn push_error(&mut self, err: PipelineError) {
        self.errors.push(err);
    }

    pub fn push_errors(&mut self, errs: AddableVec<PipelineError>) {
        errs.0.into_iter().for_each(|err| self.push_error(err));
    }
}
