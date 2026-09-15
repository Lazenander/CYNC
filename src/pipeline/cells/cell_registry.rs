use crate::pipeline::cells::cell::CellSignature;
use crate::pipeline::cells::cell_cmd::CellCommand;
use crate::pipeline::cells::cell_dependency::CellDependency;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct CellRegistry {
    pub signature: CellSignature,
    pub authors: Vec<String>,
    pub description: String,
    pub dependencies: Vec<CellDependency>,
    pub commands: Vec<CellCommand>,
}
