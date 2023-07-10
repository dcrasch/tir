use crate::tessellationfigure::{TessellationFigure, TessellationPlane};
use euclid::Angle;
use raqote::*;

use svg::node::element::path::Command;
use svg::node::element::path::Data;
use svg::node::element::path::Position::Absolute;
use svg::node::element::Path;
use svg::node::element::SVG;
use svg::Document;
use svg::Node;

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
        let points = figure
            .points()
            .windows(2)
            .filter_map(|l| {
                if l[0] != l[1] {
                    Some(m.transform_point(l[0]))
                } else {
                    None
                }
            })
            .collect::<Vec<Point>>();
        let p1 = points[0];

        let mut pb = Data::new();
        pb.append(Command::Move(Absolute, (p1.x, p1.y).into()));

        for p in points.iter().skip(1) {
            pb.append(Command::Line(Absolute, (p.x, p.y).into()));
        }
        pb.append(Command::Close);
        let path = Path::new()
            .set("fill", "none")
            .set("stroke", "black")
            .set("stroke-width", 1)
            .set("d", pb);

        let document = Document::new().set("viewBox", (0, 0, 400, 400)).add(path);
        return Some(document);
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

                    let m = Transform::rotation(angle)
                        .then_scale(70.0, 70.0)
                        .then_translate(euclid::vec2(gridpoint.x, gridpoint.y));
                    let points = figure
                        .points()
                        .windows(2)
                        .filter_map(|l| {
                            if l[0] != l[1] {
                                Some(m.transform_point(l[0]))
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<Point>>();
                    let p1 = points[0];

                    let mut pb = Data::new();
                    pb.append(Command::Move(Absolute, (p1.x, p1.y).into()));

                    for p in points.iter().skip(1) {
                        pb.append(Command::Line(Absolute, (p.x, p.y).into()));
                    }
                    pb.append(Command::Close);
                    let color = colors[(c % 4) as usize];
                    let path = Path::new()
                        .set("fill", color)
                        .set("stroke", "black")
                        .set("stroke-width", 1)
                        .set("d", pb);

                    document.append(path);

                    c += 1;
                }
                row += 1;
            }
        }
	let points = figure
            .points()
            .windows(2)
            .filter_map(|l| {
                if l[0] != l[1] {
                    Some(m.transform_point(l[0]))
                } else {
                    None
                }
            })
            .collect::<Vec<Point>>();

	let p1 = points[0];
	
	let mut pb = Data::new();
        pb.append(Command::Move(Absolute, (p1.x, p1.y).into()));
	
        for p in points.iter().skip(1) {
            pb.append(Command::Line(Absolute, (p.x, p.y).into()));
        }
        pb.append(Command::Close);
        let path = Path::new()
            .set("fill", "none")
            .set("stroke", "yellow")
            .set("stroke-width", 3)
            .set("d", pb);
	
        document.append(path);
	
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
	svg::write(&mut data, self);
	String::from_utf8(data).unwrap()
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
}
