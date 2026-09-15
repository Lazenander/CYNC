use crate::compiler::analyzer::shape_def_resolution::summary::GrandShapeContentSummaryAtWork;

pub mod bind_content;
pub mod coerce_content;
pub mod error;
pub mod operation_content;
pub mod operator_content;
pub mod shape_content;
pub mod template_call_content;
pub mod template_def_content;

impl GrandShapeContentSummaryAtWork<'_> {
    pub fn new_contents(&mut self) {
        let (shape_def_stmts, operation_def_stmts, bind_def_stmts, operator_stmts, coerce_stmts) =
            self.grand_module_skeleton_table
                .cell_module_skeleton_table
                .iter()
                .fold(
                    (vec![], vec![], vec![], vec![], vec![]),
                    |(shape_acc, operation_acc, bind_acc, operator_acc, coerce_acc),
                     (cell_sign, mst)| {
                        mst.module_skeletons.iter().fold(
                            (shape_acc, operation_acc, bind_acc, operator_acc, coerce_acc),
                            |(shape_acc, operation_acc, bind_acc, operator_acc, coerce_acc),
                             (module_route, _)| {
                                let mm = self
                                    .grand_module_merge_table
                                    .cell_module_merge_tables
                                    .get_mut(cell_sign)
                                    .unwrap()
                                    .m_modules
                                    .remove(module_route)
                                    .unwrap();
                                (
                                    shape_acc
                                        .into_iter()
                                        .chain(
                                            mm.shape_def_stmts
                                                .into_iter()
                                                .map(|(_, def)| (module_route.clone(), def)),
                                        )
                                        .collect(),
                                    operation_acc
                                        .into_iter()
                                        .chain(
                                            mm.operation_def_stmts
                                                .into_iter()
                                                .map(|(_, def)| (module_route.clone(), def)),
                                        )
                                        .collect(),
                                    bind_acc
                                        .into_iter()
                                        .chain(
                                            mm.bind_def_stmts
                                                .into_iter()
                                                .map(|(_, def)| (module_route.clone(), def)),
                                        )
                                        .collect(),
                                    operator_acc
                                        .into_iter()
                                        .chain(
                                            mm.operator_stmts
                                                .into_iter()
                                                .map(|def| (module_route.clone(), def)),
                                        )
                                        .collect(),
                                    coerce_acc
                                        .into_iter()
                                        .chain(
                                            mm.coerce_stmts
                                                .into_iter()
                                                .map(|def| (module_route.clone(), def)),
                                        )
                                        .collect(),
                                )
                            },
                        )
                    },
                );

        shape_def_stmts.into_iter().for_each(|(module_route, def)| {
            self.new_shape(module_route.clone(), def);
        });
        operation_def_stmts
            .into_iter()
            .for_each(|(module_route, def)| {
                self.new_operation(module_route.clone(), def);
            });
        bind_def_stmts.into_iter().for_each(|(module_route, def)| {
            self.new_bind(module_route.clone(), def);
        });
        operator_stmts.into_iter().for_each(|(module_route, def)| {
            self.new_operator(module_route.clone(), def);
        });
        coerce_stmts.into_iter().for_each(|(module_route, def)| {
            self.new_coerce(module_route.clone(), def);
        });
    }
}
