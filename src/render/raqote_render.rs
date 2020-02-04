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

        let mut pb = PathBuilder::new();
        pb.move_to(10., 10.);
        pb.line_to(10., 110.);
        pb.line_to(110., 110.);
        pb.line_to(110., 10.);
        pb.line_to(10., 10.);
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
