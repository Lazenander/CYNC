use crate::compiler::analyzer::error::AnalyzeErrors;
use crate::compiler::analyzer::global_name_resolution::module_skeleton::GrandModuleSkeletonTable;
use crate::pipeline::driver::driver::CompilerDriver;
use std::collections::HashMap;

// Debug functions for module skeleton construction
// To enable debug output, uncomment the function calls in module_skeleton()
/* fn _debug_print_module_skeleton_start(dependency_graph: &crate::compiler::analyzer::common::dependency_graph::dependency_graph::DependencyGraph) {
    println!("=== STARTING MODULE SKELETON & ALIAS RESOLUTION ===");
    println!("Processing cells in dependency order:");
    for sign in dependency_graph.post_order_iter() {
        println!("  Cell: {}", sign.name);
    }
}

fn _debug_print_module_skeleton_result(gmsk: &GrandModuleSkeletonTable, success: bool) {
    if success {
        println!("Module skeleton construction completed successfully!");
        println!("Final skeleton statistics:");
        for (cell_sig, skeleton_table) in &gmsk.cell_module_skeleton_table {
            println!("  Cell '{}': {} definitions",
                cell_sig.name,
                skeleton_table.def_signatures.len()
            );

            // Show resolved aliases
            let resolved_aliases: Vec<_> = skeleton_table.def_signatures.iter()
                .filter_map(|(route, sig)| {
                    if sig.is_resolved() {
                        match sig {
                            crate::compiler::analyzer::global_name_resolution::def_signature::SkeletonDefSignature::ResolvedAlias(_) => {
                                Some(format!("    Resolved alias: {} -> {}", route, sig.get_resolved_route()))
                            },
                            _ => None
                        }
                    } else {
                        Some(format!("    Unresolved: {}", route))
                    }
                })
                .collect();

            if !resolved_aliases.is_empty() {
                for alias in resolved_aliases {
                    println!("{}", alias);
                }
            }
        }
    } else {
        println!("Module skeleton construction failed with {} errors:", gmsk.errors.len());
        for (i, error) in gmsk.errors.iter().enumerate() {
            println!("  {}. {}", i + 1, error);
        }
    }
    println!("=== FINISHED MODULE SKELETON & ALIAS RESOLUTION ===");
} */

impl CompilerDriver {
    pub fn module_skeleton(&mut self) -> bool {
        // Debug output - temporarily enabled to verify alias resolution
        // _debug_print_module_skeleton_start(&self.dependency_graph);

        let gmsk = GrandModuleSkeletonTable::new(
            self.cell_registries
                .iter()
                .map(|(cell_sign, cell_registry)| {
                    (
                        cell_sign.clone(),
                        cell_registry
                            .dependencies
                            .iter()
                            .map(|dependency| match dependency.alias.clone() {
                                None => (
                                    dependency.signature.clone(),
                                    dependency.signature.name.clone(),
                                ),
                                Some(alias_name) => (dependency.signature.clone(), alias_name),
                            })
                            .collect::<HashMap<_, _>>(),
                    )
                })
                .collect(),
            &self.dependency_graph,
            &self.grand_merged_modules_table,
        );
        let flag = gmsk.errors.is_empty();

        // Debug output - temporarily enabled to verify alias resolution
        // _debug_print_module_skeleton_result(&gmsk, flag);

        if flag {
            self.grand_module_skeleton_table = gmsk;
        } else {
            self.errors.append(AnalyzeErrors::from(gmsk.errors).into());
        }

        flag
    }
}
