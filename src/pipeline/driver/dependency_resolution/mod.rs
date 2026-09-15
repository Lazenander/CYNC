pub mod error;

use crate::compiler::analyzer::common::dependency_graph::dependency_graph::DependencyGraph;
use crate::pipeline::cells::cell::CellSignature;
use crate::pipeline::cells::cell_registry::CellRegistry;
use crate::pipeline::driver::dependency_resolution::error::{
    DependencyResolutionError, DependencyResolutionErrors,
};
use crate::pipeline::driver::driver::CompilerDriver;
use crate::pipeline::driver::error::{PipelineError, PipelineErrors};
use crate::utility::common::vec_add::AddableVec;
use std::cmp::min;
use std::collections::{HashMap, HashSet, VecDeque};

impl CompilerDriver {
    pub fn construct_dependency_graph(&mut self) -> bool {
        self.dependency_graph.nodes = self.cell_registries.keys().cloned().collect();

        // Initialize nexts HashMap with empty HashSets for all nodes
        self.dependency_graph.nodes.iter().for_each(|node| {
            self.dependency_graph
                .nexts
                .entry(node.clone())
                .or_insert_with(HashSet::new);
        });

        let mut starts = self
            .dependency_graph
            .nodes
            .iter()
            .map(CellSignature::clone)
            .collect::<HashSet<CellSignature>>();
        let not_found_errs = self
            .cell_registries
            .iter()
            .fold(vec![], |errs, (sign, registry)| {
                registry.dependencies.iter().fold(errs, |errs, dept| {
                    starts.remove(&sign);
                    match self
                        .dependency_graph
                        .connect(dept.signature.clone(), sign.clone())
                    {
                        Err(err_sign) => errs
                            .into_iter()
                            .chain(vec![DependencyResolutionError::dependency_not_found(
                                err_sign,
                            )])
                            .collect(),
                        _ => errs,
                    }
                })
            });
        self.dependency_graph.starts = starts.into_iter().collect();

        let flag = not_found_errs.is_empty();

        self.push_errors(PipelineErrors::from(
            <Vec<DependencyResolutionError> as Into<DependencyResolutionErrors>>::into(
                not_found_errs,
            ),
        ));

        flag && self.dependency_graph_dag_check()
    }

    fn dependency_graph_dag_check(&mut self) -> bool {
        let sccs = DependencyGraphScc::new(&self.dependency_graph).run();

        // Filter SCCs to only include actual circular dependencies:
        // - Multi-node components (true cycles)
        // - Single-node components with self-loops
        let circular_dependencies: Vec<_> = sccs
            .into_iter()
            .filter(|scc| {
                if scc.len() > 1 {
                    // Multi-node component is always a circular dependency
                    true
                } else if scc.len() == 1 {
                    // Single-node component: check if it has a self-loop
                    let node = &scc[0];
                    self.dependency_graph
                        .nexts
                        .get(node)
                        .map(|nexts| nexts.contains(node))
                        .unwrap_or(false)
                } else {
                    // Empty component (shouldn't happen)
                    false
                }
            })
            .collect();

        if circular_dependencies.is_empty() {
            return true;
        }

        circular_dependencies.into_iter().for_each(|scc| {
            self.push_error(PipelineError::DependencyResolutionError(
                DependencyResolutionError::circular_dependency(scc),
            ))
        });
        false
    }
}

struct DependencyGraphScc<'a> {
    graph: &'a DependencyGraph,
    index: usize,
    indices: HashMap<CellSignature, Option<usize>>,
    low: HashMap<CellSignature, usize>,
    on_stack: HashSet<CellSignature>,
    stack: Vec<CellSignature>,
    sccs: Vec<Vec<CellSignature>>,
}

impl<'a> DependencyGraphScc<'a> {
    pub fn new(graph: &'a DependencyGraph) -> Self {
        Self {
            graph,
            index: 0,
            indices: graph
                .nodes
                .iter()
                .map(|node| (node.clone(), None))
                .collect(),
            low: HashMap::new(),
            on_stack: HashSet::new(),
            stack: Vec::with_capacity(graph.nodes.len()),
            sccs: vec![],
        }
    }

    pub fn run(mut self) -> Vec<Vec<CellSignature>> {
        for sign in self.graph.nodes.iter() {
            if self.indices.get(sign).unwrap().is_none() {
                self.strong_connect(sign);
            }
        }
        self.sccs
    }

    fn strong_connect(&mut self, v: &CellSignature) {
        let idx = self.index;
        self.index += 1;
        self.indices.insert(v.clone(), Some(idx));
        self.low.insert(v.clone(), idx);

        self.stack.push(v.clone());
        self.on_stack.insert(v.clone());

        for w in self.graph.nexts.get(v).unwrap() {
            match self.indices.get(w).unwrap() {
                None => {
                    self.strong_connect(w);
                    self.low.insert(
                        v.clone(),
                        min(*self.low.get(v).unwrap(), *self.low.get(w).unwrap()),
                    );
                }
                Some(w_idx) => {
                    if self.on_stack.contains(w) {
                        self.low
                            .insert(v.clone(), min(*self.low.get(v).unwrap(), *w_idx));
                    }
                }
            }
        }

        if self.low.get(v) == self.indices.get(v).unwrap().as_ref() {
            let mut component = Vec::new();
            loop {
                let w = self.stack.pop().unwrap();
                self.on_stack.remove(&w);
                component.push(w.clone());
                if w == v.clone() {
                    break;
                }
            }
            self.sccs.push(component);
        }
    }
}
