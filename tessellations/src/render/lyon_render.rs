use crate::tessellationfigure::{TessellationFigure, TessellationPlane};
use lyon::algorithms::rounded_polygon;
use lyon::math::{point, Transform};
use lyon::path::{Path, Polygon, NO_ATTRIBUTES};

#[derive(Clone, Copy)]
pub struct LyonBackend;

pub trait Compose {
    /// Compose a figure to a document
    fn compose_polygon(&self, figure: &TessellationFigure, m: &Transform) -> Option<Path>;
}

impl Compose for LyonBackend {
    fn compose_polygon(&self, figure: &TessellationFigure, m: &Transform) -> Option<Path> {
        let points: Vec<lyon::math::Point> = figure
            .points()
            .windows(2)
            .filter_map(|l| if l[0] != l[1] { Some(l[0]) } else { None })
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
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::tessellationfigure::TessellationFigure;

    #[test]
    fn test_square_shader() {
        let f = TessellationFigure::square();
        let m: Transform =
            Transform::scale(100.0, 100.0).then_translate(euclid::vec2(100.0, 100.0));

        let shaders = LyonBackend.compose_polygon(&f, &m).unwrap();
        println!("{:?}", shaders);
        assert_eq!(format!("{:?}",shaders),"\" M 0.0 0.2 L 0.0 0.8 Q -2.2143507e-8 0.8828428 0.058578625 0.94142133 Q 0.117157176 0.9999999 0.19999997 0.99999994 L 0.8 1.0 Q 0.8828431 0.9999998 0.94142133 0.94142133 Q 0.9999999 0.88284314 0.99999994 0.79999995 L 1.0 0.2 Q 0.9999998 0.11715724 0.94142133 0.05857864 Q 0.8828426 -8.646711e-9 0.79999995 -1.4901161e-8 L 0.2 0.0 Q 0.117157295 1.852233e-8 0.058578655 0.058578655 Q 7.65882e-9 0.11715731 1.4901161e-8 0.20000003 Z\"")
    }
}
