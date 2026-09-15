use crate::pipeline::cells::cell::CellSignature;
use crate::utility::common::vec_add::AddableVec;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum RegistryParseErrorKind {
    FILE_READ_FAILURE,
    PARSE_FAILURE,
    EMPTY_NAME,
    SIGNATURE_MISMATCH,
    VERSION_PARSE_FORMAT_ERROR,
}

#[derive(Debug, Clone)]
pub struct RegistryParseError {
    kind: RegistryParseErrorKind,
    path: PathBuf,
    cell_signature: CellSignature,
}

impl std::fmt::Display for RegistryParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.kind {
            RegistryParseErrorKind::FILE_READ_FAILURE => write!(
                f,
                "[{}, {}] Cell registry file read failure (i.e. cell.json does not exist)",
                self.cell_signature.to_string(),
                self.path.display()
            ),
            RegistryParseErrorKind::PARSE_FAILURE => write!(
                f,
                "[{}, {}] Cell registry parse failure",
                self.cell_signature.to_string(),
                self.path.display()
            ),
            RegistryParseErrorKind::EMPTY_NAME => write!(
                f,
                "[{}, {}] Empty cell name error",
                self.cell_signature.to_string(),
                self.path.display()
            ),
            RegistryParseErrorKind::SIGNATURE_MISMATCH => write!(
                f,
                "[{}, {}] Cell signature (name, version) mismatch",
                self.cell_signature.to_string(),
                self.path.display()
            ),
            RegistryParseErrorKind::VERSION_PARSE_FORMAT_ERROR => write!(
                f,
                "{}, [{}] Cell registry failed to parse",
                self.cell_signature.to_string(),
                self.path.display()
            ),
        }
    }
}

impl RegistryParseError {
    pub fn file_read_failure(path: PathBuf, cell_signature: CellSignature) -> Self {
        Self {
            kind: RegistryParseErrorKind::FILE_READ_FAILURE,
            path,
            cell_signature,
        }
    }

    pub fn parse_failure(path: PathBuf, cell_signature: CellSignature) -> Self {
        Self {
            kind: RegistryParseErrorKind::PARSE_FAILURE,
            path,
            cell_signature,
        }
    }

    pub fn empty_name(path: PathBuf, cell_signature: CellSignature) -> Self {
        Self {
            kind: RegistryParseErrorKind::EMPTY_NAME,
            path,
            cell_signature,
        }
    }

    pub fn signature_mismatch(path: PathBuf, cell_signature: CellSignature) -> Self {
        Self {
            kind: RegistryParseErrorKind::SIGNATURE_MISMATCH,
            path,
            cell_signature,
        }
    }

    pub fn version_parse_format_parse_failed(path: PathBuf, cell_signature: CellSignature) -> Self {
        Self {
            kind: RegistryParseErrorKind::VERSION_PARSE_FORMAT_ERROR,
            path,
            cell_signature,
        }
    }
}

pub type RegistryParseErrors = AddableVec<RegistryParseError>;

#[derive(Debug, Clone)]
pub enum CellInstallErrorKind {
    INSTALL_FAILURE_FROM_SOURCE,
}

#[derive(Debug, Clone)]
pub struct CellInstallError {
    kind: CellInstallErrorKind,
    cell_signature: CellSignature,
}

impl std::fmt::Display for CellInstallError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.kind {
            CellInstallErrorKind::INSTALL_FAILURE_FROM_SOURCE => write!(
                f,
                "[{}] Unable to install from source",
                self.cell_signature.to_string()
            ),
        }
    }
}

impl CellInstallError {
    pub fn parse_failure(cell_signature: CellSignature) -> Self {
        Self {
            kind: CellInstallErrorKind::INSTALL_FAILURE_FROM_SOURCE,
            cell_signature,
        }
    }
}

pub type CellInstallErrors = AddableVec<CellInstallError>;
