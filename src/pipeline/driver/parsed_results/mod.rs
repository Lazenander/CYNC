use crate::pipeline::driver::driver::CompilerDriver;
use crate::pipeline::driver::error::{PipelineError, PipelineErrors};
use crate::pipeline::driver::parsed_results::parsed_result::FileParsedResult;
use crate::utility::common::vec_add::AddableVec;
use crate::utility::consts::{CELL_DEPENDENCIES_DIR_NAME, CYNC_FILE_EXTENSION};
use std::path::PathBuf;
use walkdir::WalkDir;

pub mod parsed_result;

pub struct CellParsedResult(pub Vec<FileParsedResult>);

impl CompilerDriver {
    pub fn parse_cell_files(&mut self) -> bool {
        self.cell_parsed_results = self
            .cell_paths
            .iter()
            .map(|(signature, path)| {
                (
                    signature.clone(),
                    if self.this_signature.clone().unwrap() == signature.clone() {
                        Self::program_cell_parsed_result_extraction(path.clone())
                    } else {
                        Self::library_cell_parsed_result_extraction(path.clone())
                    },
                )
            })
            .collect();

        self.cell_parsed_results
            .iter()
            .fold(true, |flag, (_, CellParsedResult(result))| {
                let new_parsed_errors = AddableVec::from(result.iter().fold(
                    vec![],
                    |mut errors, FileParsedResult::Cync(file_result)| {
                        errors
                            .into_iter()
                            .chain(file_result.parse_errors.0.clone().into_iter())
                            .collect()
                    },
                ));
                let new_pipeline_errors = PipelineErrors::from(new_parsed_errors);
                let has_no_errors = new_pipeline_errors.0.is_empty();
                self.errors = self.errors.to_owned() + new_pipeline_errors;
                has_no_errors && flag
            })
    }

    fn library_cell_parsed_result_extraction(dir_path: PathBuf) -> CellParsedResult {
        CellParsedResult(
            WalkDir::new(&dir_path)
                .follow_links(true)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| {
                    e.file_type().is_file()
                        && e.path()
                            .extension()
                            .and_then(|ext| ext.to_str())
                            .map_or(false, |ext| ext == CYNC_FILE_EXTENSION)
                })
                .map(|e| FileParsedResult::new_cync_file_analysis(e.path().to_owned()))
                .collect(),
        )
    }

    fn program_cell_parsed_result_extraction(dir_path: PathBuf) -> CellParsedResult {
        CellParsedResult(
            WalkDir::new(&dir_path)
                .follow_links(true)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| {
                    e.file_type().is_file()
                        && e.path()
                            .extension()
                            .and_then(|ext| ext.to_str())
                            .map_or(false, |ext| ext == CYNC_FILE_EXTENSION)
                        && !e
                            .path()
                            .to_string_lossy()
                            .contains(format!("/{}/", CELL_DEPENDENCIES_DIR_NAME).as_str())
                })
                .map(|e| FileParsedResult::new_cync_file_analysis(e.path().to_owned()))
                .collect(),
        )
    }
}
