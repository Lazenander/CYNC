use crate::compiler::analyzer::common::route::route::{CanonicalRoute, Route};
use crate::compiler::analyzer::global_name_resolution::def_signature::SkeletonDefSignature;
use crate::compiler::analyzer::global_name_resolution::error::GlobalNameAliasResolutionErrorKind;
use crate::compiler::analyzer::global_name_resolution::module_skeleton::{
    GrandModuleSkeletonTable, ModuleSkeleton, ModuleSkeletonTable,
};
use std::collections::VecDeque;

enum LookUpCanonicalRouteIntermediateState {
    Resolved(CanonicalRoute),
    Unresolved(CanonicalRoute, VecDeque<String>),
    VisibilityDenial,
    NameNotFound,
}

impl LookUpCanonicalRouteIntermediateState {
    pub fn new_from_route(route: Route) -> Self {
        if route.path.len() == 0 {
            LookUpCanonicalRouteIntermediateState::Resolved(route)
        } else {
            LookUpCanonicalRouteIntermediateState::Unresolved(
                CanonicalRoute::new(route.root),
                VecDeque::from(route.path),
            )
        }
    }
}

impl LookUpCanonicalRouteIntermediateState {
    pub fn is_error(&self) -> bool {
        match self {
            LookUpCanonicalRouteIntermediateState::VisibilityDenial
            | LookUpCanonicalRouteIntermediateState::NameNotFound => true,
            _ => false,
        }
    }

    pub fn next_potential_route(&self) -> Option<CanonicalRoute> {
        if let LookUpCanonicalRouteIntermediateState::Unresolved(
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
        let LookUpCanonicalRouteIntermediateState::Unresolved(_, unresolved_route_fractions) = self
        else {
            unreachable!()
        };
        unresolved_route_fractions.pop_front();

        *self = if unresolved_route_fractions.len() == 0 {
            LookUpCanonicalRouteIntermediateState::Resolved(new_resolved)
        } else {
            LookUpCanonicalRouteIntermediateState::Unresolved(
                new_resolved,
                unresolved_route_fractions.clone(),
            )
        }
    }

    pub fn into_visibility_denial_error(&mut self) {
        *self = LookUpCanonicalRouteIntermediateState::VisibilityDenial;
    }

    pub fn into_name_not_found_error(&mut self) {
        *self = LookUpCanonicalRouteIntermediateState::NameNotFound;
    }
}

pub enum LookUpCanonicalRouteResult {
    Resolved(CanonicalRoute),
    VisibilityDenial,
    NameNotFound,
}

impl From<LookUpCanonicalRouteIntermediateState> for LookUpCanonicalRouteResult {
    fn from(state: LookUpCanonicalRouteIntermediateState) -> Self {
        match state {
            LookUpCanonicalRouteIntermediateState::Resolved(canonical_route) => {
                LookUpCanonicalRouteResult::Resolved(canonical_route)
            }
            LookUpCanonicalRouteIntermediateState::Unresolved(_, _) => unreachable!(),
            LookUpCanonicalRouteIntermediateState::VisibilityDenial => {
                LookUpCanonicalRouteResult::VisibilityDenial
            }
            LookUpCanonicalRouteIntermediateState::NameNotFound => {
                LookUpCanonicalRouteResult::NameNotFound
            }
        }
    }
}

impl GrandModuleSkeletonTable {
    fn lookup_child_route(
        &self,
        mskt: &ModuleSkeletonTable,
        msk_route: &CanonicalRoute,
        child_route: &Route,
        view_route: &Route,
    ) -> LookUpCanonicalRouteResult {
        let msk = mskt.module_skeletons.get(msk_route).unwrap();
        if msk.exported_submodules.contains(child_route)
            || msk.exported_concrete_definitions.contains(child_route)
        {
            return LookUpCanonicalRouteResult::Resolved(child_route.clone());
        }
        if let Some(aliased_route) = msk.exported_alias_definitions.get(child_route) {
            match mskt.def_signatures.get(&aliased_route).unwrap() {
                SkeletonDefSignature::Concrete(c_sk_def_sign) => {
                    return self.lookup_canonical_route(&c_sk_def_sign.def_route, msk_route);
                }
                SkeletonDefSignature::UnresolvedAlias(_) => unreachable!(),
                SkeletonDefSignature::ResolvedAlias(ra_sk_def_sign) => {
                    return self.lookup_canonical_route(&ra_sk_def_sign.resolved_route, msk_route);
                }
                SkeletonDefSignature::ErrorAlias(_) => {}
            }
        }

        if msk.internal_submodules.contains(&child_route)
            || msk.internal_concrete_definitions.contains(&child_route)
        {
            return if Route::is_ancestor(msk_route, view_route) {
                LookUpCanonicalRouteResult::Resolved(child_route.clone())
            } else {
                LookUpCanonicalRouteResult::VisibilityDenial
            };
        }
        if let Some(aliased_route) = msk.internal_alias_definitions.get(child_route) {
            if Route::is_ancestor(msk_route, view_route) {
                match mskt.def_signatures.get(&aliased_route).unwrap() {
                    SkeletonDefSignature::Concrete(c_sk_def_sign) => {
                        return self.lookup_canonical_route(&c_sk_def_sign.def_route, msk_route);
                    }
                    SkeletonDefSignature::UnresolvedAlias(_) => unreachable!(),
                    SkeletonDefSignature::ResolvedAlias(ra_sk_def_sign) => {
                        return self
                            .lookup_canonical_route(&ra_sk_def_sign.resolved_route, msk_route);
                    }
                    SkeletonDefSignature::ErrorAlias(_) => {}
                }
            } else {
                return LookUpCanonicalRouteResult::VisibilityDenial;
            };
        }

        LookUpCanonicalRouteResult::NameNotFound
    }

