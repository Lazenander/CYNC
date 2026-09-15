use crate::compiler::analyzer::global_name_resolution::module_skeleton::GrandModuleSkeletonTable;
use crate::compiler::analyzer::module_merge::module_merge::GrandModuleMergeTable;
use crate::compiler::analyzer::shape_def_resolution::contents::bind_content::{
    BindContent, BindContentHeadKey,
};
use crate::compiler::analyzer::shape_def_resolution::contents::coerce_content::{
    CoerceContent, CoerceContentHeadKey,
};
use crate::compiler::analyzer::shape_def_resolution::contents::operation_content::{
    OperationContent, OperationContentHeadKey,
};
use crate::compiler::analyzer::shape_def_resolution::contents::operator_content::{
    OperatorContent, OperatorContentHeadKey,
};
use crate::compiler::analyzer::shape_def_resolution::contents::shape_content::{
    ShapeContent, ShapeContentHeadKey,
};
use crate::compiler::analyzer::shape_def_resolution::description::function_description::{
    ClosureDescription, FunctionDescription, FunctionDescriptionContext,
    InClosureDescriptionContext,
};
use crate::compiler::analyzer::shape_def_resolution::description::generic_param_description::GenericParamDescription;
use crate::compiler::analyzer::shape_def_resolution::description::shape_description::{ElementaryShape, ShapeDescription, ShapeDescriptionContext, CORE_TYPE_NAME_MAP};
use crate::compiler::analyzer::shape_def_resolution::error::ShapeDefinitionResolutionErrors;
use crate::compiler::analyzer::shape_def_resolution::shape_graph::computable_shape::computable_shape::{ComputableShape, ComputableShapeGut, ElementaryPShape};
use crate::compiler::analyzer::shape_def_resolution::shape_graph::shape_graph::ShapeGraph;
use crate::compiler::analyzer::shape_def_resolution::shape_graph::subshape_assumption_buffer::SubShapeTableBuffer;
use paste::paste;
use std::cmp::PartialEq;
use std::collections::HashMap;

#[macro_export]
macro_rules! create_struct_kv_tables {
    (
        $( { item: $Item:ident, stem: $stem:ident } ),+ $(,)?
    ) => {
        paste! {
            // Key newtypes
            $(
                #[repr(transparent)]
                #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
                pub struct [<$Item Key>](pub usize);

                impl ::std::convert::From<usize> for [<$Item Key>] {
                    #[inline]
                    fn from(i: usize) -> Self { Self(i) }
                }
                impl [<$Item Key>] {
                    #[inline]
                    pub const fn idx(self) -> usize { self.0 }
                }
            )*

            struct KVTables {
                $( pub [<$stem _table>]: ::std::vec::Vec<$Item>, )*
            }

            impl KVTables {
                fn new() -> Self {
                    Self {
                        $([<$stem _table>]: ::std::vec::Vec::new(),)*
                    }
                }
            }

            impl GrandShapeContentSummaryAtWork<'_> {
                $(
                    #[inline]
                    pub fn [<new_ $stem>](&mut self, value: $Item) -> [<$Item Key>] {
                        self.gsc_summary.kv_tables.[<$stem _table>].push(value);
                        [<$Item Key>](self.gsc_summary.kv_tables.[<$stem _table>].len() - 1)
                    }

                    #[inline]
                    pub fn [<get_ $stem>](&self, key: [<$Item Key>]) -> &$Item {
                        self.gsc_summary.kv_tables.[<$stem _table>].get(key.idx()).unwrap()
                    }

                    #[inline]
                    pub fn [<map_ $stem>]<T, F>(&self, key: [<$Item Key>], f: F) -> T
                    where F: FnOnce(&$Item) -> T {
                        f(self.gsc_summary.kv_tables.[<$stem _table>].get(key.idx()).unwrap())
                    }
                )*
            }
        }
    }
}

