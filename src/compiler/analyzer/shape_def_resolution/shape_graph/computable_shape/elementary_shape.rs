use crate::compiler::analyzer::shape_def_resolution::description::shape_description::{
    CharWidth, ElementaryShape, FloatWidth, IntWidth, UnresolvedShape,
};
use crate::compiler::parser::parser::ParsedPosition;
use derivative::Derivative;
use crate::compiler::analyzer::shape_def_resolution::shape_graph::computable_shape::computable_shape::{ComputableShape, ComputableShapeGut, ElementaryPShape};
use crate::compiler::analyzer::shape_def_resolution::summary::ComputableShapeKey;

impl From<ElementaryPShape> for ComputableShape {
    fn from(value: ElementaryPShape) -> Self {
        let position = value.position.clone();
        ComputableShape {
            gut: ComputableShapeGut::Elementary(value),
            position,
        }
    }
}

impl ElementaryPShape {
    pub fn pairwise_union(shape1: Self, shape2: Self) -> Option<ElementaryPShape> {
        match (&shape1.gut, &shape2.gut) {
            (ElementaryShape::Empty, _) => Some(shape2),
            (_, ElementaryShape::Empty) => Some(shape1),
            (ElementaryShape::Any, _) => Some(shape1),
            (_, ElementaryShape::Any) => Some(shape2),

            (ElementaryShape::Unit, ElementaryShape::Unit) => Some(shape1),
            (ElementaryShape::Bool, ElementaryShape::Bool) => Some(shape1),

            (ElementaryShape::Int(IntWidth::SignedBit256), ElementaryShape::Int(_))
            | (
                ElementaryShape::Int(IntWidth::SignedBit128),
                ElementaryShape::Int(IntWidth::UnsignedBit128),
            ) => ElementaryPShape {
                gut: ElementaryShape::Int(IntWidth::SignedBit256),
                position: shape1.position.clone(),
            }
            .into(),

            (ElementaryShape::Int(_), ElementaryShape::Int(IntWidth::SignedBit256))
            | (
                ElementaryShape::Int(IntWidth::UnsignedBit128),
                ElementaryShape::Int(IntWidth::SignedBit128),
            ) => ElementaryPShape {
                gut: ElementaryShape::Int(IntWidth::SignedBit256),
                position: shape2.position.clone(),
            }
            .into(),

            (ElementaryShape::Int(IntWidth::SignedBit128), ElementaryShape::Int(_))
            | (
                ElementaryShape::Int(IntWidth::SignedBit64),
                ElementaryShape::Int(IntWidth::UnsignedBit64),
            ) => ElementaryPShape {
                gut: ElementaryShape::Int(IntWidth::SignedBit128),
                position: shape1.position.clone(),
            }
            .into(),

            (ElementaryShape::Int(_), ElementaryShape::Int(IntWidth::SignedBit128))
            | (
                ElementaryShape::Int(IntWidth::UnsignedBit64),
                ElementaryShape::Int(IntWidth::SignedBit64),
            ) => ElementaryPShape {
                gut: ElementaryShape::Int(IntWidth::SignedBit128),
                position: shape2.position.clone(),
            }
            .into(),

            (ElementaryShape::Int(IntWidth::SignedBit64), ElementaryShape::Int(_))
            | (
                ElementaryShape::Int(IntWidth::SignedBit32),
                ElementaryShape::Int(IntWidth::UnsignedBit32),
            ) => ElementaryPShape {
                gut: ElementaryShape::Int(IntWidth::SignedBit64),
                position: shape1.position.clone(),
            }
            .into(),

            (ElementaryShape::Int(_), ElementaryShape::Int(IntWidth::SignedBit64))
            | (
                ElementaryShape::Int(IntWidth::UnsignedBit32),
                ElementaryShape::Int(IntWidth::SignedBit32),
            ) => ElementaryPShape {
                gut: ElementaryShape::Int(IntWidth::SignedBit64),
                position: shape2.position.clone(),
            }
            .into(),

            (ElementaryShape::Int(IntWidth::SignedBit32), ElementaryShape::Int(_))
            | (
                ElementaryShape::Int(IntWidth::SignedBit16),
                ElementaryShape::Int(IntWidth::UnsignedBit16),
            ) => ElementaryPShape {
                gut: ElementaryShape::Int(IntWidth::SignedBit32),
                position: shape1.position.clone(),
            }
            .into(),

            (ElementaryShape::Int(_), ElementaryShape::Int(IntWidth::SignedBit32))
            | (
                ElementaryShape::Int(IntWidth::UnsignedBit16),
                ElementaryShape::Int(IntWidth::SignedBit16),
            ) => ElementaryPShape {
                gut: ElementaryShape::Int(IntWidth::SignedBit32),
                position: shape2.position.clone(),
            }
            .into(),

            (ElementaryShape::Int(IntWidth::SignedBit16), ElementaryShape::Int(_)) => Some(shape1),
            (ElementaryShape::Int(_), ElementaryShape::Int(IntWidth::SignedBit16)) => Some(shape2),

            (ElementaryShape::Int(IntWidth::UnsignedBit64), ElementaryShape::Int(_)) => {
                Some(shape1)
            }
            (ElementaryShape::Int(_), ElementaryShape::Int(IntWidth::UnsignedBit64)) => {
                Some(shape2)
            }

            (ElementaryShape::Int(IntWidth::UnsignedBit32), ElementaryShape::Int(_)) => {
                Some(shape1)
            }
            (ElementaryShape::Int(_), ElementaryShape::Int(IntWidth::UnsignedBit32)) => {
                Some(shape2)
            }

            (ElementaryShape::Int(IntWidth::UnsignedBit16), ElementaryShape::Int(_)) => {
                Some(shape1)
            }
            (ElementaryShape::Int(_), ElementaryShape::Int(IntWidth::UnsignedBit16)) => {
                Some(shape2)
            }

            (
                ElementaryShape::Int(IntWidth::UnsignedBit8),
                ElementaryShape::Int(IntWidth::UnsignedBit8),
            ) => Some(shape1),

            (ElementaryShape::Float(FloatWidth::Bit128), ElementaryShape::Float(_)) => Some(shape1),
            (ElementaryShape::Float(_), ElementaryShape::Float(FloatWidth::Bit128)) => Some(shape2),

            (ElementaryShape::Float(FloatWidth::Bit64), ElementaryShape::Float(_)) => Some(shape1),
            (ElementaryShape::Float(_), ElementaryShape::Float(FloatWidth::Bit64)) => Some(shape2),

            (ElementaryShape::Float(FloatWidth::Bit32), ElementaryShape::Float(_)) => Some(shape1),
            (ElementaryShape::Float(_), ElementaryShape::Float(FloatWidth::Bit32)) => Some(shape2),

            (
                ElementaryShape::Float(FloatWidth::Bit16),
                ElementaryShape::Float(FloatWidth::Bit16),
            ) => Some(shape1),

            (ElementaryShape::Char(CharWidth::UTF16), ElementaryShape::Char(_)) => Some(shape1),
            (ElementaryShape::Char(_), ElementaryShape::Char(CharWidth::UTF16)) => Some(shape2),

            (ElementaryShape::Char(CharWidth::Ascii), ElementaryShape::Char(CharWidth::Ascii)) => {
                Some(shape1)
            }

            (ElementaryShape::String(CharWidth::UTF16), ElementaryShape::String(_)) => Some(shape1),
            (ElementaryShape::String(_), ElementaryShape::String(CharWidth::UTF16)) => Some(shape2),

            (
                ElementaryShape::String(CharWidth::Ascii),
                ElementaryShape::String(CharWidth::Ascii),
            ) => Some(shape1),

            (_, _) => None,
        }
    }

