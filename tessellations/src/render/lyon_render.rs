use crate::tessellationfigure::{TessellationFigure, TessellationPlane};
use lyon::math::point;
use lyon::path::{Path, Polygon};
use palette::Srgb;

#[derive(Clone, Copy)]
pub struct LyonBackend;

pub trait Builder {
    /// Compose a figure to a document
    fn build(&self, figure: &TessellationFigure) -> Option<Path>;
    fn build_plane(
        &self,
        plane: &TessellationPlane,
        figure: &TessellationFigure,
        colors: &[Srgb],
    ) -> Vec<OutputPrimitive>;
}

impl Builder for LyonBackend {
    fn build(&self, figure: &TessellationFigure) -> Option<Path> {
        let points: Vec<lyon::math::Point> = figure
            .points()
            .windows(2)
            .filter_map(|l| if l[0] != l[1] { Some(l[0]) } else { None })
            .map(|p| point(p.x, p.y))
            .collect();
        let mut builder = Path::builder();
        builder.add_polygon(Polygon {
            points: &points,
            closed: true,
        });
        //rounded_polygon::add_rounded_polygon(&mut builder, figure_polygon, 0.002, NO_ATTRIBUTES);
        let figure_path = builder.build();
        Some(figure_path)
    }

    fn build_plane(
        &self,
        plane: &TessellationPlane,
        figure: &TessellationFigure,
        palette: &[Srgb],
    ) -> Vec<OutputPrimitive> {
        let ncolors = figure.colors.clamp(1, palette.len());
        plane
            .placements(figure, 16.0, 16.0)
            .into_iter()
            .map(|p| {
                let cc = palette[p.color % ncolors];
                let t = p.transform;
                OutputPrimitive {
                    x: t.m31 - 4.0,
                    y: t.m32,
                    r: cc.red,
                    g: cc.green,
                    b: cc.blue,
                    matrix: [t.m11, t.m12, t.m21, t.m22],
                }
            })
            .collect()
    }
}

pub struct OutputPrimitive {
    pub x: f32,
    pub y: f32,
    pub r: f32,
    pub g: f32,
    pub b: f32,
    /// Linear part of the placement (rotation, reflection and scale).
    pub matrix: [f32; 4],
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::tessellationfigure::TessellationFigure;

    #[test]
    fn test_square_shader() {
        let f = TessellationFigure::square();
        let shaders = LyonBackend.build(&f).unwrap();
        assert_eq!(
            format!("{:?}", shaders),
            "\" M 0.0 0.0 L 1.0 0.0 L 1.0 1.0 L 0.0 1.0 Z\""
        );
    }
}
