use crate::tessellationfigure::{TessellationFigure, TessellationPlane};
use euclid::Angle;
use raqote::*;

use svg::Document;
use svg::node::element::Path;
use svg::node::element::SVG;
use svg::node::element::path::Data;
use svg::node::element::path::Command;
use svg::node::element::path::Position::Absolute;

#[derive(Clone, Copy)]
pub struct SVGBackend;

pub trait Compose {
    /// Compose a figure to a document
    fn compose(
        &self,
        figure: &TessellationFigure,
        m: &Transform,
    ) -> Option<SVG>;
}

impl Compose for SVGBackend {
    fn compose(
        &self,
        figure: &TessellationFigure,
        m: &Transform,
    ) -> Option<SVG> {
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
	    pb.append(Command::Move(Absolute, (p1.x,p1.y).into()));

        for p in points.iter().skip(1) {    
            pb.append(Command::Line(Absolute, (p.x,p.y).into()));
        }
        pb.append(Command::Close);
        let path = Path::new()
             .set("fill", "none")
             .set("stroke", "black")
             .set("stroke-width", 1)
             .set("d", pb);
        
         let document = Document::new()
            .set("viewBox", (0, 0, 400, 400))
             .add(path);
        svg::save("image.svg", &document).unwrap();
        return Some(document);
    }
}
