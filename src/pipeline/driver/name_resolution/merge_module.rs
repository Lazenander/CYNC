use crate::compiler::analyzer::common::route::route::{CanonicalRoute, Route};
use crate::compiler::analyzer::error::AnalyzeErrors;
use crate::compiler::analyzer::module_merge::module_merge::{
    GrandModuleMergeTable, MergedModuleTable,
};
use crate::pipeline::driver::driver::CompilerDriver;
use crate::pipeline::driver::error::{PipelineError, PipelineErrors};
use crate::pipeline::driver::parsed_results::parsed_result::FileParsedResult;
use crate::utility::common::vec_add::AddableVec;
use std::collections::HashMap;

impl CompilerDriver {
    pub fn merge_modules(&mut self) -> bool {
        let mut cell_module_merge_tables = HashMap::new();

        for (key, parsed_result) in self.cell_parsed_results.drain() {
            let mmt = cell_module_merge_tables
                .entry(key.clone())
                .or_insert_with(MergedModuleTable::new);

            for file_pr in parsed_result.0.into_iter() {
                let FileParsedResult::Cync(parsed) = file_pr;
                mmt.merge_program(key.clone(), parsed.parse_result.owned_value());
            }
        }

        let flag = cell_module_merge_tables
            .iter()
            .fold(true, |flag, (_, mmt)| {
                mmt.m_modules.iter().fold(true, |flag, (m_route, mm)| {
                    let has_no_conflicts = mm.errors.is_empty();

                    let new_pipeline_errors =
                        PipelineErrors::from(AnalyzeErrors::from(mm.errors.clone()));
                    self.errors = self.errors.to_owned() + new_pipeline_errors;

                    has_no_conflicts && flag
                }) && flag
            });

        if flag {
            self.grand_merged_modules_table = GrandModuleMergeTable {
                cell_module_merge_tables,
            };
        }

        flag
    }
}