create_struct_kv_tables! {
    { item: ShapeContent, stem: shape_content },
    { item: OperationContent, stem: operation_content },
    { item: BindContent, stem: bind_content },
    { item: OperatorContent, stem: operator_content },
    { item: CoerceContent, stem: coerce_content },

    { item: GenericParamDescription, stem: generic_param_description },

    { item: ShapeDescription, stem: shape_description },
    { item: ShapeDescriptionContext, stem: shape_description_context },

    { item: FunctionDescription, stem: function_description },
    { item: FunctionDescriptionContext, stem: function_description_context },
    { item: ClosureDescription, stem: closure_description },
    { item: InClosureDescriptionContext, stem: in_closure_description_context },

    { item: ComputableShape, stem: computable_shape },
}

pub struct GrandShapeContentSummary {
    pub kv_tables: KVTables,

    pub shape_content_heads: HashMap<ShapeContentHeadKey, ShapeContentKey>,
    pub operation_content_heads: HashMap<OperationContentHeadKey, OperationContentKey>,
    pub bind_content_heads: HashMap<BindContentHeadKey, BindContentKey>,
    pub operator_content_heads: HashMap<OperatorContentHeadKey, Vec<OperatorContentKey>>,
    pub coerce_content_heads: HashMap<CoerceContentHeadKey, Vec<CoerceContentKey>>,

    pub computable_shape_table: HashMap<ShapeDescriptionKey, ComputableShapeKey>,

    pub ecs_key_table: HashMap<ElementaryShape, ComputableShapeKey>,

    pub sub_shape_table_buffer: SubShapeTableBuffer,

    pub bind_name_table: HashMap<ShapeContentHeadKey, HashMap<String, Vec<BindContentHeadKey>>>,

    pub graph: ShapeGraph,

    pub errors: ShapeDefinitionResolutionErrors,
}

impl GrandShapeContentSummary {
    pub fn new_empty() -> Self {
        Self {
            kv_tables: KVTables::new(),

            shape_content_heads: HashMap::new(),
            operation_content_heads: HashMap::new(),
            bind_content_heads: HashMap::new(),
            operator_content_heads: HashMap::new(),
            coerce_content_heads: HashMap::new(),

            computable_shape_table: HashMap::new(),

            ecs_key_table: HashMap::new(),

            sub_shape_table_buffer: SubShapeTableBuffer::new(),

            bind_name_table: CORE_TYPE_NAME_MAP
                .values()
                .into_iter()
                .map(|ele_shape| {
                    (
                        ShapeContentHeadKey::Prelude(ele_shape.clone()),
                        HashMap::new(),
                    )
                })
                .collect(),

            graph: ShapeGraph::new(),

            errors: vec![].into(),
        }
    }
}

pub struct GrandShapeContentSummaryAtWork<'a> {
    pub grand_module_skeleton_table: &'a GrandModuleSkeletonTable,
    pub grand_module_merge_table: GrandModuleMergeTable,
    pub gsc_summary: GrandShapeContentSummary,
}

impl<'a> GrandShapeContentSummaryAtWork<'a> {
    pub fn new(
        grand_module_skeleton_table: &'a GrandModuleSkeletonTable,
        grand_module_merge_table: GrandModuleMergeTable,
    ) -> Self {
        let mut this = Self {
            grand_module_skeleton_table,
            grand_module_merge_table,
            gsc_summary: GrandShapeContentSummary::new_empty(),
        };

        this.generate_ecs_key_table();

        this
    }
}