    pub fn pairwise_intersection(shape1: Self, shape2: Self) -> Self {
        match (shape1.gut, shape1.gut) {
            (ElementaryShape::Empty, _) => shape1,
            (_, ElementaryShape::Empty) => shape2,

            (_, ElementaryShape::Any) => shape2,
            (ElementaryShape::Any, _) => shape1,

            (ElementaryShape::Unit, ElementaryShape::Unit) => shape1,
            (ElementaryShape::Bool, ElementaryShape::Bool) => shape1,

            (ElementaryShape::Int(IntWidth::UnsignedBit8), ElementaryShape::Int(_))
            | (
                ElementaryShape::Int(IntWidth::UnsignedBit16),
                ElementaryShape::Int(IntWidth::SignedBit16),
            ) => ElementaryPShape {
                gut: ElementaryShape::Int(IntWidth::UnsignedBit8),
                position: shape1.position,
            },

            (ElementaryShape::Int(_), ElementaryShape::Int(IntWidth::UnsignedBit8))
            | (
                ElementaryShape::Int(IntWidth::SignedBit16),
                ElementaryShape::Int(IntWidth::UnsignedBit16),
            ) => ElementaryPShape {
                gut: ElementaryShape::Int(IntWidth::UnsignedBit8),
                position: shape2.position,
            },

            (ElementaryShape::Int(IntWidth::UnsignedBit16), ElementaryShape::Int(_))
            | (
                ElementaryShape::Int(IntWidth::UnsignedBit32),
                ElementaryShape::Int(IntWidth::SignedBit32),
            ) => ElementaryPShape {
                gut: ElementaryShape::Int(IntWidth::UnsignedBit16),
                position: shape1.position,
            },

            (ElementaryShape::Int(_), ElementaryShape::Int(IntWidth::UnsignedBit16))
            | (
                ElementaryShape::Int(IntWidth::SignedBit32),
                ElementaryShape::Int(IntWidth::UnsignedBit32),
            ) => ElementaryPShape {
                gut: ElementaryShape::Int(IntWidth::UnsignedBit16),
                position: shape2.position,
            },

            (ElementaryShape::Int(IntWidth::UnsignedBit32), ElementaryShape::Int(_))
            | (
                ElementaryShape::Int(IntWidth::UnsignedBit64),
                ElementaryShape::Int(IntWidth::SignedBit64),
            ) => ElementaryPShape {
                gut: ElementaryShape::Int(IntWidth::UnsignedBit32),
                position: shape1.position,
            },

            (ElementaryShape::Int(_), ElementaryShape::Int(IntWidth::UnsignedBit32))
            | (
                ElementaryShape::Int(IntWidth::SignedBit64),
                ElementaryShape::Int(IntWidth::UnsignedBit64),
            ) => ElementaryPShape {
                gut: ElementaryShape::Int(IntWidth::UnsignedBit32),
                position: shape2.position,
            },

            (ElementaryShape::Int(IntWidth::UnsignedBit64), ElementaryShape::Int(_))
            | (
                ElementaryShape::Int(IntWidth::UnsignedBit128),
                ElementaryShape::Int(IntWidth::SignedBit128),
            ) => ElementaryPShape {
                gut: ElementaryShape::Int(IntWidth::UnsignedBit64),
                position: shape1.position,
            },

            (ElementaryShape::Int(_), ElementaryShape::Int(IntWidth::UnsignedBit64))
            | (
                ElementaryShape::Int(IntWidth::SignedBit128),
                ElementaryShape::Int(IntWidth::UnsignedBit128),
            ) => ElementaryPShape {
                gut: ElementaryShape::Int(IntWidth::UnsignedBit64),
                position: shape2.position,
            },

            (ElementaryShape::Int(IntWidth::UnsignedBit128), ElementaryShape::Int(_)) => shape1,
            (ElementaryShape::Int(_), ElementaryShape::Int(IntWidth::UnsignedBit128)) => shape2,

            (ElementaryShape::Int(IntWidth::SignedBit16), ElementaryShape::Int(_)) => shape1,
            (ElementaryShape::Int(_), ElementaryShape::Int(IntWidth::SignedBit16)) => shape2,

            (ElementaryShape::Int(IntWidth::SignedBit32), ElementaryShape::Int(_)) => shape1,
            (ElementaryShape::Int(_), ElementaryShape::Int(IntWidth::SignedBit32)) => shape2,

            (ElementaryShape::Int(IntWidth::SignedBit64), ElementaryShape::Int(_)) => shape1,
            (ElementaryShape::Int(_), ElementaryShape::Int(IntWidth::SignedBit64)) => shape2,

            (ElementaryShape::Int(IntWidth::SignedBit128), ElementaryShape::Int(_)) => shape1,
            (ElementaryShape::Int(_), ElementaryShape::Int(IntWidth::SignedBit128)) => shape2,

            (
                ElementaryShape::Int(IntWidth::SignedBit256),
                ElementaryShape::Int(IntWidth::SignedBit256),
            ) => shape1,

            (ElementaryShape::Float(FloatWidth::Bit16), ElementaryShape::Float(_)) => shape1,
            (ElementaryShape::Float(_), ElementaryShape::Float(FloatWidth::Bit16)) => shape2,

            (ElementaryShape::Float(FloatWidth::Bit32), ElementaryShape::Float(_)) => shape1,
            (ElementaryShape::Float(_), ElementaryShape::Float(FloatWidth::Bit32)) => shape2,

            (ElementaryShape::Float(FloatWidth::Bit64), ElementaryShape::Float(_)) => shape1,
            (ElementaryShape::Float(_), ElementaryShape::Float(FloatWidth::Bit64)) => shape2,

            (
                ElementaryShape::Float(FloatWidth::Bit128),
                ElementaryShape::Float(FloatWidth::Bit128),
            ) => shape1,

            (ElementaryShape::Char(CharWidth::Ascii), ElementaryShape::Char(_)) => shape1,
            (ElementaryShape::Char(_), ElementaryShape::Char(CharWidth::Ascii)) => shape2,

            (ElementaryShape::Char(CharWidth::UTF16), ElementaryShape::Char(CharWidth::UTF16)) => {
                shape1
            }

            (ElementaryShape::String(CharWidth::Ascii), ElementaryShape::String(_)) => shape1,
            (ElementaryShape::String(_), ElementaryShape::String(CharWidth::Ascii)) => shape2,

            (
                ElementaryShape::String(CharWidth::UTF16),
                ElementaryShape::String(CharWidth::UTF16),
            ) => shape1,

            (_, _) => ElementaryPShape {
                gut: ElementaryShape::Empty,
                position: shape1.position,
            },
        }
    }

