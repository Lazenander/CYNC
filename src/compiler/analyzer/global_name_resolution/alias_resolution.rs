use crate::compiler::analyzer::common::route::route::{CanonicalRoute, Route};
use crate::compiler::analyzer::global_name_resolution::def_signature::{
    ResolvedAliasSkeletonDefSignature, SkeletonDefSignature,
};
use crate::compiler::analyzer::global_name_resolution::error::GlobalNameAliasResolutionError;
use crate::compiler::analyzer::global_name_resolution::lookup_canonical_route::LookUpCanonicalRouteResult;
use crate::compiler::analyzer::global_name_resolution::module_skeleton::{
    GrandModuleSkeletonTable, ModuleSkeleton, ModuleSkeletonTable,
};
use crate::pipeline::cells::cell::CellSignature;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug)]
enum ModuleAliasResolutionResult {
    Resolved(CanonicalRoute),
    Unresolved,
    VisibilityDenial,
    NameNotFound,
}

#[derive(Debug)]
enum ModuleAliasTo {
    InCell(Route),
    ExCell(Route),
}

#[derive(Debug)]
struct ModuleAliasGraph {
    nodes: HashMap<Route, ModuleAliasResolutionResult>,
    direct_lasts: HashMap<Route, ModuleAliasTo>,
    alias_lasts: HashMap<Route, ModuleAliasTo>,
    direct_nexts: HashMap<Route, Vec<Route>>,
    alias_nexts: HashMap<Route, Vec<Route>>,

    indegrees: HashMap<Route, usize>,
}

impl ModuleAliasGraph {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            direct_lasts: HashMap::new(),
            alias_lasts: HashMap::new(),
            direct_nexts: HashMap::new(),
            alias_nexts: HashMap::new(),
            indegrees: HashMap::new(),
        }
    }

    pub fn push_from_direct(&mut self, route: Route) {
        if !self.nodes.contains_key(&route) {
            self.nodes
                .insert(route.clone(), ModuleAliasResolutionResult::Unresolved);
            match route.get_parent_route() {
                Some(parent_route) => {
                    if !self.indegrees.contains_key(&route) {
                        self.indegrees.insert(route.clone(), 0);
                    }
                    if !self.indegrees.contains_key(&parent_route) {
                        self.indegrees.insert(parent_route.clone(), 0);
                    }
                    self.indegrees
                        .entry(route.clone())
                        .and_modify(|indegree_cnt| *indegree_cnt += 1);
                    self.direct_nexts
                        .entry(parent_route.clone())
                        .or_insert(Vec::new())
                        .push(route.clone());
                    self.direct_lasts
                        .insert(route.clone(), ModuleAliasTo::InCell(parent_route.clone()));
                    self.push_from_direct(parent_route);
                }
                None => (),
            }
        }
    }

    pub fn push_from_alias(&mut self, def_route: Route, source_route: Route) {
        self.push_from_direct(def_route.clone());
        if source_route.root == def_route.root {
            self.push_from_direct(source_route.clone());
        }
        if !self.indegrees.contains_key(&def_route) {
            self.indegrees.insert(def_route.clone(), 0);
        }
        if !self.indegrees.contains_key(&source_route) && source_route.root == def_route.root {
            self.indegrees.insert(source_route.clone(), 0);
        }
        if source_route.root == def_route.root {
            self.indegrees
                .entry(def_route.clone())
                .and_modify(|indegree_cnt| *indegree_cnt += 1);
        }
        self.alias_nexts
            .entry(source_route.clone())
            .or_insert(Vec::new())
            .push(def_route.clone());
        self.alias_lasts.insert(
            def_route.clone(),
            if source_route.root == def_route.root {
                ModuleAliasTo::InCell(source_route)
            } else {
                ModuleAliasTo::ExCell(source_route)
            },
        );
    }
}

