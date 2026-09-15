use crate::compiler::analyzer::common::route::route::{CanonicalRoute, Route};
use crate::compiler::analyzer::global_name_resolution::def_signature::{
    ConcreteSkeletonDefSignature, SkeletonDefSignature, SkeletonDefSignatureKind,
};
use crate::compiler::analyzer::global_name_resolution::lookup_canonical_route::LookUpCanonicalRouteResult;
use crate::compiler::analyzer::global_name_resolution::module_skeleton::RidToModuleRouteResult;
use crate::compiler::analyzer::shape_def_resolution::error::ShapeDefinitionResolutionError;
use crate::compiler::analyzer::shape_def_resolution::summary::GrandShapeContentSummaryAtWork;
use crate::compiler::parser::exprs::identifier::RoutedIdentifier;
use crate::compiler::parser::parser::ParsedBox;

pub enum RidToShapeRouteResult {
    Ok(Route),
    RouteExistsButKindDifferent {
        route: Route,
        expected: SkeletonDefSignatureKind,
        found: SkeletonDefSignatureKind,
    },
    RouteNotFound,
    UnknownRootRouteFraction,
    SingleCellOrRelativeKeywordRouteNotAllowed,
}

impl<'a> GrandShapeContentSummaryAtWork<'a> {
    pub fn add_error(&mut self, err: ShapeDefinitionResolutionError) {
        self.gsc_summary.errors.push(err);
    }
}

impl<'a> GrandShapeContentSummaryAtWork<'a> {
    pub fn lookup_canonical_route(
        &self,
        target_route: CanonicalRoute,
        view_route: CanonicalRoute,
    ) -> LookUpCanonicalRouteResult {
        self.grand_module_skeleton_table
            .lookup_canonical_route(&target_route, &view_route)
    }

    fn lookup_route_from_rid_naive(
        &self,
        pb_rid: ParsedBox<RoutedIdentifier>,
        view_route: CanonicalRoute,
    ) -> RidToModuleRouteResult {
        self.grand_module_skeleton_table
            .cell_module_skeleton_table
            .get(&view_route.root)
            .unwrap()
            .rid_to_module_route_with_internal_rrt(view_route, &pb_rid)
    }

    pub fn lookup_def_kind_from_rid(
        &self,
        pb_rid: ParsedBox<RoutedIdentifier>,
        expected_kind: SkeletonDefSignatureKind,
        view_route: CanonicalRoute,
    ) -> RidToShapeRouteResult {
        match self.lookup_route_from_rid_naive(pb_rid, view_route) {
            RidToModuleRouteResult::Ok(route) => {
                match self
                    .grand_module_skeleton_table
                    .lookup_def_signature(&route)
                    .unwrap()
                {
                    SkeletonDefSignature::Concrete(ConcreteSkeletonDefSignature {
                        is_exported: _,
                        kind,
                        position: _,
                        def_route: route,
                    }) => {
                        if kind == &expected_kind {
                            RidToShapeRouteResult::Ok(route.clone())
                        } else {
                            RidToShapeRouteResult::RouteExistsButKindDifferent {
                                route: route.clone(),
                                expected: expected_kind,
                                found: kind.clone(),
                            }
                        }
                    }
                    SkeletonDefSignature::ResolvedAlias(_)
                    | SkeletonDefSignature::UnresolvedAlias(_) => unreachable!(),
                    SkeletonDefSignature::ErrorAlias(_) => RidToShapeRouteResult::RouteNotFound,
                }
            }
            RidToModuleRouteResult::UnknownRootRouteFraction => {
                RidToShapeRouteResult::UnknownRootRouteFraction
            }
            RidToModuleRouteResult::SingleCellOrRelativeKeywordRouteNotAllowed => {
                RidToShapeRouteResult::SingleCellOrRelativeKeywordRouteNotAllowed
            }
        }
    }
}
