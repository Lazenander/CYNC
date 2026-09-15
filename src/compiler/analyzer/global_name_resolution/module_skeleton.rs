use crate::compiler::analyzer::common::dependency_graph::dependency_graph::DependencyGraph;
use crate::compiler::analyzer::common::route::route::{ConcreteRoute, Route};
use crate::compiler::analyzer::common::route_root_table::route_root_table::RouteRootTable;
use crate::compiler::analyzer::global_name_resolution::def_signature::{
    ResolvedAliasSkeletonDefSignature, SkeletonDefSignature, SkeletonDefSignatureKind,
};
use crate::compiler::analyzer::global_name_resolution::error::{
    GlobalNameAliasResolutionError, GlobalNameAliasResolutionErrorKind,
    GlobalNameAliasResolutionErrors,
};
use crate::compiler::analyzer::module_merge::module_merge::{GrandModuleMergeTable, MergedModule};
use crate::compiler::analyzer::{
    common::route::route::CanonicalRoute, module_merge::module_merge::MergedModuleTable,
};
use crate::compiler::parser::exprs::identifier::RoutedIdentifier;
use crate::compiler::parser::parser::ParsedBox;
use crate::compiler::parser::stmts::definition_stmts::bind::BindDef;
use crate::compiler::parser::stmts::definition_stmts::operation::OperationDef;
use crate::compiler::parser::stmts::definition_stmts::r#use::Use;
use crate::compiler::parser::stmts::definition_stmts::shape::ShapeDef;
use crate::pipeline::cells::cell::CellSignature;
use crate::utility::consts::{
    RELATIVE_ROUTE_ROOT_KEYWORD_NAME_PARENT_MODULE, RELATIVE_ROUTE_ROOT_KEYWORD_NAME_THIS_CELL,
    RELATIVE_ROUTE_ROOT_KEYWORD_NAME_THIS_MODULE,
};
use clap::builder::TypedValueParser;
use std::collections::{HashMap, HashSet};

pub struct GrandModuleSkeletonTable {
    pub cell_module_skeleton_table: HashMap<CellSignature, ModuleSkeletonTable>,

    pub errors: GlobalNameAliasResolutionErrors,
}

#[derive(Debug, Clone)]
pub struct ModuleSkeletonTable {
    pub root: CanonicalRoute,
    pub module_skeletons: HashMap<CanonicalRoute, ModuleSkeleton>,
    pub def_signatures: HashMap<CanonicalRoute, SkeletonDefSignature>,
    pub errors: GlobalNameAliasResolutionErrors,
}

#[derive(Debug, Clone)]
pub struct ModuleSkeleton {
    pub this_route: CanonicalRoute,
    pub route_root_table: RouteRootTable,
    pub exported_submodules: HashSet<CanonicalRoute>,
    pub internal_submodules: HashSet<CanonicalRoute>,
    pub exported_concrete_definitions: HashSet<CanonicalRoute>,
    pub internal_concrete_definitions: HashSet<CanonicalRoute>,
    pub exported_alias_definitions: HashMap<CanonicalRoute, Route>,
    pub internal_alias_definitions: HashMap<CanonicalRoute, Route>,
}

impl ModuleSkeleton {
    pub fn contains_concrete(&self, route: &CanonicalRoute) -> bool {
        self.internal_submodules.contains(route)
            || self.exported_submodules.contains(route)
            || self.exported_concrete_definitions.contains(route)
            || self.internal_concrete_definitions.contains(route)
    }
}

impl GrandModuleSkeletonTable {
    pub fn new_empty() -> Self {
        Self {
            cell_module_skeleton_table: HashMap::new(),
            errors: vec![].into(),
        }
    }

    pub fn new(
        dependency_alias: HashMap<CellSignature, HashMap<CellSignature, String>>,
        dependency_graph: &DependencyGraph,
        grand_module_merge_table: &GrandModuleMergeTable,
    ) -> Self {
        let mut this = Self::new_empty();

        for sign in dependency_graph.post_order_iter() {
            let mut route_root_table = dependency_graph
                .lasts
                .get(&sign)
                .unwrap_or(&HashSet::new())
                .iter()
                .fold(RouteRootTable::new(), |mut rrt, d_sign| {
                    rrt.insert_common(
                        dependency_alias
                            .get(&sign)
                            .unwrap()
                            .get(&d_sign)
                            .unwrap()
                            .clone(),
                        CanonicalRoute::new(d_sign.clone()),
                    );
                    rrt
                });

            route_root_table.insert_common(sign.name.clone(), CanonicalRoute::new(sign.clone()));

            route_root_table.insert_relative_keyword(
                RELATIVE_ROUTE_ROOT_KEYWORD_NAME_THIS_CELL.into(),
                CanonicalRoute::new(sign.clone()),
            );

            let mmt = grand_module_merge_table
                .cell_module_merge_tables
                .get(&sign)
                .unwrap();
            let mskt = ModuleSkeletonTable::new(sign.clone(), &route_root_table, mmt);
            this.errors.append(mskt.errors.clone());
            this.module_skeleton_resolution(&sign, mskt);
        }

        this.into()
    }
}

