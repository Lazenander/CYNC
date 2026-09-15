use crate::pipeline::cells::cell::CellSignature;
use crate::pipeline::cells::cell_registry::CellRegistry;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug)]
pub struct DependencyGraph {
    pub nodes: HashSet<CellSignature>,
    pub starts: Vec<CellSignature>,
    pub nexts: HashMap<CellSignature, HashSet<CellSignature>>,
    pub lasts: HashMap<CellSignature, HashSet<CellSignature>>,
}

impl DependencyGraph {
    pub fn new() -> Self {
        Self {
            nodes: HashSet::new(),
            starts: vec![],
            nexts: HashMap::new(),
            lasts: HashMap::new(),
        }
    }

    pub fn connect(&mut self, from: CellSignature, to: CellSignature) -> Result<(), CellSignature> {
        if !self.nodes.contains(&from) {
            return Err(from);
        }
        if !self.nodes.contains(&to) {
            return Err(to);
        }

        self.nexts
            .entry(from.clone())
            .or_insert_with(HashSet::new)
            .insert(to.clone());
        self.lasts
            .entry(to)
            .or_insert_with(HashSet::new)
            .insert(from);

        Ok(())
    }
}

impl DependencyGraph {
    pub fn post_order_iter(&self) -> DependencyTreeIterator {
        DependencyTreeIterator {
            nexts: &self.nexts,
            q: self.starts.iter().map(CellSignature::clone).collect(),
        }
    }
}

pub struct DependencyTreeIterator<'a> {
    nexts: &'a HashMap<CellSignature, HashSet<CellSignature>>,
    q: VecDeque<CellSignature>,
}

impl<'a> Iterator for DependencyTreeIterator<'a> {
    type Item = CellSignature;

    fn next(&mut self) -> Option<Self::Item> {
        if self.q.is_empty() {
            None
        } else {
            let top = self.q.pop_front().unwrap();
            self.nexts
                .get(&top)
                .unwrap()
                .iter()
                .for_each(|v| self.q.push_back(v.clone()));
            Some(top)
        }
    }
}
