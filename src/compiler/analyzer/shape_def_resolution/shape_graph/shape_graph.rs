use crate::compiler::analyzer::common::route::route::Route;
use crate::compiler::analyzer::shape_def_resolution::contents::bind_content::BindContentHeadKey;
use crate::compiler::analyzer::shape_def_resolution::contents::coerce_content::CoerceContentHeadKey;
use crate::compiler::analyzer::shape_def_resolution::contents::operator_content::OperatorContentHeadKey;
use crate::compiler::analyzer::shape_def_resolution::contents::shape_content::ShapeContentHeadKey;
use crate::compiler::analyzer::shape_def_resolution::description::shape_description::ElementaryShape;
use crate::compiler::analyzer::shape_def_resolution::summary::{
    ComputableShapeKey, GenericParamDescriptionKey, GrandShapeContentSummaryAtWork,
};
use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ShapeGraphNodeKey(usize);

pub enum AnnotatedSubshapeKind {
    ProvedTrue,
    ProvedFalse,
    Unproved,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ShapeGraphNodeSignature {
    Elementary(ElementaryShape),
    Specific(Route, Vec<ComputableShapeKey>),
}

pub struct ShapeGraphNode {
    key: ShapeGraphNodeKey,

    signatures: HashSet<ShapeGraphNodeSignature>,

    annotation_subshapes: HashMap<ShapeGraphNodeKey, AnnotatedSubshapeKind>,
    coerce: Vec<CoerceContentHeadKey>,

    bind_oprd: Vec<BindContentHeadKey>,
    operator_oprd1: Vec<OperatorContentHeadKey>,
    operator_oprd2: Vec<OperatorContentHeadKey>,
}

pub struct ShapeGraph {
    nodes: HashMap<ShapeGraphNodeKey, ShapeGraphNode>,

    from_def: HashMap<ShapeContentHeadKey, ShapeGraphNodeKey>,
}

impl ShapeGraph {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            from_def: HashMap::new(),
        }
    }
}

impl GrandShapeContentSummaryAtWork<'_> {
    pub fn connect_buffer_subshape_prove_true(
        &mut self,
        potential_subshape: ShapeGraphNodeSignature,
        potential_supershape: ShapeGraphNodeSignature,
    ) {
    }

    pub fn connect_buffer_subshape_prove_false(
        &mut self,
        potential_subshape: ShapeGraphNodeSignature,
        potential_supershape: ShapeGraphNodeSignature,
    ) {
    }

    pub fn prove_annotated_subshape_true(
        &mut self,
        potential_subshape: ShapeGraphNodeSignature,
        potential_supershape: ShapeGraphNodeSignature,
    ) {
    }

    pub fn prove_annotated_subshape_false(
        &mut self,
        potential_subshape: ShapeGraphNodeSignature,
        potential_supershape: ShapeGraphNodeSignature,
    ) {
    }

    pub fn is_subset_from_graph(
        &self,
        potential_subshape: ShapeGraphNodeSignature,
        potential_supershape: ShapeGraphNodeSignature,
    ) -> Option<bool> {
        // self.gsc_summary.graph.
    }
}
