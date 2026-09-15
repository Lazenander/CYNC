use crate::compiler::analyzer::common::route::route::{CanonicalRoute, Route, RouteTree};
use crate::compiler::analyzer::module_merge::error::{ModuleMergeError, ModuleMergeErrors};
use crate::compiler::parser::parser::{ParsedBox, ParsedPosition};
use crate::compiler::parser::program::program::Program;
use crate::compiler::parser::stmts::definition_stmts::bind::BindDef;
use crate::compiler::parser::stmts::definition_stmts::coerce::Coerce;
use crate::compiler::parser::stmts::definition_stmts::operation::OperationDef;
use crate::compiler::parser::stmts::definition_stmts::operator::OperatorOverload;
use crate::compiler::parser::stmts::definition_stmts::r#use::Use;
use crate::compiler::parser::stmts::definition_stmts::shape::ShapeDef;
use crate::compiler::parser::stmts::definition_stmts::{DefinitionStmt, DefinitionStmts, Module};
use crate::pipeline::cells::cell::CellSignature;
use std::collections::{HashMap, HashSet};
#[derive(Debug)]
pub struct MergedModule {
    pub module_def_names: HashMap<String, ParsedBox<bool>>,
    pub shape_def_stmts: HashMap<String, ParsedBox<ShapeDef>>,
    pub operation_def_stmts: HashMap<String, ParsedBox<OperationDef>>,
    pub bind_def_stmts: HashMap<String, ParsedBox<BindDef>>,
    pub coerce_stmts: Vec<ParsedBox<Coerce>>,
    pub operator_stmts: Vec<ParsedBox<OperatorOverload>>,
    pub use_stmts: Vec<(String, ParsedBox<Use>)>,

    pub errors: ModuleMergeErrors,
}

#[derive(Debug)]
pub struct MergedModuleTable {
    pub m_modules: HashMap<CanonicalRoute, MergedModule>,
}

#[derive(Default)]
pub struct GrandModuleMergeTable {
    pub cell_module_merge_tables: HashMap<CellSignature, MergedModuleTable>,
}

impl GrandModuleMergeTable {
    pub fn new_empty() -> Self {
        Self {
            cell_module_merge_tables: HashMap::new(),
        }
    }
}

trait NamePositionGettable {
    fn get_name(&self) -> String;
    fn get_name_position(&self) -> ParsedPosition;
}

impl NamePositionGettable for ParsedBox<Module> {
    fn get_name(&self) -> String {
        self.value.module_name.get_name()
    }

    fn get_name_position(&self) -> ParsedPosition {
        self.value.module_name.position.clone()
    }
}

impl NamePositionGettable for ParsedBox<ShapeDef> {
    fn get_name(&self) -> String {
        self.value.signature.value.id.get_name()
    }

    fn get_name_position(&self) -> ParsedPosition {
        self.value.signature.value.id.position.clone()
    }
}

impl NamePositionGettable for ParsedBox<OperationDef> {
    fn get_name(&self) -> String {
        self.value.name.get_name()
    }

    fn get_name_position(&self) -> ParsedPosition {
        self.value.name.position.clone()
    }
}

impl NamePositionGettable for ParsedBox<BindDef> {
    fn get_name(&self) -> String {
        self.value.name.get_name()
    }

    fn get_name_position(&self) -> ParsedPosition {
        self.value.name.position.clone()
    }
}

impl NamePositionGettable for ParsedBox<Use> {
    fn get_name(&self) -> String {
        self.value()
            .clone()
            .name
            .map(|pb_id| pb_id.get_name())
            .unwrap_or_else(|| self.value().routed_name.value().this_id.get_name())
    }

    fn get_name_position(&self) -> ParsedPosition {
        match self.value.name.clone() {
            Some(pb_id) => pb_id.position,
            None => self.value.routed_name.value().this_id.position.clone(),
        }
    }
}

impl MergedModule {
    pub fn new_empty() -> Self {
        Self {
            module_def_names: HashMap::new(),
            shape_def_stmts: HashMap::new(),
            operation_def_stmts: HashMap::new(),
            bind_def_stmts: HashMap::new(),
            coerce_stmts: Vec::new(),
            operator_stmts: Vec::new(),
            use_stmts: Vec::new(),

            errors: Vec::new().into(),
        }
    }

