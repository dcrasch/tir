use crate::tessellationfigure::{TessellationFigure, TessellationPlane};
use euclid::Angle;
use lyon::algorithms::rounded_polygon;
use lyon::math::{point, Transform};
use lyon::path::{Path, Polygon, NO_ATTRIBUTES};
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
        let mut res: Vec<OutputPrimitive> = Vec::new();
        let mut row = 0;
        let g = plane.grid(figure, 3.0, 3.0);
        let mut c = 0;
        for rotdiv in 1..=figure.rotdiv {
            let angle = Angle::degrees(360.0 * (rotdiv as f32) / (figure.rotdiv as f32));

            for gridrow in &g {
                if !figure.is_reversed {
                    c = row % 2; // use for brick
                }
                for gridpoint in gridrow {
                    if figure.is_reversed {
                        c = rotdiv - 1; // for diamond
                    }
                    if !figure.is_reversed && figure.gridincy < figure.gridincx {
                        c = row % 3; // used for hexagon
                    }

                    let cc = palette[(c % 4) as usize];
                    res.push(OutputPrimitive {
                        x: gridpoint.x,
                        y: gridpoint.y,
                        r: cc.red,
                        g: cc.green,
                        b: cc.blue,
                        angle: angle.radians,
                    });

                    c += 1;
                }
                row += 1;
            }
        }
        res
    }
}

pub struct OutputPrimitive {
    pub x: f32,
    pub y: f32,
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub angle: f32,
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::tessellationfigure::TessellationFigure;

    #[test]
    fn test_square_shader() {
        let f = TessellationFigure::square();
        let m: Transform = Transform::identity();
        let shaders = LyonBackend.build(&f).unwrap();
        assert_eq!(
            format!("{:?}", shaders),
            "\" M 0.0 0.0 L 0.0 1.0 L 1.0 1.0 L 1.0 0.0 Z\""
        );
    }
}
