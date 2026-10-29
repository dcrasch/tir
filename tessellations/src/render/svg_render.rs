use crate::render::palette::{Palette, CLASSIC};
use crate::tessellationfigure::TessellationFigure;
use crate::tessellationline::{Point, Transform};

use svg::node::element::path::{Command, Data, Position::Absolute};
use svg::node::element::{Definitions, Path, Use, SVG};
use svg::{Document, Node};

#[derive(Clone, Copy)]
pub struct SVGBackend;

pub trait Compose {
    /// Compose a figure to a document
    fn compose(&self, figure: &TessellationFigure, m: &Transform) -> Option<SVG>;

    fn compose_tiling(&self, figure: &TessellationFigure, m: &Transform) -> Option<SVG>;
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

    fn compose_tiling(&self, figure: &TessellationFigure, m: &Transform) -> Option<SVG> {
        self.compose_tiling_with_palette(figure, m, &CLASSIC)
    }
}

impl SVGBackend {
    /// Compose a tiling whose tiles are coloured from `palette`.
    pub fn compose_tiling_with_palette(
        &self,
        figure: &TessellationFigure,
        m: &Transform,
        palette: &Palette,
    ) -> Option<SVG> {
        let colors = palette.css();
        if colors.is_empty() {
            return None;
        }
        let mut document = Document::new().set("viewBox", (0, 0, 400, 400));

        let points = figure
            .points()
            .windows(2)
            .filter_map(|l| if l[0] != l[1] { Some(l[0]) } else { None })
            .collect::<Vec<Point>>();
        let p1 = *points.first()?;

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

        let ncolors = figure.colors.clamp(1, colors.len());
        for placement in figure.placements(12.0, 12.0) {
            let t = placement.transform.then(m);
            let tile_figure = Use::new()
                .set("href", "#figure")
                .set("fill", colors[placement.color % ncolors].as_str())
                .set(
                    "transform",
                    format!(
                        "matrix({},{},{},{},{},{})",
                        t.m11, t.m12, t.m21, t.m22, t.m31, t.m32
                    ),
                );
            document.append(tile_figure);
        }

        // The edited tile is outlined twice: a light halo below, so the outline
        // stays visible on dark tiles, and a dark line above, so it stays
        // visible on light ones. A single colour would disappear into some
        // palette entries.
        let transform = format!(
            "matrix({},{},{},{},{},{})",
            m.m11, m.m12, m.m21, m.m22, m.m31, m.m32
        );
        for (stroke, width) in [("#ffffff", "6px"), ("#000000", "2px")] {
            let edit_figure = Use::new()
                .set("transform", transform.as_str())
                .set("stroke", stroke)
                .set("stroke-width", width)
                .set("fill", "none")
                .set("href", "#figure");
            document.append(edit_figure);
        }

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
        let expected_svg = "<svg viewBox=\"0 0 400 400\" xmlns=\"http://www.w3.org/2000/svg\">\n<defs>\n<path d=\"M0,0 L1,0 L1,1 L0,1 z\" id=\"figure\" vector-effect=\"non-scaling-stroke\"/>\n</defs>\n<use fill=\"none\" href=\"#figure\" stroke=\"yellow\" stroke-width=\"3px\" transform=\"matrix(100,0,0,100,100,100)\"/>\n</svg>";
        assert_eq!(svg_document.get_data(), expected_svg);
    }

    #[test]
    fn test_tiling_uses_the_palette() {
        use crate::render::palette::ALHAMBRA;

        let f = TessellationFigure::square();
        let m: Transform =
            Transform::scale(100.0, 100.0).then_translate(euclid::vec2(100.0, 100.0));

        let svg_document = SVGBackend
            .compose_tiling_with_palette(&f, &m, &ALHAMBRA)
            .unwrap()
            .get_data();
        assert!(svg_document.contains("fill=\"#c96c9b\""), "{svg_document}");
        assert!(svg_document.contains("fill=\"#568783\""), "{svg_document}");
        assert!(!svg_document.contains("fill=\"red\""), "{svg_document}");
    }
}
