use std::collections::{HashMap, HashSet};
use crate::compiler::analyzer::common::route::route::Route;
use crate::compiler::analyzer::shape_def_resolution::shape_graph::computable_shape::computable_shape::{ComputableShape, ComputableShapeGut, UnresolvedPShape, UnresolvedPShapeGut};
use crate::compiler::analyzer::shape_def_resolution::shape_graph::shape_graph::ShapeGraphNodeSignature;
use crate::compiler::analyzer::shape_def_resolution::shape_graph::subshape::subshape::IsSubShapeResult;
use crate::compiler::analyzer::shape_def_resolution::shape_signature::SpecifiedShapeDefSignature;
use crate::compiler::analyzer::shape_def_resolution::summary::{ComputableShapeKey, GenericParamDescriptionKey, GrandShapeContentSummaryAtWork};

pub struct SubShapeTableBuffer {
    pub proven_true: HashMap<ComputableShapeKey, HashSet<ComputableShapeKey>>,
    pub proven_false: HashMap<ComputableShapeKey, HashSet<ComputableShapeKey>>,
    pub proven_stuck: HashMap<ComputableShapeKey, HashSet<ComputableShapeKey>>,
    pub proven_overcost: HashMap<ComputableShapeKey, HashSet<ComputableShapeKey>>,
    pub assumption: HashMap<ComputableShapeKey, HashSet<ComputableShapeKey>>,
}

impl SubShapeTableBuffer {
    pub fn new() -> Self {
        Self {
            proven_true: HashMap::new(),
            proven_false: HashMap::new(),
            proven_stuck: HashMap::new(),
            proven_overcost: HashMap::new(),
            assumption: HashMap::new(),
        }
    }
}

impl GrandShapeContentSummaryAtWork<'_> {
    pub fn add_assumption(&mut self, subset: ComputableShapeKey, superset: ComputableShapeKey) {
        self.gsc_summary
            .sub_shape_table_buffer
            .assumption
            .entry(superset)
            .or_insert_with(HashSet::new)
            .insert(subset);
    }

    pub fn remove_assumption(
        &mut self,
        subset: &ComputableShapeKey,
        superset: &ComputableShapeKey,
    ) -> Option<()> {
        self.gsc_summary
            .sub_shape_table_buffer
            .assumption
            .get_mut(superset)?
            .remove(subset);
        Some(())
    }

    pub fn proven_true(&mut self, subset: ComputableShapeKey, superset: ComputableShapeKey) {
        self.remove_assumption(&subset, &superset).map(|_| {
            self.gsc_summary
                .sub_shape_table_buffer
                .proven_true
                .entry(superset)
                .or_insert_with(HashSet::new)
                .insert(subset);
        });
    }

    pub fn proven_false(&mut self, subset: ComputableShapeKey, superset: ComputableShapeKey) {
        self.remove_assumption(&subset, &superset).map(|_| {
            self.gsc_summary
                .sub_shape_table_buffer
                .proven_false
                .entry(superset)
                .or_insert_with(HashSet::new)
                .insert(subset);
        });
    }

    pub fn proven_stuck(&mut self, subset: ComputableShapeKey, superset: ComputableShapeKey) {
        self.remove_assumption(&subset, &superset).map(|_| {
            self.gsc_summary
                .sub_shape_table_buffer
                .proven_stuck
                .entry(superset)
                .or_insert_with(HashSet::new)
                .insert(subset);
        });
    }

    pub fn proven_overcost(&mut self, subset: ComputableShapeKey, superset: ComputableShapeKey) {
        self.remove_assumption(&subset, &superset).map(|_| {
            self.gsc_summary
                .sub_shape_table_buffer
                .proven_overcost
                .entry(superset)
                .or_insert_with(HashSet::new)
                .insert(subset);
        });
    }

    pub fn is_assumed(
        &mut self,
        subset: &ComputableShapeKey,
        superset: &ComputableShapeKey,
    ) -> bool {
        let o_assumptions = self
            .gsc_summary
            .sub_shape_table_buffer
            .assumption
            .get(superset);
        o_assumptions.is_some() && o_assumptions.unwrap().get(subset).is_some()
    }

    pub fn is_proven_true(
        &self,
        subset: &ComputableShapeKey,
        superset: &ComputableShapeKey,
    ) -> bool {
        let Some(subsets) = self
            .gsc_summary
            .sub_shape_table_buffer
            .proven_true
            .get(superset)
        else {
            return false;
        };
        subsets.get(subset).is_some()
    }

    pub fn is_proven_false(
        &self,
        subset: &ComputableShapeKey,
        superset: &ComputableShapeKey,
    ) -> bool {
        let Some(subsets) = self
            .gsc_summary
            .sub_shape_table_buffer
            .proven_false
            .get(superset)
        else {
            return false;
        };
        subsets.get(subset).is_some()
    }

    pub fn is_proven_stuck(
        &self,
        subset: &ComputableShapeKey,
        superset: &ComputableShapeKey,
    ) -> bool {
        let Some(subsets) = self
            .gsc_summary
            .sub_shape_table_buffer
            .proven_stuck
            .get(superset)
        else {
            return false;
        };
        subsets.get(subset).is_some()
    }

    pub fn is_proven_overcost(
        &self,
        subset: &ComputableShapeKey,
        superset: &ComputableShapeKey,
    ) -> bool {
        let Some(subsets) = self
            .gsc_summary
            .sub_shape_table_buffer
            .proven_overcost
            .get(superset)
        else {
            return false;
        };
        subsets.get(subset).is_some()
    }

    pub fn is_proven_true_or_assumed(
        &self,
        subset: &ComputableShapeKey,
        superset: &ComputableShapeKey,
    ) -> bool {
        let o_subsets = self
            .gsc_summary
            .sub_shape_table_buffer
            .proven_true
            .get(superset);
        let o_assumptions = self
            .gsc_summary
            .sub_shape_table_buffer
            .assumption
            .get(superset);

        (o_subsets.is_some() && o_subsets.unwrap().get(subset).is_some())
            || (o_assumptions.is_some() && o_assumptions.unwrap().get(subset).is_some())
    }

    pub fn find_proven_result_or_assumed(
        &self,
        subset: &ComputableShapeKey,
        superset: &ComputableShapeKey,
    ) -> Option<IsSubShapeResult> {
        if self.is_proven_true_or_assumed(subset, superset) {
            Some(IsSubShapeResult::True)
        } else if self.is_proven_false(subset, superset) {
            Some(IsSubShapeResult::False)
        } else if self.is_proven_stuck(subset, superset) {
            Some(IsSubShapeResult::Stuck)
        } else if self.is_proven_overcost(subset, superset) {
            Some(IsSubShapeResult::OverCost)
        } else {
            None
        }
    }
}
