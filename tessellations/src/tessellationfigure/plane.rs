use crate::tessellationfigure::{Placement, TessellationFigure};

pub struct TessellationPlane {}

impl TessellationPlane {
    /// Copies of the tile that cover a `width` x `height` area centred on the origin.
    pub fn placements(
        &self,
        figure: &TessellationFigure,
        width: f32,
        height: f32,
    ) -> Vec<Placement> {
        figure.placements(width, height)
    }
}
