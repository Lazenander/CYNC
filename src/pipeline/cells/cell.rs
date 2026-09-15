use std::fmt::{write, Display, Formatter};

#[derive(Debug, Clone, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub struct CellSignature {
    pub name: String,
    pub version: (u32, u32, u32),
}

impl Display for CellSignature {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}@{}.{}.{}",
            self.name, self.version.0, self.version.1, self.version.2
        )
    }
}