    pub fn push_error_by_pb<T, F>(&mut self, module_route: Route, pb_module: &T, gen_error: F)
    where
        T: NamePositionGettable,
        F: FnOnce(String, Route, ParsedPosition) -> ModuleMergeError,
    {
        self.errors.0.push(gen_error(
            pb_module.get_name(),
            module_route,
            pb_module.get_name_position(),
        ))
    }

    pub fn push_error_by_name_route_pos<F>(
        &mut self,
        name: String,
        module_route: Route,
        position: ParsedPosition,
        gen_error: F,
    ) where
        F: FnOnce(String, Route, ParsedPosition) -> ModuleMergeError,
    {
        self.errors.0.push(gen_error(name, module_route, position))
    }

    pub fn contain_name(&self, name: &String) -> bool {
        self.module_def_names.contains_key(name)
            || self.shape_def_stmts.contains_key(name)
            || self.operation_def_stmts.contains_key(name)
            || self.bind_def_stmts.contains_key(name)
            || self
                .use_stmts
                .iter()
                .fold(false, |flag, (o_name, _)| flag || o_name == name)
    }

    pub fn contain_non_module_name(&self, name: &String) -> bool {
        self.shape_def_stmts.contains_key(name)
            || self.operation_def_stmts.contains_key(name)
            || self.bind_def_stmts.contains_key(name)
            || self
                .use_stmts
                .iter()
                .fold(false, |flag, (o_name, _)| flag || o_name == name)
    }

    pub fn new(route: Route, module_stmts: DefinitionStmts) -> (Self, Vec<ParsedBox<Module>>) {
        let mut this = Self::new_empty();

        let (sub_module_stmts, non_sub_module_stmts): (Vec<_>, Vec<_>) = module_stmts
            .0
            .into_iter()
            .partition(|stmt| matches!(stmt.value(), DefinitionStmt::SubModule(_)));

        let sub_modules = sub_module_stmts.into_iter().fold(vec![], |v, module_stmt| {
            match module_stmt.owned_value() {
                DefinitionStmt::SubModule(sub_mod) => {
                    let name = sub_mod.get_name();
                    if this.contain_non_module_name(&name) {
                        this.push_error_by_pb(
                            route.clone(),
                            &sub_mod,
                            ModuleMergeError::module_merge_conflict,
                        )
                    } else {
                        this.module_def_names
                            .insert(name, sub_mod.ref_map(|v| v.is_export));
                    }
                    v.into_iter().chain(vec![sub_mod]).collect()
                }
                _ => unreachable!(),
            }
        });
        non_sub_module_stmts
            .into_iter()
            .for_each(|module_stmt| match module_stmt.owned_value() {
                DefinitionStmt::Empty => {}
                DefinitionStmt::Shape(shape_def) => {
                    let name = shape_def.get_name();
                    if this.contain_name(&name) {
                        this.push_error_by_pb(
                            route.clone(),
                            &shape_def,
                            ModuleMergeError::module_merge_conflict,
                        )
                    } else {
                        this.shape_def_stmts.insert(name, shape_def);
                    }
                }
                DefinitionStmt::Operation(operation_def) => {
                    let name = operation_def.get_name();
                    if this.contain_name(&name) {
                        this.push_error_by_pb(
                            route.clone(),
                            &operation_def,
                            ModuleMergeError::module_merge_conflict,
                        )
                    } else {
                        this.operation_def_stmts.insert(name, operation_def);
                    }
                }
                DefinitionStmt::BindDef(bind_def) => {
                    let name = bind_def.get_name();
                    if this.contain_name(&name) {
                        this.push_error_by_pb(
                            route.clone(),
                            &bind_def,
                            ModuleMergeError::module_merge_conflict,
                        )
                    } else {
                        this.bind_def_stmts.insert(name, bind_def);
                    }
                }
                DefinitionStmt::Coerce(coerce) => {
                    this.coerce_stmts.push(coerce);
                }
                DefinitionStmt::Operator(operator) => {
                    this.operator_stmts.push(operator);
                }
                DefinitionStmt::Use(use_def) => {
                    let name = use_def.get_name();
                    if this.contain_name(&name) {
                        this.push_error_by_pb(
                            route.clone(),
                            &use_def,
                            ModuleMergeError::module_merge_conflict,
                        )
                    } else {
                        this.use_stmts.push((name, use_def));
                    }
                }
                _ => unreachable!(),
            });
        (this, sub_modules)
    }

