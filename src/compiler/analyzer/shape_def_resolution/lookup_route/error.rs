use crate::compiler::analyzer::common::route::route::Route;
use crate::compiler::analyzer::global_name_resolution::def_signature::SkeletonDefSignatureKind;
use crate::compiler::parser::parser::ParsedPosition;
use crate::utility::common::vec_add::AddableVec;
use std::fmt;

#[derive(Debug, Clone)]
pub enum LookUpRouteErrorKind {
    RouteExistsButKindDifferent {
        route: Route,
        expected: SkeletonDefSignatureKind,
        found: SkeletonDefSignatureKind,
    },
    RouteNotFound,
    UnknownRootRouteFraction,
    SingleCellOrRelativeKeywordRouteNotAllowed,
}

#[derive(Debug, Clone)]
pub struct LookUpRouteError {
    pub kind: LookUpRouteErrorKind,
    pub position: ParsedPosition,
}

impl fmt::Display for LookUpRouteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            LookUpRouteErrorKind::RouteExistsButKindDifferent {
                route,
                expected,
                found,
            } => {
                write!(
                    f,
                    "[{}] Route {} should have kind {}, but found {}",
                    self.position.print_path_line_column_span(),
                    route,
                    expected,
                    found,
                )
            }
            LookUpRouteErrorKind::RouteNotFound => {
                write!(
                    f,
                    "[{}] Route was not found",
                    self.position.print_path_line_column_span(),
                )
            }
            LookUpRouteErrorKind::UnknownRootRouteFraction => {
                write!(
                    f,
                    "[{}] Route cannot be parsed as its root is unknown",
                    self.position.print_path_line_column_span(),
                )
            }
            LookUpRouteErrorKind::SingleCellOrRelativeKeywordRouteNotAllowed => {
                write!(
                    f,
                    "[{}] Route cannot be parsed as cell name or relative keyword root (this, super) cannot form a reachable route",
                    self.position.print_path_line_column_span(),
                )
            }
        }
    }
}

impl LookUpRouteError {
    pub fn route_exists_but_kind_different(
        position: ParsedPosition,
        route: Route,
        expected: SkeletonDefSignatureKind,
        found: SkeletonDefSignatureKind,
    ) -> Self {
        Self {
            kind: LookUpRouteErrorKind::RouteExistsButKindDifferent {
                route,
                expected,
                found,
            },
            position,
        }
    }

    pub fn route_not_found(position: ParsedPosition) -> Self {
        Self {
            kind: LookUpRouteErrorKind::RouteNotFound,
            position,
        }
    }

    pub fn unknown_root_route_fraction(position: ParsedPosition) -> Self {
        Self {
            kind: LookUpRouteErrorKind::UnknownRootRouteFraction,
            position,
        }
    }

    pub fn single_cell_or_relative_keyword_route_not_allowed(position: ParsedPosition) -> Self {
        Self {
            kind: LookUpRouteErrorKind::SingleCellOrRelativeKeywordRouteNotAllowed,
            position,
        }
    }
}

pub type LookUpRouteErrors = AddableVec<LookUpRouteError>;