pub enum RidToModuleRouteResult {
    Ok(Route),
    UnknownRootRouteFraction,
    SingleCellOrRelativeKeywordRouteNotAllowed,
}

impl ModuleSkeletonTable {
    pub fn rid_to_module_route_with_internal_rrt(
        &self,
        parent_module_route: CanonicalRoute,
        pb_rid: &ParsedBox<RoutedIdentifier>,
    ) -> RidToModuleRouteResult {
        self.rid_to_module_route(
            &self
                .module_skeletons
                .get(&parent_module_route)
                .unwrap()
                .route_root_table,
            pb_rid,
        )
    }

    pub fn rid_to_module_route(
        &self,
        rrt: &RouteRootTable,
        pb_rid: &ParsedBox<RoutedIdentifier>,
    ) -> RidToModuleRouteResult {
        let route_fractions = pb_rid.value().to_vec_string();
        let root_name = route_fractions.first().unwrap();

        if !rrt.contains(root_name) {
            RidToModuleRouteResult::UnknownRootRouteFraction
        } else if route_fractions.len() == 1 && rrt.is_common_or_keyword_root(root_name) {
            RidToModuleRouteResult::SingleCellOrRelativeKeywordRouteNotAllowed
        } else {
            let route_root = rrt
                .get_root_route(route_fractions.first().unwrap())
                .unwrap();
            RidToModuleRouteResult::Ok(CanonicalRoute::chain(
                route_root,
                route_fractions[1..].to_vec(),
            ))
        }
    }

    pub fn new(cell_sign: CellSignature, rrt: &RouteRootTable, mmt: &MergedModuleTable) -> Self {
        let root = CanonicalRoute::new_root(cell_sign);
        mmt.m_modules.iter().fold(
            Self {
                root: root.clone(),
                module_skeletons: HashMap::new(),
                def_signatures: vec![(
                    root.clone(),
                    SkeletonDefSignature::new_dummy_module(root.clone(), true),
                )]
                .into_iter()
                .collect(),
                errors: vec![].into(),
            },
            |mut this, (this_module_route, mm)| {
                let mut this_rrt = rrt.clone();
                let mut this_msk = ModuleSkeleton::new(this_module_route.clone());

                this_rrt.insert_relative_keyword(
                    RELATIVE_ROUTE_ROOT_KEYWORD_NAME_THIS_MODULE.into(),
                    this_module_route.clone(),
                );
                if let Some(parent_module_route) = this_module_route.get_parent_route() {
                    this_rrt.insert_relative_keyword(
                        RELATIVE_ROUTE_ROOT_KEYWORD_NAME_PARENT_MODULE.into(),
                        parent_module_route,
                    );
                }
                mm.shape_def_stmts.iter().for_each(|(name, pb_shape_def)| {
                    let def_route =
                        CanonicalRoute::chain_one(this_module_route.clone(), name.clone());
                    let skeleton_def_sig =
                        SkeletonDefSignature::push_from_pb_shape(def_route.clone(), pb_shape_def);

                    this_rrt.insert_relative_defined(name.clone(), def_route.clone());
                    this.def_signatures
                        .insert(def_route.clone(), skeleton_def_sig);
                    this_msk.add_from_pb_shape(def_route, pb_shape_def);
                });
                mm.operation_def_stmts
                    .iter()
                    .for_each(|(name, pb_operation_def)| {
                        let def_route =
                            CanonicalRoute::chain_one(this_module_route.clone(), name.clone());

                        this_rrt.insert_relative_defined(name.clone(), def_route.clone());
                        this.def_signatures.insert(
                            def_route.clone(),
                            SkeletonDefSignature::push_from_pb_operation(
                                def_route.clone(),
                                pb_operation_def,
                            ),
                        );
                        this_msk.add_from_pb_operation(def_route, pb_operation_def);
                    });
                mm.bind_def_stmts.iter().for_each(|(name, pb_bind_def)| {
                    let def_route =
                        CanonicalRoute::chain_one(this_module_route.clone(), name.clone());

                    this_rrt.insert_relative_defined(name.clone(), def_route.clone());
                    this.def_signatures.insert(
                        def_route.clone(),
                        SkeletonDefSignature::push_from_pb_bind(def_route.clone(), pb_bind_def),
                    );
                    this_msk.add_from_pb_bind(def_route, pb_bind_def);
                });
                mm.module_def_names.iter().for_each(|(name, pb_is_export)| {
                    let def_route =
                        CanonicalRoute::chain_one(this_module_route.clone(), name.clone());
                    this_rrt.insert_relative_defined(name.clone(), def_route.clone());
                    this.def_signatures.insert(
                        def_route.clone(),
                        SkeletonDefSignature::push_from_pb_module(def_route.clone(), pb_is_export),
                    );
                    this_msk.add_from_pb_module(def_route, pb_is_export);
                });
                mm.use_stmts.iter().for_each(|(name, pb_use)| {
                    let def_route =
                        CanonicalRoute::chain_one(this_module_route.clone(), name.clone());
                    this_rrt.insert_relative_defined(name.clone(), def_route);
                });

                mm.use_stmts.iter().for_each(|(name, pb_use)| {
                    let def_route =
                        CanonicalRoute::chain_one(this_module_route.clone(), name.clone());
                    let alias_route_result = this.rid_to_module_route(
                        &this_rrt,
                        &pb_use.value().routed_name,
                    );

                    match alias_route_result {
                        RidToModuleRouteResult::Ok(alias_route) => {
                            this.def_signatures.insert(
                                def_route.clone(),
                                SkeletonDefSignature::push_from_pb_use(
                                    def_route.clone(),
                                    alias_route.clone(),
                                    pb_use,
                                ),
                            );
                            this_msk.add_from_pb_use(def_route, alias_route, pb_use);
                        }
                        RidToModuleRouteResult::UnknownRootRouteFraction => {
                            this.errors
                                .0
                                .push(GlobalNameAliasResolutionError::unknown_root_route_fraction(
                                    this_module_route.clone(),
                                    pb_use.value().routed_name.value().to_vec_string(),
                                    pb_use.value().routed_name.position.clone(),
                                ));
                            this.def_signatures.insert(
                                def_route,
                                SkeletonDefSignature::ErrorAlias(
                                    GlobalNameAliasResolutionErrorKind::UnknownRootRouteFraction,
                                ),
                            );
                        }
                        RidToModuleRouteResult::SingleCellOrRelativeKeywordRouteNotAllowed => {
                            this.errors
                                .0
                                .push(GlobalNameAliasResolutionError::single_cell_or_relative_keyword_route_not_allowed(
                                    this_module_route.clone(),
                                    pb_use.value().routed_name.value().to_vec_string(),
                                    pb_use.value().routed_name.position.clone(),
                                ));
                            this.def_signatures.insert(
                                def_route,
                                SkeletonDefSignature::ErrorAlias(
                                    GlobalNameAliasResolutionErrorKind::UnknownRootRouteFraction,
                                ),
                            );}
                    }
                });

                this_msk.route_root_table = this_rrt;

                this.module_skeletons
                    .insert(this_module_route.clone(), this_msk);

                this
            },
        )
    }
}

