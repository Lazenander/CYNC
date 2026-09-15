use crate::compiler::analyzer::common::route::route::{CanonicalRoute, Route};
use crate::compiler::parser::parser::ParsedPosition;
use crate::utility::common::vec_add::AddableVec;
use std::fmt;

#[derive(Debug, Clone)]
pub enum GlobalNameAliasResolutionErrorKind {
    UnknownRootRouteFraction,
    SingleCellOrRelativeKeywordRouteNotAllowed,

    VisibilityDenied,
    AliasNameNotFound,

    CircularAlias,
}

#[derive(Debug, Clone)]
pub struct GlobalNameAliasResolutionError {
    pub kind: GlobalNameAliasResolutionErrorKind,
    pub parent_module_route: CanonicalRoute,
    pub route: Result<Route, Vec<String>>,
    pub position: ParsedPosition,
}

impl fmt::Display for GlobalNameAliasResolutionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            GlobalNameAliasResolutionErrorKind::UnknownRootRouteFraction => {
                write!(
                    f,
                    "[{}] Route \"{}\" cannot be parsed as its root is unknown",
                    self.position.print_path_line_column_span(),
                    self.route.clone().err().unwrap().join("::")
                )
            }
            GlobalNameAliasResolutionErrorKind::SingleCellOrRelativeKeywordRouteNotAllowed => {
                write!(
                    f,
                    "[{}] Route \"{}\" cannot be parsed as cell name or relative keyword root (this, super) cannot form a aliasable route",
                    self.position.print_path_line_column_span(),
                    self.route.clone().err().unwrap().join("::")
                )
            }
            GlobalNameAliasResolutionErrorKind::VisibilityDenied => {
                write!(
                    f,
                    "[{}] Name \"{}\" cannot be aliased due to visibility restrictions",
                    self.position.print_path_line_column_span(),
                    self.route.clone().unwrap()
                )
            }
            GlobalNameAliasResolutionErrorKind::AliasNameNotFound => {
                write!(
                    f,
                    "[{}] Name \"{}\" cannot be aliased due to name not being found",
                    self.position.print_path_line_column_span(),
                    self.route.clone().unwrap()
                )
            }
            GlobalNameAliasResolutionErrorKind::CircularAlias => {
                write!(
                    f,
                    "[{}] Name \"{}\" cannot be aliased due to circular aliasing",
                    self.position.print_path_line_column_span(),
                    self.route.clone().unwrap()
                )
            }
        }
    }
}

impl GlobalNameAliasResolutionError {
    pub fn unknown_root_route_fraction(
        parent_module_route: CanonicalRoute,
        pre_route: Vec<String>,
        position: ParsedPosition,
    ) -> Self {
        Self {
            kind: GlobalNameAliasResolutionErrorKind::UnknownRootRouteFraction,
            parent_module_route,
            route: Err(pre_route),
            position,
        }
    }

    pub fn single_cell_or_relative_keyword_route_not_allowed(
        parent_module_route: CanonicalRoute,
        pre_route: Vec<String>,
        position: ParsedPosition,
    ) -> Self {
        Self {
            kind: GlobalNameAliasResolutionErrorKind::SingleCellOrRelativeKeywordRouteNotAllowed,
            parent_module_route,
            route: Err(pre_route),
            position,
        }
    }

    pub fn visibility_denial(
        parent_module_route: CanonicalRoute,
        route: Route,
        position: ParsedPosition,
    ) -> Self {
        Self {
            kind: GlobalNameAliasResolutionErrorKind::VisibilityDenied,
            parent_module_route,
            route: Ok(route),
            position,
        }
    }

    pub fn alias_name_not_found(
        parent_module_route: CanonicalRoute,
        route: Route,
        position: ParsedPosition,
    ) -> Self {
        Self {
            kind: GlobalNameAliasResolutionErrorKind::AliasNameNotFound,
            parent_module_route,
            route: Ok(route),
            position,
        }
    }

    pub fn circular_alias(
        parent_module_route: CanonicalRoute,
        route: Route,
        position: ParsedPosition,
    ) -> Self {
        Self {
            kind: GlobalNameAliasResolutionErrorKind::CircularAlias,
            parent_module_route,
            route: Ok(route),
            position,
        }
    }
}

pub type GlobalNameAliasResolutionErrors = AddableVec<GlobalNameAliasResolutionError>;