impl GrandModuleSkeletonTable {
    pub fn module_skeleton_resolution(
        &mut self,
        cell_sign: &CellSignature,
        mut mskt: ModuleSkeletonTable,
    ) {
        let mut graph = mskt.module_skeletons.iter().fold(
            ModuleAliasGraph::new(),
            |mut graph, (module_route, skeleton)| {
                skeleton
                    .exported_submodules
                    .iter()
                    .chain(
                        skeleton.internal_submodules.iter().chain(
                            skeleton
                                .exported_concrete_definitions
                                .iter()
                                .chain(skeleton.internal_concrete_definitions.iter()),
                        ),
                    )
                    .for_each(|route| {
                        graph.push_from_direct(route.clone());
                    });

                skeleton
                    .exported_alias_definitions
                    .iter()
                    .chain(skeleton.internal_alias_definitions.iter())
                    .for_each(|(def_route, source_route)| {
                        graph.push_from_alias(def_route.clone(), source_route.clone());
                    });

                graph
            },
        );

        let mut q = graph
            .indegrees
            .iter()
            .filter(|(_, indegree_cnt)| **indegree_cnt == 0)
            .map(|(route, _)| route.clone())
            .collect::<VecDeque<_>>();

        while let Some(route) = q.pop_front() {
            if let Some(SkeletonDefSignature::Concrete(csk_def_sign)) =
                mskt.def_signatures.get(&route)
            {
                graph.nodes.insert(
                    route.clone(),
                    ModuleAliasResolutionResult::Resolved(csk_def_sign.def_route.clone()),
                );
            } else {
                match (
                    graph.direct_lasts.get(&route),
                    graph.alias_lasts.get(&route),
                ) {
                    (Some(ModuleAliasTo::InCell(direct_last_unresolved_route)), None) => {
                        match graph.nodes.get(direct_last_unresolved_route).unwrap() {
                            ModuleAliasResolutionResult::Resolved(direct_last_resolve_route) => {
                                let this_resolve_route = CanonicalRoute::chain_one(
                                    direct_last_resolve_route.clone(),
                                    route.get_last_name(),
                                );
                                if cell_sign == &direct_last_resolve_route.root
                                    && mskt
                                        .module_skeletons
                                        .get(&direct_last_resolve_route)
                                        .unwrap()
                                        .contains_concrete(&this_resolve_route)
                                {
                                    graph.nodes.insert(
                                        route.clone(),
                                        ModuleAliasResolutionResult::Resolved(this_resolve_route),
                                    );
                                } else if cell_sign != &direct_last_resolve_route.root {
                                    match self.lookup_canonical_route(
                                        &this_resolve_route,
                                        &direct_last_resolve_route,
                                    ) {
                                        LookUpCanonicalRouteResult::Resolved(resolved_route) => {
                                            graph.nodes.insert(
                                                route.clone(),
                                                ModuleAliasResolutionResult::Resolved(
                                                    resolved_route,
                                                ),
                                            );
                                        }
                                        LookUpCanonicalRouteResult::VisibilityDenial => {
                                            graph.nodes.insert(
                                                route.clone(),
                                                ModuleAliasResolutionResult::VisibilityDenial,
                                            );
                                        }
                                        LookUpCanonicalRouteResult::NameNotFound => {
                                            graph.nodes.insert(
                                                route.clone(),
                                                ModuleAliasResolutionResult::NameNotFound,
                                            );
                                        }
                                    }
                                } else {
                                    graph.nodes.insert(
                                        route.clone(),
                                        ModuleAliasResolutionResult::NameNotFound,
                                    );
                                }
                            }
                            ModuleAliasResolutionResult::VisibilityDenial => {
                                graph.nodes.insert(
                                    route.clone(),
                                    ModuleAliasResolutionResult::VisibilityDenial,
                                );
                            }
                            ModuleAliasResolutionResult::NameNotFound => {
                                graph.nodes.insert(
                                    route.clone(),
                                    ModuleAliasResolutionResult::NameNotFound,
                                );
                            }
                            ModuleAliasResolutionResult::Unresolved => unreachable!(),
                        }
                    }
                    (
                        Some(ModuleAliasTo::InCell(direct_last_unresolved_route)),
                        Some(ModuleAliasTo::InCell(alias_last_unresolved_route)),
                    ) => {
                        match (
                            graph.nodes.get(direct_last_unresolved_route).unwrap(),
                            graph.nodes.get(alias_last_unresolved_route).unwrap(),
                        ) {
                            (ModuleAliasResolutionResult::NameNotFound, _)
                            | (_, ModuleAliasResolutionResult::NameNotFound) => {
                                graph.nodes.insert(
                                    route.clone(),
                                    ModuleAliasResolutionResult::NameNotFound,
                                );
                            }
                            (ModuleAliasResolutionResult::VisibilityDenial, _)
                            | (_, ModuleAliasResolutionResult::VisibilityDenial) => {
                                graph.nodes.insert(
                                    route.clone(),
                                    ModuleAliasResolutionResult::VisibilityDenial,
                                );
                            }
                            (ModuleAliasResolutionResult::Unresolved, _)
                            | (_, ModuleAliasResolutionResult::Unresolved) => unreachable!(),
                            (
                                ModuleAliasResolutionResult::Resolved(direct_last_resolve_route),
                                ModuleAliasResolutionResult::Resolved(alias_last_resolve_route),
                            ) => {
                                let this_resolve_route = CanonicalRoute::chain_one(
                                    direct_last_resolve_route.clone(),
                                    route.get_last_name(),
                                );

                                assert_eq!(route, this_resolve_route);

                                match self.is_canonical_route_viewable(
                                    cell_sign,
                                    if &alias_last_unresolved_route.root != cell_sign {
                                        self.cell_module_skeleton_table
                                            .get(&alias_last_unresolved_route.root)
                                            .unwrap()
                                    } else {
                                        &mskt
                                    },
                                    &graph,
                                    &alias_last_unresolved_route,
                                    &direct_last_resolve_route,
                                ) {
                                    IsCanonicalRouteViewableResult::Viewable(_) => {
                                        graph.nodes.insert(
                                            route.clone(),
                                            ModuleAliasResolutionResult::Resolved(
                                                alias_last_resolve_route.clone(),
                                            ),
                                        );
                                    }
                                    IsCanonicalRouteViewableResult::VisibilityDenial => {
                                        graph.nodes.insert(
                                            route.clone(),
                                            ModuleAliasResolutionResult::VisibilityDenial,
                                        );
                                    }
                                    IsCanonicalRouteViewableResult::NameNotFound => {
                                        graph.nodes.insert(
                                            route.clone(),
                                            ModuleAliasResolutionResult::NameNotFound,
                                        );
                                    }
                                }
                            }
                        }
                    }
                    (
                        Some(ModuleAliasTo::InCell(direct_last_unresolved_route)),
                        Some(ModuleAliasTo::ExCell(alias_last_unresolved_route)),
                    ) => {
                        match (
                            graph.nodes.get(direct_last_unresolved_route).unwrap(),
                            self.lookup_canonical_route(
                                alias_last_unresolved_route,
                                direct_last_unresolved_route,
                            ),
                        ) {
                            (ModuleAliasResolutionResult::NameNotFound, _)
                            | (_, LookUpCanonicalRouteResult::NameNotFound) => {
                                graph.nodes.insert(
                                    route.clone(),
                                    ModuleAliasResolutionResult::NameNotFound,
                                );
                            }
                            (ModuleAliasResolutionResult::VisibilityDenial, _)
                            | (_, LookUpCanonicalRouteResult::VisibilityDenial) => {
                                graph.nodes.insert(
                                    route.clone(),
                                    ModuleAliasResolutionResult::VisibilityDenial,
                                );
                            }
                            (ModuleAliasResolutionResult::Unresolved, _) => unreachable!(),
                            (
                                ModuleAliasResolutionResult::Resolved(direct_last_resolve_route),
                                LookUpCanonicalRouteResult::Resolved(alias_last_resolve_route),
                            ) => {
                                let this_resolve_route = CanonicalRoute::chain_one(
                                    direct_last_resolve_route.clone(),
                                    route.get_last_name(),
                                );

                                assert_eq!(route, this_resolve_route);

                                match self.is_canonical_route_viewable(
                                    cell_sign,
                                    if &alias_last_unresolved_route.root != cell_sign {
                                        self.cell_module_skeleton_table
                                            .get(&alias_last_unresolved_route.root)
                                            .unwrap()
                                    } else {
                                        &mskt
                                    },
                                    &graph,
                                    &alias_last_unresolved_route,
                                    &direct_last_resolve_route,
                                ) {
                                    IsCanonicalRouteViewableResult::Viewable(_) => {
                                        graph.nodes.insert(
                                            route.clone(),
                                            ModuleAliasResolutionResult::Resolved(
                                                alias_last_resolve_route.clone(),
                                            ),
                                        );
                                    }
                                    IsCanonicalRouteViewableResult::VisibilityDenial => {
                                        graph.nodes.insert(
                                            route.clone(),
                                            ModuleAliasResolutionResult::VisibilityDenial,
                                        );
                                    }
                                    IsCanonicalRouteViewableResult::NameNotFound => {
                                        graph.nodes.insert(
                                            route.clone(),
                                            ModuleAliasResolutionResult::NameNotFound,
                                        );
                                    }
                                }
                            }
                        }
                    }
                    (_, _) => unreachable!(), // starting nodes should be concrete
                }
            }

            graph
                .direct_nexts
                .get(&route)
                .unwrap_or(&vec![].into())
                .iter()
                .chain(
                    graph
                        .alias_nexts
                        .get(&route)
                        .unwrap_or(&vec![].into())
                        .iter(),
                )
                .for_each(|next_route| {
                    graph
                        .indegrees
                        .entry(next_route.clone())
                        .and_modify(|indegree_cnt| *indegree_cnt -= 1);
                    if *graph.indegrees.get(next_route).unwrap() == 0 {
                        q.push_back(next_route.clone());
                    }
                })
        }

        self.cell_module_skeleton_table.insert(
            cell_sign.clone(),
            ModuleSkeletonTable {
                root: mskt.root,
                module_skeletons: mskt.module_skeletons,
                def_signatures: mskt.def_signatures.iter().fold(
                    HashMap::new(),
                    |mut new_def_signatures, (route, sk_def_sign)| {
                        match sk_def_sign {
                            SkeletonDefSignature::Concrete(c_sk_def_sign) => {
                                new_def_signatures.insert(
                                    route.clone(),
                                    SkeletonDefSignature::Concrete(c_sk_def_sign.clone()),
                                );
                            }
                            SkeletonDefSignature::UnresolvedAlias(ua_sk_def_sign) => {
                                match graph.nodes.get(&route).unwrap() {
                                    ModuleAliasResolutionResult::Resolved(resolved_route) => {
                                        if resolved_route.root == *cell_sign {
                                            new_def_signatures.insert(
                                                route.clone(),
                                                SkeletonDefSignature::ResolvedAlias(
                                                    ResolvedAliasSkeletonDefSignature {
                                                        is_exported: ua_sk_def_sign.is_exported,
                                                        position: ua_sk_def_sign.position.clone(),
                                                        kind: mskt
                                                            .def_signatures
                                                            .get(&resolved_route)
                                                            .unwrap()
                                                            .get_resolved_kind(),
                                                        def_route: ua_sk_def_sign.def_route.clone(),
                                                        resolved_route: resolved_route.clone(),
                                                    },
                                                ),
                                            );
                                        } else {
                                            new_def_signatures.insert(
                                                route.clone(),
                                                SkeletonDefSignature::ResolvedAlias(
                                                    ResolvedAliasSkeletonDefSignature {
                                                        is_exported: ua_sk_def_sign.is_exported,
                                                        position: ua_sk_def_sign.position.clone(),
                                                        kind: self
                                                            .cell_module_skeleton_table
                                                            .get(&resolved_route.root)
                                                            .unwrap()
                                                            .def_signatures
                                                            .get(&resolved_route)
                                                            .unwrap()
                                                            .get_resolved_kind(),
                                                        def_route: ua_sk_def_sign.def_route.clone(),
                                                        resolved_route: resolved_route.clone(),
                                                    },
                                                ),
                                            );
                                        }
                                    }
                                    ModuleAliasResolutionResult::Unresolved => self.errors.0.push(
                                        GlobalNameAliasResolutionError::circular_alias(
                                            ua_sk_def_sign.def_route.get_parent_route().unwrap(),
                                            ua_sk_def_sign.def_route.clone(),
                                            ua_sk_def_sign.position.clone(),
                                        ),
                                    ),
                                    ModuleAliasResolutionResult::VisibilityDenial => self
                                        .errors
                                        .0
                                        .push(GlobalNameAliasResolutionError::visibility_denial(
                                            ua_sk_def_sign.def_route.get_parent_route().unwrap(),
                                            ua_sk_def_sign.def_route.clone(),
                                            ua_sk_def_sign.position.clone(),
                                        )),
                                    ModuleAliasResolutionResult::NameNotFound => {
                                        self.errors.0.push(
                                            GlobalNameAliasResolutionError::alias_name_not_found(
                                                ua_sk_def_sign
                                                    .def_route
                                                    .get_parent_route()
                                                    .unwrap(),
                                                ua_sk_def_sign.def_route.clone(),
                                                ua_sk_def_sign.position.clone(),
                                            ),
                                        )
                                    }
                                }
                            }
                            SkeletonDefSignature::ResolvedAlias(_) => unreachable!(),
                            SkeletonDefSignature::ErrorAlias(_) => (),
                        }

                        new_def_signatures
                    },
                ),
                errors: mskt.errors,
            },
        );
    }
}

