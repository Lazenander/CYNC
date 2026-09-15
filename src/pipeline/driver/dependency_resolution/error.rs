use crate::compiler::lexer::tokens::TokenGut;
use crate::compiler::parser::error::ParseError;
use crate::pipeline::cells::cell::CellSignature;
use crate::utility::common::vec_add::AddableVec;
use std::fmt;

#[derive(Debug, Clone)]
pub enum DependencyResolutionErrorKind {
    CIRCULAR_DEPENDENCY(Vec<CellSignature>),
    DEPENDENCY_NOT_FOUND(CellSignature),
}

#[derive(Debug, Clone)]
pub struct DependencyResolutionError {
    kind: DependencyResolutionErrorKind,
}

impl fmt::Display for DependencyResolutionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            DependencyResolutionErrorKind::CIRCULAR_DEPENDENCY(signs) => write!(
                f,
                "[Dependency] Circular dependency detected at cells [{}]",
                signs
                    .iter()
                    .fold("".into(), |print, sign| { format!("{}, {}", print, sign) })
            ),
            DependencyResolutionErrorKind::DEPENDENCY_NOT_FOUND(sign) => {
                write!(f, "[Dependency] Dependency cell [{}] not found", sign)
            }
        }
    }
}

impl DependencyResolutionError {
    pub fn circular_dependency(cell_signatures: Vec<CellSignature>) -> Self {
        Self {
            kind: DependencyResolutionErrorKind::CIRCULAR_DEPENDENCY(cell_signatures),
        }
    }
    pub fn dependency_not_found(cell_signature: CellSignature) -> Self {
        Self {
            kind: DependencyResolutionErrorKind::DEPENDENCY_NOT_FOUND(cell_signature),
        }
    }
}

pub type DependencyResolutionErrors = AddableVec<DependencyResolutionError>;
