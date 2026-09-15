use crate::pipeline::cells::cell::CellSignature;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct CellDependency {
    pub alias: Option<String>,
    pub signature: CellSignature,
    pub source: Option<PathBuf>,
}