    pub fn lookup_canonical_route(
        &self,
        route: &Route,
        view_route: &CanonicalRoute,
    ) -> LookUpCanonicalRouteResult {
        if let Some(sk_def_sign) = self
            .cell_module_skeleton_table
            .get(&route.root)
            .unwrap()
            .def_signatures
            .get(&route)
        {
            return if let SkeletonDefSignature::ErrorAlias(err_kind) = sk_def_sign {
                match err_kind {
                    GlobalNameAliasResolutionErrorKind::VisibilityDenied => {
                        LookUpCanonicalRouteResult::VisibilityDenial
                    }
                    _ => LookUpCanonicalRouteResult::NameNotFound,
                }
            } else {
                LookUpCanonicalRouteResult::Resolved(sk_def_sign.get_resolved_route())
            };
        }

        let mut state = LookUpCanonicalRouteIntermediateState::new_from_route(route.clone());
        while let Some(next_potential_route) = state.next_potential_route() {
            let current_resolved_route = next_potential_route.get_parent_route().unwrap();
            let mskt = self
                .cell_module_skeleton_table
                .get(&current_resolved_route.root)
                .unwrap();
            match mskt.module_skeletons.get(&current_resolved_route) {
                None => state.into_name_not_found_error(),
                Some(_) => {
                    match self.lookup_child_route(
                        mskt,
                        &current_resolved_route,
                        &next_potential_route,
                        view_route,
                    ) {
                        LookUpCanonicalRouteResult::VisibilityDenial => {
                            state.into_visibility_denial_error();
                        }
                        LookUpCanonicalRouteResult::NameNotFound => {
                            state.into_name_not_found_error();
                        }
                        LookUpCanonicalRouteResult::Resolved(next_resolved_route) => {
                            state.resolve_next(next_resolved_route);
                        }
                    }
                }
            }
        }

        LookUpCanonicalRouteResult::from(state)
    }
}

impl GrandModuleSkeletonTable {
    pub fn lookup_def_signature(&self, route: &CanonicalRoute) -> Option<&SkeletonDefSignature> {
        self.cell_module_skeleton_table
            .get(&route.root)?
            .def_signatures
            .get(route)
    }
}
