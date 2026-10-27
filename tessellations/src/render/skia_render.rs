use crate::tessellationfigure::{TessellationFigure, TessellationPlane};
use crate::tessellationline::{Point, Transform};
use tiny_skia::{Color, FillRule, LineCap, LineJoin, Paint, Path, PathBuilder, Pixmap, Stroke};

const WIDTH: u32 = 400;
const HEIGHT: u32 = 400;

#[derive(Clone, Copy)]
pub struct Backend;

pub trait Render {
    /// Render a figure to a image
    fn render_to_image(
        &self,
        figure: &TessellationFigure,
        m: &Transform,
    ) -> Option<Box<dyn OutputImage>>;

    /// Render a figure and tiling on a plane to a image
    fn render_plane_to_image(
        &self,
        plane: &TessellationPlane,
        figure: &TessellationFigure,
        m: &Transform,
    ) -> Option<Box<dyn OutputImage>>;
}

/// Build a closed path from the figure outline, transformed by `m`.
fn figure_path(figure: &TessellationFigure, m: &Transform) -> Option<Path> {
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
    let (p1, rest) = points.split_first()?;
    let mut pb = PathBuilder::new();
    pb.move_to(p1.x, p1.y);
    for p in rest {
        pb.line_to(p.x, p.y);
    }
    pb.close();
    pb.finish()
}

fn paint(r: u8, g: u8, b: u8) -> Paint<'static> {
    let mut paint = Paint::default();
    paint.set_color_rgba8(r, g, b, 0xff);
    paint.anti_alias = true;
    paint
}

fn stroke(width: f32) -> Stroke {
    Stroke {
        width,
        miter_limit: 1.0,
        line_cap: LineCap::Square,
        line_join: LineJoin::Bevel,
        dash: None,
    }
}

impl Render for Backend {
    fn render_to_image(
        &self,
        figure: &TessellationFigure,
        m: &Transform,
    ) -> Option<Box<dyn OutputImage>> {
        let mut pixmap = Pixmap::new(WIDTH, HEIGHT)?;
        // white background
        pixmap.fill(Color::WHITE);

        if let Some(path) = figure_path(figure, m) {
            pixmap.stroke_path(
                &path,
                &paint(0x0, 0x88, 0x0),
                &stroke(1.0),
                tiny_skia::Transform::identity(),
                None,
            );
        }

        Some(Box::new(pixmap))
    }

    fn render_plane_to_image(
        &self,
        plane: &TessellationPlane,
        figure: &TessellationFigure,
        m: &Transform,
    ) -> Option<Box<dyn OutputImage>> {
        let mut pixmap = Pixmap::new(WIDTH, HEIGHT)?;
        let colors = [
            paint(0xf6, 0x88, 0xbb),
            paint(0xe8, 0xf9, 0xe9),
            paint(0xba, 0xfa, 0xa1),
            paint(0x9d, 0xe3, 0xd0),
        ];
        // white background
        pixmap.fill(Color::WHITE);

        let tile = figure_path(figure, &Transform::identity())?;
        let ncolors = figure.colors.clamp(1, colors.len());
        for placement in plane.placements(figure, 12.0, 12.0) {
            let t = placement.transform.then(m);
            pixmap.fill_path(
                &tile,
                &colors[placement.color % ncolors],
                FillRule::Winding,
                tiny_transform(&t),
                None,
            );
        }

        if let Some(path) = figure_path(figure, m) {
            pixmap.stroke_path(
                &path,
                &paint(0x0, 0x88, 0x0),
                &stroke(3.0),
                tiny_skia::Transform::identity(),
                None,
            );
        }

        Some(Box::new(pixmap))
    }
}

/// Convert a euclid transform to the tiny-skia row convention.
fn tiny_transform(t: &Transform) -> tiny_skia::Transform {
    tiny_skia::Transform::from_row(t.m11, t.m12, t.m21, t.m22, t.m31, t.m32)
}

pub trait OutputImage {
    /// Saves rendered image to the selected path.
    fn save_png(&self, path: &std::path::Path) -> bool;

    /// Returns a reference to the underlying pixel data as bytes in RGBA order
    /// (premultiplied alpha).
    fn get_data(&self) -> &[u8];

    /// Returns a mut reference to the underlying pixel data as bytes in RGBA order
    /// (premultiplied alpha).
    fn get_data_u8(&mut self) -> &mut [u8];
}

impl OutputImage for Pixmap {
    fn save_png(&self, path: &std::path::Path) -> bool {
        Pixmap::save_png(self, path).is_ok()
    }

    fn get_data(&self) -> &[u8] {
        self.data()
    }

    fn get_data_u8(&mut self) -> &mut [u8] {
        self.data_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pixel(image: &dyn OutputImage, x: u32, y: u32) -> [u8; 4] {
        let i = ((y * WIDTH + x) * 4) as usize;
        image.get_data()[i..i + 4].try_into().unwrap()
    }

    #[test]
    fn test_render_plane_fills_tiles() {
        let f = TessellationFigure::square();
        let m = Transform::scale(100.0, 100.0).then_translate(euclid::vec2(100.0, 100.0));
        let image = Backend
            .render_plane_to_image(&TessellationPlane {}, &f, &m)
            .unwrap();
        assert_eq!(image.get_data().len(), (WIDTH * HEIGHT * 4) as usize);
        // Inside a tile: one of the palette colours, not the white background.
        assert_ne!(pixel(image.as_ref(), 35, 35), [0xff, 0xff, 0xff, 0xff]);
        // On the outline of the edited figure (x = 100, y = 150): green stroke.
        assert_eq!(pixel(image.as_ref(), 100, 150), [0x0, 0x88, 0x0, 0xff]);
    }

    #[test]
    fn test_render_figure_outline() {
        let f = TessellationFigure::square();
        let m = Transform::scale(100.0, 100.0).then_translate(euclid::vec2(100.0, 100.0));
        let image = Backend.render_to_image(&f, &m).unwrap();
        assert_eq!(pixel(image.as_ref(), 150, 150), [0xff, 0xff, 0xff, 0xff]);
        assert_ne!(pixel(image.as_ref(), 100, 150), [0xff, 0xff, 0xff, 0xff]);
    }
}