    pub fn pairwise_is_subset(oprd1: &Self, oprd2: &Self) -> bool {
        match (oprd1.gut, oprd2.gut) {
            (ElementaryShape::Empty, _) => true,
            (_, ElementaryShape::Empty) => false,
            (_, ElementaryShape::Any) => true,
            (ElementaryShape::Any, _) => false,

            (ElementaryShape::Unit, ElementaryShape::Unit) => true,
            (ElementaryShape::Bool, ElementaryShape::Bool) => true,

            (
                ElementaryShape::Int(IntWidth::UnsignedBit128),
                ElementaryShape::Int(IntWidth::UnsignedBit128),
            )
            | (
                ElementaryShape::Int(IntWidth::UnsignedBit64),
                ElementaryShape::Int(IntWidth::UnsignedBit128),
            )
            | (
                ElementaryShape::Int(IntWidth::UnsignedBit32),
                ElementaryShape::Int(IntWidth::UnsignedBit128),
            )
            | (
                ElementaryShape::Int(IntWidth::UnsignedBit16),
                ElementaryShape::Int(IntWidth::UnsignedBit128),
            )
            | (
                ElementaryShape::Int(IntWidth::UnsignedBit8),
                ElementaryShape::Int(IntWidth::UnsignedBit128),
            )
            | (
                ElementaryShape::Int(IntWidth::UnsignedBit64),
                ElementaryShape::Int(IntWidth::UnsignedBit64),
            )
            | (
                ElementaryShape::Int(IntWidth::UnsignedBit32),
                ElementaryShape::Int(IntWidth::UnsignedBit64),
            )
            | (
                ElementaryShape::Int(IntWidth::UnsignedBit16),
                ElementaryShape::Int(IntWidth::UnsignedBit64),
            )
            | (
                ElementaryShape::Int(IntWidth::UnsignedBit8),
                ElementaryShape::Int(IntWidth::UnsignedBit64),
            )
            | (
                ElementaryShape::Int(IntWidth::UnsignedBit32),
                ElementaryShape::Int(IntWidth::UnsignedBit32),
            )
            | (
                ElementaryShape::Int(IntWidth::UnsignedBit16),
                ElementaryShape::Int(IntWidth::UnsignedBit32),
            )
            | (
                ElementaryShape::Int(IntWidth::UnsignedBit8),
                ElementaryShape::Int(IntWidth::UnsignedBit32),
            )
            | (
                ElementaryShape::Int(IntWidth::UnsignedBit16),
                ElementaryShape::Int(IntWidth::UnsignedBit16),
            )
            | (
                ElementaryShape::Int(IntWidth::UnsignedBit8),
                ElementaryShape::Int(IntWidth::UnsignedBit16),
            )
            | (
                ElementaryShape::Int(IntWidth::UnsignedBit8),
                ElementaryShape::Int(IntWidth::UnsignedBit8),
            ) => true,

            (ElementaryShape::Int(_), ElementaryShape::Int(IntWidth::UnsignedBit128))
            | (ElementaryShape::Int(_), ElementaryShape::Int(IntWidth::UnsignedBit64))
            | (ElementaryShape::Int(_), ElementaryShape::Int(IntWidth::UnsignedBit32))
            | (ElementaryShape::Int(_), ElementaryShape::Int(IntWidth::UnsignedBit16))
            | (ElementaryShape::Int(_), ElementaryShape::Int(IntWidth::UnsignedBit8)) => false,

            (ElementaryShape::Int(_), ElementaryShape::Int(IntWidth::SignedBit256)) => true,
            (ElementaryShape::Int(IntWidth::SignedBit256), ElementaryShape::Int(_)) => false,
            (ElementaryShape::Int(IntWidth::UnsignedBit128), ElementaryShape::Int(_)) => false,

            (ElementaryShape::Int(_), ElementaryShape::Int(IntWidth::SignedBit128)) => true,
            (ElementaryShape::Int(IntWidth::SignedBit128), ElementaryShape::Int(_)) => false,
            (ElementaryShape::Int(IntWidth::UnsignedBit64), ElementaryShape::Int(_)) => false,

            (ElementaryShape::Int(_), ElementaryShape::Int(IntWidth::SignedBit64)) => true,
            (ElementaryShape::Int(IntWidth::SignedBit64), ElementaryShape::Int(_)) => false,
            (ElementaryShape::Int(IntWidth::UnsignedBit32), ElementaryShape::Int(_)) => false,

            (ElementaryShape::Int(_), ElementaryShape::Int(IntWidth::SignedBit32)) => true,
            (ElementaryShape::Int(IntWidth::SignedBit32), ElementaryShape::Int(_)) => false,
            (ElementaryShape::Int(IntWidth::UnsignedBit16), ElementaryShape::Int(_)) => false,

            (ElementaryShape::Int(_), ElementaryShape::Int(IntWidth::SignedBit16)) => true,

            (ElementaryShape::Float(_), ElementaryShape::Float(FloatWidth::Bit128)) => true,
            (ElementaryShape::Float(FloatWidth::Bit128), ElementaryShape::Float(_)) => false,

            (ElementaryShape::Float(_), ElementaryShape::Float(FloatWidth::Bit64)) => true,
            (ElementaryShape::Float(FloatWidth::Bit64), ElementaryShape::Float(_)) => false,

            (ElementaryShape::Float(_), ElementaryShape::Float(FloatWidth::Bit32)) => true,
            (ElementaryShape::Float(FloatWidth::Bit32), ElementaryShape::Float(_)) => false,

            (
                ElementaryShape::Float(FloatWidth::Bit16),
                ElementaryShape::Float(FloatWidth::Bit16),
            ) => true,

            (ElementaryShape::Char(_), ElementaryShape::Char(CharWidth::UTF16)) => true,
            (ElementaryShape::Char(CharWidth::UTF16), ElementaryShape::Char(_)) => false,

            (ElementaryShape::Char(CharWidth::Ascii), ElementaryShape::Char(CharWidth::Ascii)) => {
                true
            }

            (ElementaryShape::String(_), ElementaryShape::String(CharWidth::UTF16)) => true,
            (ElementaryShape::String(CharWidth::UTF16), ElementaryShape::String(_)) => false,

            (
                ElementaryShape::String(CharWidth::Ascii),
                ElementaryShape::String(CharWidth::Ascii),
            ) => true,

            (_, _) => false,
        }
    }
}

impl ElementaryPShape {
    pub fn is_any(&self) -> bool {
        matches!(
            self,
            ElementaryPShape {
                gut: ElementaryShape::Any,
                ..
            }
        )
    }

    pub fn is_empty(&self) -> bool {
        matches!(
            self,
            ElementaryPShape {
                gut: ElementaryShape::Empty,
                ..
            }
        )
    }
}
