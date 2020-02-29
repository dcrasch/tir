use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum TessellationShape {
    S,
    U,
    I,
    J,
}

impl Default for TessellationShape {
    fn default() -> Self {
        TessellationShape::S
    }
}
