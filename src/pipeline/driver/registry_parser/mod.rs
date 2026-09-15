use crate::pipeline::cells::cell::CellSignature;
use crate::pipeline::driver::driver::CompilerDriver;
use crate::pipeline::driver::error::{PipelineError, PipelineErrors};
use crate::pipeline::driver::registry_parser::error::CellInstallError;
use crate::pipeline::driver::registry_parser::registry_parser::RegistryParser;
use crate::utility::common::vec_add::AddableVec;
use crate::utility::consts::{CELL_DEPENDENCIES_DIR_NAME, CELL_REGISTRY_FILE_NAME};
use std::io;
use std::path::PathBuf;

pub mod error;
mod operation_parser;
pub mod registry_parser;
mod version_parser;

impl CompilerDriver {
    pub fn process_registry_from_none(&mut self) -> bool {
        self.process_registry(None)
    }

    fn process_registry(&mut self, cell_signature: Option<CellSignature>) -> bool {
        let is_start = cell_signature.is_none();
        let path: PathBuf;
        if is_start {
            path = self.this_path.clone();
        } else {
            path = self
                .this_path
                .clone()
                .join(CELL_DEPENDENCIES_DIR_NAME)
                .join(cell_signature.clone().unwrap().to_string());
        }
        let registry_path = path.join(CELL_REGISTRY_FILE_NAME);
        let this_registry = RegistryParser::parse(cell_signature, registry_path.clone());
        let cell_registry = this_registry.value;
        self.errors = self.errors.to_owned() + PipelineErrors::from(this_registry.error);
        match cell_registry {
            None => false,
            Some(cell_registry) => {
                if is_start {
                    self.this_signature = Some(cell_registry.signature.clone());
                }
                self.cell_paths
                    .insert(cell_registry.signature.clone(), path.clone());

                let flag = cell_registry
                    .dependencies
                    .iter()
                    .fold(true, |flag, dependency| {
                        if !self.cell_registries.contains_key(&dependency.signature) {
                            self.add_cell(
                                dependency.signature.clone(),
                                dependency.source.clone(),
                                false,
                            );
                            self.process_registry(Some(dependency.signature.clone())) && flag
                        } else {
                            flag
                        }
                    });

                self.cell_registries
                    .insert(cell_registry.signature.clone(), cell_registry);

                flag
            }
        }
    }

    fn add_cell(
        &mut self,
        cell_signature: CellSignature,
        source_path: Option<PathBuf>,
        allow_overwrite: bool,
    ) {
        match source_path {
            Some(source_path) => {
                let target_path = self
                    .this_path
                    .clone()
                    .join(CELL_DEPENDENCIES_DIR_NAME)
                    .join(cell_signature.clone().to_string());
                if target_path.exists() {
                    return;
                }
                if let Err(e) = Self::clone_directory(&source_path, &target_path, allow_overwrite) {
                    println!("{}", e.to_string());
                    self.push_errors(AddableVec::from(vec![PipelineError::CellInstallError(
                        CellInstallError::parse_failure(cell_signature.clone()),
                    )]));
                }
            }
            None => {}
        }
    }

    fn clone_directory(
        source_path: &PathBuf,
        target_path: &PathBuf,
        allow_overwrite: bool,
    ) -> io::Result<()> {
        let mut options = fs_extra::dir::CopyOptions::new();
        options.overwrite = allow_overwrite;
        options.copy_inside = true;

        fs_extra::dir::copy(source_path, target_path, &options).expect("Failed to copy directory");
        Ok(())
    }
}
