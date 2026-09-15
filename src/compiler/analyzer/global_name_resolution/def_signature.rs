use crate::compiler::analyzer::common::route::route::{CanonicalRoute, Route};
use crate::compiler::analyzer::global_name_resolution::error::{
    GlobalNameAliasResolutionError, GlobalNameAliasResolutionErrorKind,
};
use crate::compiler::parser::parser::{ParsedBox, ParsedPosition};
use crate::compiler::parser::stmts::definition_stmts::bind::BindDef;
use crate::compiler::parser::stmts::definition_stmts::operation::OperationDef;
use crate::compiler::parser::stmts::definition_stmts::r#use::Use;
use crate::compiler::parser::stmts::definition_stmts::shape::ShapeDef;
use core::fmt;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SkeletonDefSignatureKind {
    Shape,
    Operation,
    Module,
    BindDef,
}

impl fmt::Display for SkeletonDefSignatureKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SkeletonDefSignatureKind::Shape => write!(f, "shape"),
            SkeletonDefSignatureKind::Operation => write!(f, "operation"),
            SkeletonDefSignatureKind::Module => write!(f, "module"),
            SkeletonDefSignatureKind::BindDef => write!(f, "bind"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ConcreteSkeletonDefSignature {
    pub is_exported: bool,
    pub position: ParsedPosition,
    pub kind: SkeletonDefSignatureKind,
    pub def_route: CanonicalRoute,
}

#[derive(Debug, Clone)]
pub struct UnresolvedAliasSkeletonDefSignature {
    pub is_exported: bool,
    pub position: ParsedPosition,
    pub def_route: CanonicalRoute,
    pub unresolved_route: CanonicalRoute,
}

#[derive(Debug, Clone)]
pub struct ResolvedAliasSkeletonDefSignature {
    pub is_exported: bool,
    pub position: ParsedPosition,
    pub kind: SkeletonDefSignatureKind,
    pub def_route: CanonicalRoute,
    pub resolved_route: CanonicalRoute,
}

#[derive(Debug, Clone)]
pub enum SkeletonDefSignature {
    Concrete(ConcreteSkeletonDefSignature),
    UnresolvedAlias(UnresolvedAliasSkeletonDefSignature),
    ResolvedAlias(ResolvedAliasSkeletonDefSignature),
    ErrorAlias(GlobalNameAliasResolutionErrorKind),
}

impl SkeletonDefSignature {
    pub fn push_from_pb_shape(def_route: CanonicalRoute, pb_shape: &ParsedBox<ShapeDef>) -> Self {
        SkeletonDefSignature::Concrete(ConcreteSkeletonDefSignature {
            is_exported: pb_shape.value().is_export,
            position: pb_shape.position.clone(),
            kind: SkeletonDefSignatureKind::Shape,
            def_route,
        })
    }

    pub fn push_from_pb_operation(
        def_route: CanonicalRoute,
        pb_operation: &ParsedBox<OperationDef>,
    ) -> Self {
        SkeletonDefSignature::Concrete(ConcreteSkeletonDefSignature {
            is_exported: pb_operation.value().is_export,
            position: pb_operation.position.clone(),
            kind: SkeletonDefSignatureKind::Operation,
            def_route,
        })
    }

    pub fn push_from_pb_bind(def_route: CanonicalRoute, pb_bind: &ParsedBox<BindDef>) -> Self {
        SkeletonDefSignature::Concrete(ConcreteSkeletonDefSignature {
            is_exported: pb_bind.value().is_export,
            position: pb_bind.position.clone(),
            kind: SkeletonDefSignatureKind::BindDef,
            def_route,
        })
    }

    pub fn push_from_pb_module(
        def_route: CanonicalRoute,
        pb_module_is_export: &ParsedBox<bool>,
    ) -> Self {
        SkeletonDefSignature::Concrete(ConcreteSkeletonDefSignature {
            is_exported: *pb_module_is_export.value(),
            position: pb_module_is_export.position.clone(),
            kind: SkeletonDefSignatureKind::Module,
            def_route,
        })
    }

    pub fn push_from_pb_use(
        def_route: CanonicalRoute,
        alias_route: Route,
        pb_use_is_export: &ParsedBox<Use>,
    ) -> Self {
        SkeletonDefSignature::UnresolvedAlias(UnresolvedAliasSkeletonDefSignature {
            is_exported: pb_use_is_export.value().is_export,
            position: pb_use_is_export.position.clone(),
            def_route,
            unresolved_route: alias_route,
        })
    }

    pub fn new_dummy_module(def_route: CanonicalRoute, is_exported: bool) -> Self {
        SkeletonDefSignature::Concrete(ConcreteSkeletonDefSignature {
            is_exported,
            position: ParsedPosition {
                path: std::path::PathBuf::from("<synthetic>"),
                start: 0,
                end: 0,
                start_line: 0,
                start_column: 0,
                end_line: 0,
                end_column: 0,
            },
            kind: SkeletonDefSignatureKind::Module,
            def_route,
        })
    }
}

impl SkeletonDefSignature {
    pub fn is_resolved(&self) -> bool {
        match &self {
            SkeletonDefSignature::Concrete(_) => true,
            SkeletonDefSignature::ResolvedAlias(_) => true,
            _ => false,
        }
    }

    pub fn get_resolved_route(&self) -> CanonicalRoute {
        match &self {
            SkeletonDefSignature::Concrete(concrete) => concrete.def_route.clone(),
            SkeletonDefSignature::ResolvedAlias(resolved) => resolved.resolved_route.clone(),
            SkeletonDefSignature::UnresolvedAlias(_) => unreachable!(),
            SkeletonDefSignature::ErrorAlias(_) => unreachable!(),
        }
    }

    pub fn get_resolved_kind(&self) -> SkeletonDefSignatureKind {
        match &self {
            SkeletonDefSignature::Concrete(concrete) => concrete.kind.clone(),
            SkeletonDefSignature::ResolvedAlias(resolved) => resolved.kind.clone(),
            SkeletonDefSignature::UnresolvedAlias(_) => unreachable!(),
            SkeletonDefSignature::ErrorAlias(_) => unreachable!(),
        }
    }

    pub fn get_position(&self) -> ParsedPosition {
        match &self {
            SkeletonDefSignature::Concrete(concrete) => concrete.position.clone(),
            SkeletonDefSignature::UnresolvedAlias(unresolved) => unresolved.position.clone(),
            SkeletonDefSignature::ResolvedAlias(resolved) => resolved.position.clone(),
            SkeletonDefSignature::ErrorAlias(_) => unreachable!(),
        }
    }

    pub fn is_exported(&self) -> bool {
        match &self {
            SkeletonDefSignature::Concrete(concrete) => concrete.is_exported,
            SkeletonDefSignature::UnresolvedAlias(unresolved) => unresolved.is_exported,
            SkeletonDefSignature::ResolvedAlias(resolved) => resolved.is_exported,
            SkeletonDefSignature::ErrorAlias(_) => unreachable!(),
        }
    }

    pub fn is_error(&self) -> bool {
        match &self {
            SkeletonDefSignature::ErrorAlias(_) => true,
            _ => false,
        }
    }

    pub fn is_not_error(&self) -> bool {
        !self.is_error()
    }
}