enum IsCanonicalRouteViewableIntermediateState {
    Viewable(CanonicalRoute),
    InProcess(CanonicalRoute, VecDeque<String>),
    VisibilityDenial,
    NameNotFound,
}

impl IsCanonicalRouteViewableIntermediateState {
    pub fn new_from_route(route: Route) -> Self {
        if route.path.len() == 0 {
            IsCanonicalRouteViewableIntermediateState::Viewable(route)
        } else {
            IsCanonicalRouteViewableIntermediateState::InProcess(
                CanonicalRoute::new(route.root),
                VecDeque::from(route.path),
            )
        }
    }
}

impl IsCanonicalRouteViewableIntermediateState {
    pub fn is_error(&self) -> bool {
        match self {
            IsCanonicalRouteViewableIntermediateState::VisibilityDenial
            | IsCanonicalRouteViewableIntermediateState::NameNotFound => true,
            _ => false,
        }
    }

    pub fn next_potential_route(&self) -> Option<CanonicalRoute> {
        if let IsCanonicalRouteViewableIntermediateState::InProcess(
            canonical_prefix_route,
            unresolved_route_fractions,
        ) = self
        {
            Some(Route::chain_one(
                canonical_prefix_route.clone(),
                unresolved_route_fractions.get(0).unwrap().clone(),
            ))
        } else {
            None
        }
    }

