use crate::tessellationfigure::{TessellationFigure, TessellationPlane};
use euclid::Angle;
use raqote::*;

use svg::node::element::path::{Command, Data, Position::Absolute};
use svg::node::element::{Definitions, Path, Use, SVG};
use svg::{Document, Node};

#[derive(Clone, Copy)]
pub struct SVGBackend;

pub trait Compose {
    /// Compose a figure to a document
    fn compose(&self, figure: &TessellationFigure, m: &Transform) -> Option<SVG>;

    fn compose_plane(
        &self,
        plane: &TessellationPlane,
        figure: &TessellationFigure,
        m: &Transform,
    ) -> Option<SVG>;
}

impl Compose for SVGBackend {
    fn compose(&self, figure: &TessellationFigure, m: &Transform) -> Option<SVG> {
        let mut document: SVG = Document::new().set("viewBox", (0, 0, 400, 400));

        let points = figure
            .points()
            .windows(2)
            .filter_map(|l| if l[0] != l[1] { Some(l[0]) } else { None })
            .collect::<Vec<Point>>();
        let p1 = points[0];

        let mut pb = Data::new();
        pb.append(Command::Move(Absolute, (p1.x, p1.y).into()));

        for p in points.iter().skip(1) {
            pb.append(Command::Line(Absolute, (p.x, p.y).into()));
        }
        pb.append(Command::Close);
        let path = Path::new()
            .set("vector-effect", "non-scaling-stroke")
            .set("d", pb)
            .set("id", "figure");

        let defs = Definitions::new().add(path);
        document.append(defs);

        let main_figure = Use::new()
            .set(
                "transform",
                format!(
                    "matrix({},{},{},{},{},{})",
                    m.m11, m.m12, m.m21, m.m22, m.m31, m.m32
                ),
            )
            .set("stroke", "yellow")
            .set("stroke-width", "3px")
            .set("fill", "none")
            .set("href", "#figure");
        document.append(main_figure);

        Some(document)
    }

    fn compose_plane(
        &self,
        plane: &TessellationPlane,
        figure: &TessellationFigure,
        m: &Transform,
    ) -> Option<SVG> {
        let mut row = 0;
        let g = plane.grid(figure, 400.0, 400.0, 70.);
        let mut c = 0;
        let mut document = Document::new().set("viewBox", (0, 0, 400, 400));
        let colors = vec!["red", "green", "blue", "black"];

        let points = figure
            .points()
            .windows(2)
            .filter_map(|l| if l[0] != l[1] { Some(l[0]) } else { None })
            .collect::<Vec<Point>>();
        let p1 = points[0];

        let mut pb = Data::new();
        pb.append(Command::Move(Absolute, (p1.x, p1.y).into()));
        for p in points.iter().skip(1) {
            pb.append(Command::Line(Absolute, (p.x, p.y).into()));
        }
        pb.append(Command::Close);

        let path = Path::new()
            .set("vector-effect", "non-scaling-stroke")
            .set("d", pb)
            .set("id", "figure");

        let defs = Definitions::new().add(path);
        document.append(defs);

        for rotdiv in 0..figure.rotdiv {
            let angle = Angle::degrees(360.0 * (rotdiv as f32) / (figure.rotdiv as f32));

            for gridrow in &g {
                if !figure.is_reversed {
                    c = row % 2; // use for brick
                }
                for gridpoint in gridrow {
                    if figure.is_reversed {
                        c = rotdiv; // for diamond
                    }
                    if !figure.is_reversed && figure.gridincy < figure.gridincx {
                        c = row % 3; // used for hexagon
                    }
                    let m = Transform::rotation(angle)
                        .then_scale(70.0, 70.0)
                        .then_translate(euclid::vec2(gridpoint.x, gridpoint.y));

                    let tile_figure = Use::new()
                        .set("href", "#figure")
                        .set("fill", colors[(c % 4) as usize])
                        .set(
                            "transform",
                            format!(
                                "matrix({},{},{},{},{},{})",
                                m.m11, m.m12, m.m21, m.m22, m.m31, m.m32
                            ),
                        );
                    document.append(tile_figure);

                    c += 1;
                }
                row += 1;
            }
        }
        let edit_figure = Use::new()
            .set(
                "transform",
                format!(
                    "matrix({},{},{},{},{},{})",
                    m.m11, m.m12, m.m21, m.m22, m.m31, m.m32
                ),
            )
            .set("stroke", "yellow")
            .set("stroke-width", "3px")
            .set("fill", "none")
            .set("href", "#figure");
        document.append(edit_figure);

        Some(document)
    }
}

pub trait SVGImage {
    fn save_svg(&self, path: &std::path::Path) -> bool;
    fn get_data(&self) -> String;
}

impl SVGImage for Document {
    fn save_svg(&self, path: &::std::path::Path) -> bool {
        svg::save(path, self).is_ok()
    }

    fn get_data(&self) -> String {
        let mut data = vec![];
        svg::write(&mut data, self).unwrap();
        String::from_utf8(data).unwrap()
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::tessellationfigure::TessellationFigure;

    #[test]
    fn test_square_svg() {
        let f = TessellationFigure::square();
        let m: Transform =
            Transform::scale(100.0, 100.0).then_translate(euclid::vec2(100.0, 100.0));

        let svgbackend = Box::new(SVGBackend);
        let svg_document = svgbackend.compose(&f, &m).unwrap();
        let expected_svg = "<svg viewBox=\"0 0 400 400\" xmlns=\"http://www.w3.org/2000/svg\">\n<defs>\n<path d=\"M0,0 L0,1 L1,1 L1,0 z\" id=\"figure\" vector-effect=\"non-scaling-stroke\"/>\n</defs>\n<use fill=\"none\" href=\"#figure\" stroke=\"yellow\" stroke-width=\"3px\" transform=\"matrix(100,0,0,100,100,100)\"/>\n</svg>";
        assert_eq!(svg_document.get_data(), expected_svg);
    }
}
