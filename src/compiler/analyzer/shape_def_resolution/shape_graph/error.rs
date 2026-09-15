use crate::compiler::parser::parser::ParsedPosition;
use crate::utility::common::vec_add::AddableVec;
use std::fmt;

#[derive(Debug, Clone)]
pub enum ShapeGraphErrorKind {
    IsSubShapeStuck {
        subshape: String,
        supershape: String,
    },
    IsSubShapeOverCost {
        subshape: String,
        supershape: String,
    },
}

#[derive(Debug, Clone)]
pub struct ShapeGraphError {
    pub kind: ShapeGraphErrorKind,
    pub position: ParsedPosition,
}

impl fmt::Display for ShapeGraphError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            ShapeGraphErrorKind::IsSubShapeStuck {
                subshape,
                supershape,
            } => {
                write!(
                    f,
                    "[{}] Subshape computation terminate due to stuck during comparing: {}, {}",
                    self.position.print_path_line_column_span(),
                    subshape,
                    supershape,
                )
            }
            ShapeGraphErrorKind::IsSubShapeOverCost {
                subshape,
                supershape,
            } => {
                write!(
                    f,
                    "[{}] Subshape computation terminate due to over cost during comparing: {}, {}",
                    self.position.print_path_line_column_span(),
                    subshape,
                    supershape,
                )
            }
        }
    }
}

impl ShapeGraphError {
    pub fn is_sub_shape_stuck(
        position: ParsedPosition,
        subshape: String,
        supershape: String,
    ) -> Self {
        ShapeGraphError {
            kind: ShapeGraphErrorKind::IsSubShapeStuck {
                subshape,
                supershape,
            },
            position,
        }
    }

    pub fn is_sub_shape_over_cost(
        position: ParsedPosition,
        subshape: String,
        supershape: String,
    ) -> Self {
        ShapeGraphError {
            kind: ShapeGraphErrorKind::IsSubShapeStuck {
                subshape,
                supershape,
            },
            position,
        }
    }
}

pub type ShapeGraphErrors = AddableVec<ShapeGraphError>;
