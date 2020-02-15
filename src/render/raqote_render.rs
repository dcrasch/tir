use crate::tessellationfigure::TessellationFigure;
use nalgebra::{Matrix3, Vector2};

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

        let m = Matrix3::identity()
            .append_scaling(30.0)
            .append_translation(&Vector2::<f32>::new(0.0, 0.0));
        let mut pb = PathBuilder::new();
        for l in figure.points().windows(2) {
            let p1 = m.transform_point(&l[0]);
            let p2 = m.transform_point(&l[1]);
            println!("{:?} {:?}", p1, p2);

            pb.move_to(p1.x, p1.y);
            pb.line_to(p2.x, p2.y);
        }
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
