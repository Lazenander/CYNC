use crate::compiler::analyzer::shape_def_resolution::description::shape_description::{
    CharWidth, ElementaryShape, FloatWidth, IntWidth, ShapeDescriptionGut, ShapeVarianceAnnotation,
    UnresolvedShape,
};
use crate::compiler::analyzer::shape_def_resolution::summary::{
    GrandShapeContentSummaryAtWork, ShapeDescriptionKey,
};
use std::fmt;
use std::fmt::Formatter;

impl fmt::Display for IntWidth {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            IntWidth::UnsignedBit8 => write!(f, "U8"),
            IntWidth::UnsignedBit16 => write!(f, "U16"),
            IntWidth::UnsignedBit32 => write!(f, "U32"),
            IntWidth::UnsignedBit64 => write!(f, "U64"),
            IntWidth::UnsignedBit128 => write!(f, "U128"),
            IntWidth::SignedBit16 => write!(f, "I16"),
            IntWidth::SignedBit32 => write!(f, "I32"),
            IntWidth::SignedBit64 => write!(f, "I64"),
            IntWidth::SignedBit128 => write!(f, "I128"),
            IntWidth::SignedBit256 => write!(f, "I256"),
        }
    }
}

impl fmt::Display for FloatWidth {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            FloatWidth::Bit16 => write!(f, "16"),
            FloatWidth::Bit32 => write!(f, "32"),
            FloatWidth::Bit64 => write!(f, "64"),
            FloatWidth::Bit128 => write!(f, "128"),
        }
    }
}

impl fmt::Display for CharWidth {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            CharWidth::Ascii => write!(f, "ASCII"),
            CharWidth::UTF16 => write!(f, "UTF16"),
        }
    }
}

impl fmt::Display for ElementaryShape {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ElementaryShape::Empty => write!(f, "Empty"),
            ElementaryShape::Unit => write!(f, "Unit"),
            ElementaryShape::Bool => write!(f, "Bool"),
            ElementaryShape::Int(width) => {
                write!(f, "{}", width)
            }
            ElementaryShape::Float(width) => {
                write!(f, "F{}", width)
            }

            ElementaryShape::Char(width) => {
                write!(f, "{}Char", width)
            }
            ElementaryShape::String(width) => {
                write!(f, "{}String", width)
            }
            ElementaryShape::Any => write!(f, "Any"),
        }
    }
}

impl fmt::Display for ShapeVarianceAnnotation {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ShapeVarianceAnnotation::Covariant => {
                write!(f, "+")
            }
            ShapeVarianceAnnotation::Contravariant => {
                write!(f, "-")
            }
            ShapeVarianceAnnotation::Invariant => {
                write!(f, "0")
            }
        }
    }
}

impl GrandShapeContentSummaryAtWork<'_> {
    pub fn display_unresolved_shape(&self, unresolved_shape: &UnresolvedShape) -> String {
        match unresolved_shape {
            UnresolvedShape::GenericVariable(g_var) => format!("{}", g_var),
            UnresolvedShape::Specified(specified_shape, generic_params) => {
                if generic_params.is_empty() {
                    format!("{}", specified_shape)
                } else {
                    format!(
                        "{}<{}>",
                        specified_shape,
                        generic_params
                            .iter()
                            .map(|shape| self.display_shape_definition(shape).to_string())
                            .collect::<Vec<_>>()
                            .join(", "),
                    )
                }
            }
        }
    }
}

impl GrandShapeContentSummaryAtWork<'_> {
    pub fn display_shape_definition(&self, shape_description_key: &ShapeDescriptionKey) -> String {
        let shape_description = self.get_shape_description(shape_description_key.clone());
        format!("{}", shape_description.annotation);
        match &shape_description.gut {
            ShapeDescriptionGut::Elementary(ele_shape) => {
                format!("{}", ele_shape)
            }
            ShapeDescriptionGut::Unresolved(unresolved_shape) => {
                self.display_unresolved_shape(unresolved_shape)
            }
            ShapeDescriptionGut::ParenSingle(paren_shape) => {
                format!("({})", self.display_shape_definition(paren_shape))
            }
            ShapeDescriptionGut::Tuple(tuple_shapes) => {
                format!(
                    "({})",
                    tuple_shapes
                        .iter()
                        .map(|shape| self.display_shape_definition(shape))
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
            ShapeDescriptionGut::Array(array_shape) => {
                format!("[{}]", self.display_shape_definition(array_shape))
            }
            ShapeDescriptionGut::Map { key, value } => {
                format!(
                    "[{} => {}]",
                    self.display_shape_definition(key),
                    self.display_shape_definition(value)
                )
            }
            ShapeDescriptionGut::Struct(struct_shapes) => {
                format!(
                    "{{\n{}\n}}",
                    struct_shapes
                        .iter()
                        .map(|shape| {
                            format!("{}: {},", shape.0, self.display_shape_definition(shape.1))
                        })
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
            ShapeDescriptionGut::Function { args, ret } => {
                format!(
                    "({}) -> {}",
                    args.iter()
                        .map(|shape| self.display_shape_definition(shape))
                        .collect::<Vec<_>>()
                        .join(", "),
                    self.display_shape_definition(ret)
                )
            }
            ShapeDescriptionGut::Union(shape1, shape2) => {
                format!(
                    "({} | {})",
                    self.display_shape_definition(shape1),
                    self.display_shape_definition(shape2)
                )
            }
            ShapeDescriptionGut::Intersection(shape1, shape2) => {
                format!(
                    "({} & {})",
                    self.display_shape_definition(shape1),
                    self.display_shape_definition(shape2)
                )
            }
        }
    }
}
