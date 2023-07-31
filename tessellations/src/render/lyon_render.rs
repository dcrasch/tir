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
    fn build(&self, figure: &TessellationFigure, m: &Transform) -> Option<Path>;
    fn build_plane(
        &self,
        plane: &TessellationPlane,
        figure: &TessellationFigure,
        m: &Transform,
        colors: &[Srgb],
    ) -> Vec<OutputPrimitive>;
}

impl Builder for LyonBackend {
    fn build(&self, figure: &TessellationFigure, m: &Transform) -> Option<Path> {
        let points: Vec<lyon::math::Point> = figure
            .points()
            .windows(2)
            .filter_map(|l| {
                if l[0] != l[1] {
                    Some(m.transform_point(l[0]))
                } else {
                    None
                }
            })
            .map(|p| point(p.x, p.y))
            .collect();
        let figure_polygon = Polygon {
            points: &points,
            closed: true,
        };

        let mut builder = Path::builder();
        rounded_polygon::add_rounded_polygon(&mut builder, figure_polygon, 0.2, NO_ATTRIBUTES);
        let figure_path = builder.build();
        Some(figure_path)
    }

    fn build_plane(
        &self,
        plane: &TessellationPlane,
        figure: &TessellationFigure,
        m: &Transform,
        palette: &[Srgb],
    ) -> Vec<OutputPrimitive> {
        let mut res: Vec<OutputPrimitive> = Vec::new();
        let mut row = 0;
        let g = plane.grid(figure, 800.0, 800.0, 70.);
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

                    let m = Transform::scale(70.0, 70.0)
                        .then_translate(euclid::vec2(gridpoint.x, gridpoint.y));
                    let p = m.transform_point(point(0.0, 0.0));
                    let cc = palette[(c % 4) as usize];
           
                    res.push(OutputPrimitive {
                        x: p.x - 400.0 + 35.0,
                        y: p.y - 400.0 + 35.0,
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
        let shaders = LyonBackend.build(&f, &m).unwrap();
        assert_eq!(format!("{:?}",shaders),"\" M 0.0 0.2 L 0.0 0.8 Q -2.2143507e-8 0.8828428 0.058578625 0.94142133 Q 0.117157176 0.9999999 0.19999997 0.99999994 L 0.8 1.0 Q 0.8828431 0.9999998 0.94142133 0.94142133 Q 0.9999999 0.88284314 0.99999994 0.79999995 L 1.0 0.2 Q 0.9999998 0.11715724 0.94142133 0.05857864 Q 0.8828426 -8.646711e-9 0.79999995 -1.4901161e-8 L 0.2 0.0 Q 0.117157295 1.852233e-8 0.058578655 0.058578655 Q 7.65882e-9 0.11715731 1.4901161e-8 0.20000003 Z\"")
    }
}
