use crate::compiler::analyzer::common::route::route::CanonicalRoute;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct RouteRootTable {
    common_root_table: HashMap<String, CanonicalRoute>,
    keyword_root_table: HashMap<String, CanonicalRoute>,
    relative_root_table: HashMap<String, CanonicalRoute>,
}

impl RouteRootTable {
    pub fn new() -> Self {
        Self {
            common_root_table: HashMap::new(),
            keyword_root_table: HashMap::new(),
            relative_root_table: HashMap::new(),
        }
    }

    pub fn insert_common(&mut self, root_name: String, route: CanonicalRoute) {
        self.relative_root_table.insert(root_name, route);
    }

    pub fn insert_relative_keyword(&mut self, root_name: String, route: CanonicalRoute) {
        self.relative_root_table.insert(root_name, route);
    }

    pub fn insert_relative_defined(&mut self, root_name: String, route: CanonicalRoute) {
        self.relative_root_table.insert(root_name, route);
    }

    pub fn is_common_or_keyword_root(&self, root_name: &String) -> bool {
        self.common_root_table.contains_key(root_name)
            || self.keyword_root_table.contains_key(root_name)
    }

    pub fn is_relative_root(&self, root_name: &String) -> bool {
        self.relative_root_table.contains_key(root_name)
    }

    pub fn contains(&self, root_name: &String) -> bool {
        self.is_common_or_keyword_root(root_name) || self.is_relative_root(root_name)
    }

    pub fn get_root_route(&self, root_name: &String) -> Option<CanonicalRoute> {
        self.common_root_table
            .get(root_name)
            .map(CanonicalRoute::clone)
            .or_else(|| {
                self.relative_root_table
                    .get(root_name)
                    .map(CanonicalRoute::clone)
            })
    }
}