impl GrandShapeContentSummaryAtWork<'_> {
    pub fn new_shape_content_head(
        &mut self,
        shape_key: ShapeContentHeadKey,
        shape_content: ShapeContent,
    ) {
        let key = self.new_shape_content(shape_content);
        self.gsc_summary.shape_content_heads.insert(shape_key, key);
    }

    pub fn new_operation_content_head(
        &mut self,
        operation_key: OperationContentHeadKey,
        operation_content: OperationContent,
    ) {
        let key = self.new_operation_content(operation_content);
        self.gsc_summary
            .operation_content_heads
            .insert(operation_key, key);
    }

    pub fn new_bind_content_head(
        &mut self,
        bind_key: BindContentHeadKey,
        bind_content: BindContent,
    ) {
        let key = self.new_bind_content(bind_content);
        self.gsc_summary.bind_content_heads.insert(bind_key, key);
    }

    pub fn insert_operator_content_head(
        &mut self,
        operator_key: OperatorContentHeadKey,
        operator_content: OperatorContent,
    ) {
        let key = self.new_operator_content(operator_content);
        self.gsc_summary
            .operator_content_heads
            .entry(operator_key)
            .or_insert(vec![])
            .push(key);
    }

    pub fn insert_coerce_content_head(
        &mut self,
        coerce_key: CoerceContentHeadKey,
        coerce_content: CoerceContent,
    ) {
        let key = self.new_coerce_content(coerce_content);
        self.gsc_summary
            .coerce_content_heads
            .entry(coerce_key)
            .or_insert(vec![])
            .push(key);
    }
}

impl GrandShapeContentSummaryAtWork<'_> {
    pub fn get_shape_content_head(&mut self, shape_key: &ShapeContentHeadKey) -> &ShapeContent {
        self.gsc_summary
            .shape_content_heads
            .get(shape_key)
            .map(|key| self.get_shape_content(*key))
            .unwrap()
    }

    pub fn get_operation_content_head(
        &mut self,
        operation_key: &OperationContentHeadKey,
    ) -> &OperationContent {
        self.gsc_summary
            .operation_content_heads
            .get(operation_key)
            .map(|key| self.get_operation_content(*key))
            .unwrap()
    }

    pub fn get_bind_content_head(&mut self, bind_key: &BindContentHeadKey) -> &BindContent {
        self.gsc_summary
            .bind_content_heads
            .get(bind_key)
            .map(|key| self.get_bind_content(*key))
            .unwrap()
    }

    pub fn get_operator_content_head(
        &mut self,
        operator_key: &OperatorContentHeadKey,
    ) -> Vec<&OperatorContent> {
        self.gsc_summary
            .operator_content_heads
            .get(operator_key)
            .map(|keys| {
                keys.iter()
                    .map(|key| self.get_operator_content(*key))
                    .collect()
            })
            .unwrap()
    }

    pub fn get_coerce_content_head(
        &mut self,
        coerce_key: &CoerceContentHeadKey,
    ) -> Vec<&CoerceContent> {
        self.gsc_summary
            .coerce_content_heads
            .get(coerce_key)
            .map(|keys| {
                keys.iter()
                    .map(|key| self.get_coerce_content(*key))
                    .collect()
            })
            .unwrap()
    }
}

impl GrandShapeContentSummaryAtWork<'_> {
    pub fn get_computable_shape_key(
        &mut self,
        shape_key: &ShapeDescriptionKey,
    ) -> &ComputableShapeKey {
        if !self.has_simple_shape(shape_key) {
            let computable_shape = self.build_computable_shape_from_description(shape_key);
            self.gsc_summary
                .computable_shape_table
                .insert(*shape_key, computable_shape);
        }
        self.gsc_summary
            .computable_shape_table
            .get(shape_key)
            .unwrap()
    }

    pub fn has_simple_shape(&mut self, shape_key: &ShapeDescriptionKey) -> bool {
        self.gsc_summary
            .computable_shape_table
            .contains_key(shape_key)
    }
}

impl GrandShapeContentSummaryAtWork<'_> {
    pub fn generate_ecs_key_table(&mut self) {
        self.gsc_summary.ecs_key_table = CORE_TYPE_NAME_MAP
            .into_iter()
            .map(|(_, shape)| {
                (
                    shape.clone(),
                    self.new_computable_shape(ComputableShape {
                        gut: ComputableShapeGut::Elementary(ElementaryPShape {
                            gut: shape.clone(),
                            position: None,
                        }),
                        position: None,
                    }),
                )
            })
            .collect();
    }

    pub fn ask_for_ecs_key(&self, ele_shape: &ElementaryShape) -> &ComputableShapeKey {
        self.gsc_summary.ecs_key_table.get(ele_shape).unwrap()
    }
}
