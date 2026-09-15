use crate::compiler::parser::exprs::identifier::{Identifier, RoutedIdentifier};
use crate::pipeline::cells::cell::CellSignature;
use std::collections::{btree_set, BTreeMap, BTreeSet, HashMap};
use std::fmt::Formatter;

#[derive(Debug, Clone, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub struct Route {
    pub root: CellSignature,
    pub path: Vec<String>,
}

impl std::fmt::Display for Route {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.path.is_empty() {
            write!(f, "[{}]", self.root.clone().to_string(),)
        } else {
            write!(
                f,
                "[{}] {}",
                self.root.clone().to_string(),
                self.path
                    .clone()
                    .into_iter()
                    .collect::<Vec<String>>()
                    .join("::")
            )
        }
    }
}

impl Route {
    pub fn new_root(root: CellSignature) -> Route {
        Route { root, path: vec![] }
    }

    pub fn new(root: CellSignature) -> Self {
        Self::new_from_vec_str(root, vec![])
    }

    pub fn new_from_vec_str(root: CellSignature, vec_str: Vec<String>) -> Self {
        Route {
            root,
            path: vec_str,
        }
    }

    pub fn chain(route: Route, new: Vec<String>) -> Route {
        Route {
            root: route.root.clone(),
            path: route
                .path
                .clone()
                .into_iter()
                .chain(new.into_iter())
                .collect(),
        }
    }

    pub fn chain_one(route: Route, new: String) -> Route {
        Route::chain(route, vec![new])
    }

    pub fn ignore_new_first_chain(route: &Route, new: Vec<String>) -> Option<Route> {
        if route.get_last_name() == new.first()?.clone() {
            Some(Route {
                root: route.root.clone(),
                path: route
                    .path
                    .clone()
                    .into_iter()
                    .chain(new[1..new.len()].into_iter().map(|v| v.clone()))
                    .collect(),
            })
        } else {
            None
        }
    }

    pub fn concat(original: Route, other: Vec<String>) -> Route {
        Route {
            root: original.root,
            path: original.path.into_iter().chain(other.into_iter()).collect(),
        }
    }

    pub fn get_parent_route(&self) -> Option<Self> {
        if self.path.len() == 0 {
            None
        } else {
            Some(Self {
                root: self.root.clone(),
                path: self.path[0..self.path.len() - 1].to_vec(),
            })
        }
    }

    pub fn get_last_name(&self) -> String {
        self.path
            .last()
            .cloned()
            .unwrap_or_else(|| self.root.name.clone())
    }
}

impl Route {
    pub fn is_ancestor(ancestor: &Self, precedence: &Self) -> bool {
        Self::is_prefix(ancestor, precedence)
    }

    pub fn get_cell_signature_root_route(&self) -> Self {
        self.get_route_prefix(0)
    }

    pub fn get_route_prefix(&self, index_size: usize) -> Self {
        Self {
            root: self.root.clone(),
            path: self.path[0..index_size].to_vec(),
        }
    }

    pub fn get_module_route(&self) -> Self {
        Self {
            root: self.root.clone(),
            path: self.path[0..self.path.len() - 1].to_vec(),
        }
    }

    pub fn is_prefix(s1: &Self, s2: &Self) -> bool {
        if s1.root != s2.root || s1.path.len() > s2.path.len() {
            return false;
        }
        for (str1, str2) in s1.path.iter().zip(s2.path[0..s1.path.len()].iter()) {
            if (str1 != str2) {
                return false;
            }
        }
        true
    }
}

impl Route {
    pub fn new_from_relative_path(
        name_routes: &HashMap<String, CanonicalRoute>,
        relative_path: Vec<String>,
    ) -> Option<CanonicalRoute> {
        name_routes
            .get(relative_path.first().unwrap())
            .map(|route| Route::ignore_new_first_chain(route, relative_path).unwrap())
    }

    pub fn get_one_entry_longer(
        root_route: CanonicalRoute,
        root_route_alternative: CanonicalRoute,
        extended_route: CanonicalRoute,
    ) -> Option<CanonicalRoute> {
        if extended_route.root != extended_route.root
            || root_route.path.len() >= extended_route.path.len()
        {
            return None;
        }
        let mut i = 0;
        while i < root_route.path.len() && root_route.path[i] == extended_route.path[i] {
            i += 1;
        }
        if i != root_route.path.len() {
            return None;
        }
        Some(CanonicalRoute {
            root: root_route_alternative.root.clone(),
            path: root_route_alternative
                .path
                .into_iter()
                .chain(vec![extended_route.path[i].clone()])
                .collect(),
        })
    }