    pub fn resolve_next(&mut self, new_resolved: CanonicalRoute) {
        let IsCanonicalRouteViewableIntermediateState::InProcess(_, unresolved_route_fractions) =
            self
        else {
            unreachable!()
        };
        unresolved_route_fractions.pop_front();

        *self = if unresolved_route_fractions.len() == 0 {
            IsCanonicalRouteViewableIntermediateState::Viewable(new_resolved)
        } else {
            IsCanonicalRouteViewableIntermediateState::InProcess(
                new_resolved,
                unresolved_route_fractions.clone(),
            )
        }
    }

    pub fn into_visibility_denial_error(&mut self) {
        *self = IsCanonicalRouteViewableIntermediateState::VisibilityDenial;
    }

    pub fn into_name_not_found_error(&mut self) {
        *self = IsCanonicalRouteViewableIntermediateState::NameNotFound;
    }
}

pub enum IsCanonicalRouteViewableResult {
    Viewable(CanonicalRoute),
    VisibilityDenial,
    NameNotFound,
}

impl From<IsCanonicalRouteViewableIntermediateState> for IsCanonicalRouteViewableResult {
    fn from(state: IsCanonicalRouteViewableIntermediateState) -> Self {
        match state {
            IsCanonicalRouteViewableIntermediateState::Viewable(canonical_route) => {
                IsCanonicalRouteViewableResult::Viewable(canonical_route)
            }
            IsCanonicalRouteViewableIntermediateState::InProcess(_, _) => unreachable!(),
            IsCanonicalRouteViewableIntermediateState::VisibilityDenial => {
                IsCanonicalRouteViewableResult::VisibilityDenial
            }
            IsCanonicalRouteViewableIntermediateState::NameNotFound => {
                IsCanonicalRouteViewableResult::NameNotFound
            }
        }
    }
}

