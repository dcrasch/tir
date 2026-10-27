use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub enum TessellationShape {
    #[default]
    S,
    U,
    I,
    J,
}