    pub fn shadow_route(
        root_route: CanonicalRoute,
        shadow_route: CanonicalRoute,
        extended_route: CanonicalRoute,
    ) -> Option<CanonicalRoute> {
        if extended_route.root != extended_route.root
            || root_route.path.len() >= extended_route.path.len()
        {
            return None;
        }
        let mut i = 0;
        while i < root_route.path.len() && root_route.path[i] == extended_route.path[i] {
            i += 1;
        }
        if i != root_route.path.len() {
            return None;
        }
        Some(CanonicalRoute {
            root: shadow_route.root.clone(),
            path: shadow_route
                .path
                .into_iter()
                .chain(extended_route.path[i..].iter().map(|str| str.clone()))
                .collect(),
        })
    }
}

impl Route {
    pub fn into_prefix_iter(self) -> RoutePrefixIterator {
        let len = self.path.len();
        RoutePrefixIterator {
            route: self,
            current_len: Some(len),
        }
    }
}

pub struct RoutePrefixIterator {
    route: Route,
    current_len: Option<usize>,
}

impl Iterator for RoutePrefixIterator {
    type Item = Route;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current_len {
            Some(len) => {
                let result = Route {
                    root: self.route.root.clone(),
                    path: self.route.path[0..len].to_vec(),
                };

                if len == 0 {
                    self.current_len = None;
                } else {
                    self.current_len = Some(len - 1);
                }

                Some(result)
            }
            None => None,
        }
    }
}

#[derive(Debug, Clone, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub struct RouteTree {
    pub routes: BTreeSet<Route>,
    pub children: BTreeMap<String, RouteTree>,
}

impl RouteTree {
    pub fn into_post_order_iter(self) -> RouteTreeIterator {
        RouteTreeIterator {
            route_iter: self.routes.into_iter(),
            children_iter: self
                .children
                .into_iter()
                .map(|(_, v)| v.into_post_order_iter())
                .collect(),
            children_index: 0,
        }
    }
}

pub struct RouteTreeIterator {
    pub route_iter: btree_set::IntoIter<Route>,
    pub children_iter: Vec<RouteTreeIterator>,
    pub children_index: usize,
}

impl Iterator for RouteTreeIterator {
    type Item = Route;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(child) = self.children_iter.get_mut(self.children_index) {
                if let Some(x) = child.next() {
                    return Some(x);
                }
                self.children_index += 1;
            } else {
                return self.route_iter.next();
            }
        }
    }
}

impl<Routes> From<Routes> for RouteTree
where
    Routes: IntoIterator<Item = Route>,
{
    fn from(routes: Routes) -> Self {
        // routes must share the same route root
        routes
            .into_iter()
            .fold(RouteTree::new(), RouteTree::insertion)
    }
}

impl RouteTree {
    pub fn new() -> Self {
        RouteTree {
            routes: BTreeSet::new(),
            children: BTreeMap::new(),
        }
    }

    pub fn insertion(current_tree: Self, route: Route) -> Self {
        let mut new_tree = current_tree.to_owned();
        let mut tree_pointer = &mut new_tree;
        let route_len = route.path.len();
        for (route_fragment, cnt) in route.path.iter().zip(0..route_len) {
            if cnt == route_len - 1 {
                tree_pointer.routes.insert(route.clone());
            } else if !tree_pointer.children.contains_key(route_fragment) {
                tree_pointer
                    .children
                    .insert(route_fragment.clone(), RouteTree::new());
                tree_pointer = tree_pointer.children.get_mut(route_fragment).unwrap();
            } else {
                tree_pointer = tree_pointer.children.get_mut(route_fragment).unwrap();
            }
        }
        new_tree
    }
}

pub type CanonicalRoute = Route;
pub type ConcreteRoute = Route;
pub type AliasRoute = Route;

impl Into<Vec<String>> for RoutedIdentifier {
    fn into(self) -> Vec<String> {
        self.route
            .into_iter()
            .map(|x| x.owned_value().name)
            .chain(vec![self.this_id.owned_value().name])
            .collect()
    }
}

impl Into<String> for Identifier {
    fn into(self) -> String {
        self.name
    }
}