impl GrandModuleSkeletonTable {
    fn is_child_route_viewable(
        &self,
        cell_sign: &CellSignature,
        mskt: &ModuleSkeletonTable,
        msk: &ModuleSkeleton,
        graph: &ModuleAliasGraph,
        msk_route: &CanonicalRoute,
        child_route: &Route,
        view_route: &Route,
    ) -> IsCanonicalRouteViewableResult {
        if msk.exported_submodules.contains(child_route)
            || msk.exported_concrete_definitions.contains(child_route)
        {
            return IsCanonicalRouteViewableResult::Viewable(child_route.clone());
        }
        if let Some(aliased_route) = msk.exported_alias_definitions.get(child_route) {
            if aliased_route.root == *cell_sign {
                match graph.nodes.get(&aliased_route).unwrap() {
                    ModuleAliasResolutionResult::Resolved(resolved_aliased_route) => {
                        if resolved_aliased_route.root != *cell_sign {
                            return match self
                                .lookup_canonical_route(&resolved_aliased_route, msk_route)
                            {
                                LookUpCanonicalRouteResult::Resolved(resolved_route) => {
                                    IsCanonicalRouteViewableResult::Viewable(resolved_route)
                                }
                                LookUpCanonicalRouteResult::VisibilityDenial => {
                                    IsCanonicalRouteViewableResult::VisibilityDenial
                                }
                                LookUpCanonicalRouteResult::NameNotFound => {
                                    IsCanonicalRouteViewableResult::NameNotFound
                                }
                            };
                        } else {
                            return self.is_canonical_route_viewable(
                                cell_sign,
                                mskt,
                                graph,
                                &resolved_aliased_route,
                                msk_route,
                            );
                        }
                    }
                    ModuleAliasResolutionResult::Unresolved => unreachable!(),
                    ModuleAliasResolutionResult::VisibilityDenial => {
                        return IsCanonicalRouteViewableResult::VisibilityDenial
                    }
                    ModuleAliasResolutionResult::NameNotFound => {
                        return IsCanonicalRouteViewableResult::NameNotFound
                    }
                }
            } else {
                return match self.lookup_canonical_route(&aliased_route, msk_route) {
                    LookUpCanonicalRouteResult::Resolved(resolved_route) => {
                        IsCanonicalRouteViewableResult::Viewable(resolved_route)
                    }
                    LookUpCanonicalRouteResult::VisibilityDenial => {
                        IsCanonicalRouteViewableResult::VisibilityDenial
                    }
                    LookUpCanonicalRouteResult::NameNotFound => {
                        IsCanonicalRouteViewableResult::NameNotFound
                    }
                };
            }
        }

        if msk.internal_submodules.contains(&child_route)
            || msk.internal_concrete_definitions.contains(&child_route)
        {
            return if Route::is_ancestor(msk_route, view_route) {
                IsCanonicalRouteViewableResult::Viewable(child_route.clone())
            } else {
                IsCanonicalRouteViewableResult::VisibilityDenial
            };
        }
        if let Some(aliased_route) = msk.internal_alias_definitions.get(child_route) {
            if Route::is_ancestor(msk_route, view_route) {
                if aliased_route.root == *cell_sign {
                    match graph.nodes.get(&aliased_route).unwrap() {
                        ModuleAliasResolutionResult::Resolved(resolved_aliased_route) => {
                            if resolved_aliased_route.root != *cell_sign {
                                return match self
                                    .lookup_canonical_route(&resolved_aliased_route, msk_route)
                                {
                                    LookUpCanonicalRouteResult::Resolved(resolved_route) => {
                                        IsCanonicalRouteViewableResult::Viewable(resolved_route)
                                    }
                                    LookUpCanonicalRouteResult::VisibilityDenial => {
                                        IsCanonicalRouteViewableResult::VisibilityDenial
                                    }
                                    LookUpCanonicalRouteResult::NameNotFound => {
                                        IsCanonicalRouteViewableResult::NameNotFound
                                    }
                                };
                            } else {
                                return self.is_canonical_route_viewable(
                                    cell_sign,
                                    mskt,
                                    graph,
                                    &resolved_aliased_route,
                                    msk_route,
                                );
                            }
                        }
                        ModuleAliasResolutionResult::Unresolved => unreachable!(),
                        ModuleAliasResolutionResult::VisibilityDenial => {
                            return IsCanonicalRouteViewableResult::VisibilityDenial
                        }
                        ModuleAliasResolutionResult::NameNotFound => {
                            return IsCanonicalRouteViewableResult::NameNotFound
                        }
                    }
                } else {
                    return match self.lookup_canonical_route(&aliased_route, msk_route) {
                        LookUpCanonicalRouteResult::Resolved(resolved_route) => {
                            IsCanonicalRouteViewableResult::Viewable(resolved_route)
                        }
                        LookUpCanonicalRouteResult::VisibilityDenial => {
                            IsCanonicalRouteViewableResult::VisibilityDenial
                        }
                        LookUpCanonicalRouteResult::NameNotFound => {
                            IsCanonicalRouteViewableResult::NameNotFound
                        }
                    };
                }
            } else {
                return IsCanonicalRouteViewableResult::VisibilityDenial;
            };
        }

        IsCanonicalRouteViewableResult::NameNotFound
    }

