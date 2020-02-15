use crate::tessellationfigure::TessellationFigure;
use raqote::*;

#[derive(Clone, Copy)]
pub struct Backend;

pub trait Render {
    fn render_to_image(&self, figure: TessellationFigure) -> Option<Box<dyn OutputImage>>;
}

impl Render for Backend {
    fn render_to_image(&self, figure: TessellationFigure) -> Option<Box<dyn OutputImage>> {
        let mut dt = DrawTarget::new(400, 400);

        // white background
        let mut pb = PathBuilder::new();
        pb.rect(0., 0., 400., 400.);
        dt.fill(
            &pb.finish(),
            &Source::Solid(SolidSource {
                r: 0xff,
                g: 0xff,
                b: 0xff,
                a: 0xff,
            }),
            &DrawOptions::new(),
        );

        let m = Transform::create_scale(100.0, 100.0).post_translate(euclid::vec2(0.0, 0.0));
        let mut pb = PathBuilder::new();
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
        let p1 = points[0].clone();
        pb.move_to(p1.x, p1.y);
        for p in points.iter().skip(1) {
            pb.line_to(p.x, p.y);
        }
        pb.close();
        let path = pb.finish();

        dt.stroke(
            &path,
            &Source::Solid(SolidSource {
                r: 0x0,
                g: 0x0,
                b: 0x0,
                a: 0xff,
            }),
            &StrokeStyle::default(),
            &DrawOptions::new(),
        );

        Some(Box::new(dt))
    }
}

pub trait OutputImage {
    /// Saves rendered image to the selected path.
    fn save_png(&self, path: &std::path::Path) -> bool;
}

impl OutputImage for raqote::DrawTarget {
    fn save_png(&self, path: &::std::path::Path) -> bool {
        self.write_png(path).is_ok()
    }
}
