use crate::compiler::analyzer::common::route::route::Route;
use crate::compiler::parser::parser::ParsedPosition;
use crate::utility::common::vec_add::AddableVec;
use std::fmt;

#[derive(Debug, Clone)]
pub enum ModuleMergeErrorKind {
    ModuleMergeConflict,
}

#[derive(Debug, Clone)]
pub struct ModuleMergeError {
    pub kind: ModuleMergeErrorKind,
    pub name: String,
    pub route: Route,
    pub position: ParsedPosition,
}

impl fmt::Display for ModuleMergeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            ModuleMergeErrorKind::ModuleMergeConflict => {
                write!(
                    f,
                    "[{}] Name \"{}\" conflict under the module \"{}\"",
                    self.position.print_path_line_column_span(),
                    self.name,
                    self.route
                )
            }
        }
    }
}

impl ModuleMergeError {
    pub fn module_merge_conflict(name: String, route: Route, position: ParsedPosition) -> Self {
        Self {
            kind: ModuleMergeErrorKind::ModuleMergeConflict,
            name,
            route,
            position,
        }
    }
}

pub type ModuleMergeErrors = AddableVec<ModuleMergeError>;