    pub fn is_canonical_route_viewable(
        &self,
        cell_sign: &CellSignature,
        mskt: &ModuleSkeletonTable,
        graph: &ModuleAliasGraph,
        route: &Route,
        view_route: &CanonicalRoute,
    ) -> IsCanonicalRouteViewableResult {
        let mut state = IsCanonicalRouteViewableIntermediateState::new_from_route(route.clone());
        while let Some(next_potential_route) = state.next_potential_route() {
            let current_resolved_route = next_potential_route.get_parent_route().unwrap();
            match mskt.module_skeletons.get(&current_resolved_route) {
                None => state.into_name_not_found_error(),
                Some(msk) => {
                    match self.is_child_route_viewable(
                        cell_sign,
                        mskt,
                        msk,
                        graph,
                        &current_resolved_route,
                        &next_potential_route,
                        view_route,
                    ) {
                        IsCanonicalRouteViewableResult::VisibilityDenial => {
                            state.into_visibility_denial_error();
                        }
                        IsCanonicalRouteViewableResult::NameNotFound => {
                            state.into_name_not_found_error();
                        }
                        IsCanonicalRouteViewableResult::Viewable(next_resolved_route) => {
                            state.resolve_next(next_resolved_route);
                        }
                    }
                }
            }
        }

        IsCanonicalRouteViewableResult::from(state)
    }
}
