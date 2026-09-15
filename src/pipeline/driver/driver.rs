use crate::compiler::analyzer::common::dependency_graph::dependency_graph::DependencyGraph;
use crate::compiler::analyzer::global_name_resolution::module_skeleton::GrandModuleSkeletonTable;
use crate::compiler::analyzer::module_merge::module_merge::{
    GrandModuleMergeTable, MergedModuleTable,
};
use crate::compiler::analyzer::shape_def_resolution::summary::GrandShapeContentSummary;
use crate::pipeline::cells::cell::CellSignature;
use crate::pipeline::cells::cell_registry::CellRegistry;
use crate::pipeline::driver::error::{PipelineError, PipelineErrors};
use crate::pipeline::driver::parsed_results::CellParsedResult;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::{fs, io};
use walkdir::WalkDir;

pub struct CompilerDriver {
    pub this_path: PathBuf,
    pub this_signature: Option<CellSignature>,
    pub cell_paths: HashMap<CellSignature, PathBuf>,
    pub cell_registries: HashMap<CellSignature, CellRegistry>,
    pub cell_parsed_results: HashMap<CellSignature, CellParsedResult>,

    pub dependency_graph: DependencyGraph,
    pub grand_merged_modules_table: GrandModuleMergeTable,
    pub grand_module_skeleton_table: GrandModuleSkeletonTable,

    pub grand_shape_content_summary: GrandShapeContentSummary,

    pub errors: PipelineErrors,
}

impl CompilerDriver {
    pub fn get_dependency_cell_path(&self, signature: &CellSignature) -> PathBuf {
        self.this_path.clone().join(signature.to_string())
    }

    pub fn new(this_path: PathBuf) -> CompilerDriver {
        CompilerDriver {
            this_path,
            this_signature: None,
            cell_paths: HashMap::new(),
            cell_registries: HashMap::new(),
            cell_parsed_results: HashMap::new(),

            dependency_graph: DependencyGraph::new(),
            grand_merged_modules_table: GrandModuleMergeTable::new_empty(),
            grand_module_skeleton_table: GrandModuleSkeletonTable::new_empty(),

            grand_shape_content_summary: GrandShapeContentSummary::new_empty(),

            errors: vec![].into(),
        }
    }

    pub fn pipeline(&mut self) -> bool {
        println!("Step 1: Processing registry...");
        let step1 = self.process_registry_from_none();
        println!("Step 1 result: {}", step1);

        if !step1 {
            return false;
        }

        println!("Step 2: Constructing dependency graph...");
        let step2 = self.construct_dependency_graph();
        println!("Step 2 result: {}", step2);

        if !step2 {
            return false;
        }

        println!("Step 3: Parsing cell files...");
        let step3 = self.parse_cell_files();
        println!("Step 3 result: {}", step3);

        if !step3 {
            return false;
        }

        println!("Step 4: Resolving names...");
        let step4 = self.resolve_names();
        println!("Step 4 result: {}", step4);

        step4
    }

    pub fn log_errors(&self) {
        self.errors.iter().for_each(|error| match error {
            PipelineError::CellRegistryError(err) => {
                println!("{}", err.to_string())
            }
            PipelineError::CellInstallError(err) => {
                println!("{}", err.to_string())
            }
            PipelineError::ParseError(err) => {
                println!("{}", err.to_string())
            }
            PipelineError::AnalyzeError(err) => {
                println!("{}", err.to_string())
            }
            PipelineError::DependencyResolutionError(err) => {
                println!("{}", err.to_string())
            }
        })
    }
}