impl ModuleSkeleton {
    pub fn new(module_route: CanonicalRoute) -> Self {
        Self {
            this_route: module_route.clone(),
            route_root_table: RouteRootTable::new(),
            exported_submodules: HashSet::new(),
            internal_submodules: HashSet::new(),
            exported_concrete_definitions: HashSet::new(),
            internal_concrete_definitions: HashSet::new(),
            exported_alias_definitions: HashMap::new(),
            internal_alias_definitions: HashMap::new(),
        }
    }

    pub fn add_from_pb_shape(&mut self, def_route: CanonicalRoute, pb_shape: &ParsedBox<ShapeDef>) {
        if pb_shape.value().is_export {
            self.exported_concrete_definitions.insert(def_route);
        } else {
            self.internal_concrete_definitions.insert(def_route);
        }
    }

    pub fn add_from_pb_operation(
        &mut self,
        def_route: CanonicalRoute,
        pb_operation: &ParsedBox<OperationDef>,
    ) {
        if pb_operation.value().is_export {
            self.exported_concrete_definitions.insert(def_route);
        } else {
            self.internal_concrete_definitions.insert(def_route);
        }
    }

    pub fn add_from_pb_bind(&mut self, def_route: CanonicalRoute, pb_bind: &ParsedBox<BindDef>) {
        if pb_bind.value().is_export {
            self.exported_concrete_definitions.insert(def_route);
        } else {
            self.internal_concrete_definitions.insert(def_route);
        }
    }

    pub fn add_from_pb_module(
        &mut self,
        def_route: CanonicalRoute,
        pb_module_is_export: &ParsedBox<bool>,
    ) {
        if *pb_module_is_export.value() {
            self.exported_submodules.insert(def_route);
        } else {
            self.internal_submodules.insert(def_route);
        }
    }

    pub fn add_from_pb_use(
        &mut self,
        def_route: CanonicalRoute,
        alias_route: Route,
        pb_use_is_export: &ParsedBox<Use>,
    ) {
        if pb_use_is_export.value().is_export {
            self.exported_alias_definitions
                .insert(def_route, alias_route);
        } else {
            self.internal_alias_definitions
                .insert(def_route, alias_route);
        }
    }
}