    pub fn merge(&mut self, route: Route, other: Self) {
        for (name, pb_is_export) in other.module_def_names {
            if self.contain_non_module_name(&name) {
                self.push_error_by_name_route_pos(
                    name,
                    route.clone(),
                    pb_is_export.position.clone(),
                    ModuleMergeError::module_merge_conflict,
                )
            } else {
                if !self.module_def_names.contains_key(&name) {
                    self.module_def_names.insert(name, pb_is_export);
                } else {
                    let original_is_export = self.module_def_names.get(&name).unwrap().value();
                    self.module_def_names.insert(
                        name,
                        pb_is_export.map(|is_export| *original_is_export || is_export),
                    );
                }
            }
        }
        for (name, shape_def) in other.shape_def_stmts {
            if self.contain_name(&name) {
                self.push_error_by_pb(
                    route.clone(),
                    &shape_def,
                    ModuleMergeError::module_merge_conflict,
                )
            } else {
                self.shape_def_stmts.insert(name, shape_def);
            }
        }
        for (name, operation_def) in other.operation_def_stmts {
            if self.contain_name(&name) {
                self.push_error_by_pb(
                    route.clone(),
                    &operation_def,
                    ModuleMergeError::module_merge_conflict,
                )
            } else {
                self.operation_def_stmts.insert(name, operation_def);
            }
        }
        for (name, bind_def) in other.bind_def_stmts {
            if self.contain_name(&name) {
                self.push_error_by_pb(
                    route.clone(),
                    &bind_def,
                    ModuleMergeError::module_merge_conflict,
                )
            } else {
                self.bind_def_stmts.insert(name, bind_def);
            }
        }
        self.coerce_stmts.extend(other.coerce_stmts);
        self.operator_stmts.extend(other.operator_stmts);
        for (name, use_def) in other.use_stmts {
            if self.contain_name(&name) {
                self.push_error_by_pb(
                    route.clone(),
                    &use_def,
                    ModuleMergeError::module_merge_conflict,
                )
            } else {
                self.use_stmts.push((name, use_def));
            }
        }
    }
}

impl MergedModuleTable {
    pub fn new() -> Self {
        Self {
            m_modules: HashMap::new(),
        }
    }

    pub fn merge_program(&mut self, cell_signature: CellSignature, file_parsed_result: Program) {
        let root_module_route = CanonicalRoute::new(cell_signature);
        let (root_merged_module, sub_modules) =
            MergedModule::new(root_module_route.clone(), file_parsed_result.stmts);
        sub_modules.into_iter().for_each(|sub_mod| {
            self.merge_module(root_module_route.clone(), sub_mod.owned_value())
        });
        if self.m_modules.contains_key(&root_module_route) {
            self.m_modules
                .get_mut(&root_module_route)
                .unwrap()
                .merge(root_module_route, root_merged_module);
        } else {
            self.m_modules.insert(root_module_route, root_merged_module);
        }
    }

    fn merge_module(&mut self, parent_module_route: CanonicalRoute, module: Module) {
        let this_mod_name = module.module_name.get_name();
        let this_mod_route =
            CanonicalRoute::chain(parent_module_route.clone(), vec![this_mod_name]);
        let (other, sub_modules) =
            MergedModule::new(parent_module_route.clone(), module.module_stmts);
        sub_modules
            .into_iter()
            .for_each(|sub_mod| self.merge_module(this_mod_route.clone(), sub_mod.owned_value()));
        if self.m_modules.contains_key(&this_mod_route) {
            self.m_modules
                .get_mut(&this_mod_route)
                .unwrap()
                .merge(parent_module_route, other);
        } else {
            self.m_modules.insert(this_mod_route, other);
        }
    }
}
