#[derive(Debug)]
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
