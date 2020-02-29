use crate::tessellationfigure::{TessellationFigure, TessellationPlane};
use euclid::Angle;
use raqote::*;

#[derive(Clone, Copy)]
pub struct Backend;

pub trait Render {
    fn render_to_image(
        &self,
        figure: &TessellationFigure,
        m: &Transform,
    ) -> Option<Box<dyn OutputImage>>;

    fn render_plane_to_image(
        &self,
        plane: &TessellationPlane,
        figure: &TessellationFigure,
        m: &Transform,
    ) -> Option<Box<dyn OutputImage>>;
}

impl Render for Backend {
    fn render_to_image(
        &self,
        figure: &TessellationFigure,
        m: &Transform,
    ) -> Option<Box<dyn OutputImage>> {
        let mut dt = DrawTarget::new(400, 400);
        let colors = vec![
            SolidSource {
                r: 0x0,
                g: 0x88,
                b: 0x0,
                a: 0xff,
            },
            SolidSource {
                r: 0x0,
                g: 0x0,
                b: 0x88,
                a: 0xff,
            },
            SolidSource {
                r: 0xff,
                g: 0x88,
                b: 0x0,
                a: 0xff,
            },
            SolidSource {
                r: 0xff,
                g: 0x0,
                b: 0xff,
                a: 0xff,
            },
        ];
        // white background
        dt.clear(SolidSource {
            r: 0xff,
            g: 0xff,
            b: 0xff,
            a: 0xff,
        });

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
        let p1 = points[0];
        pb.move_to(p1.x, p1.y);
        for p in points.iter().skip(1) {
            pb.line_to(p.x, p.y);
        }
        pb.close();
        let path = pb.finish();

        dt.stroke(
            &path,
            &Source::Solid(colors[0]),
            &StrokeStyle {
                cap: LineCap::Square,
                join: LineJoin::Bevel,
                width: 1.0,
                miter_limit: 1.,
                dash_array: vec![],
                dash_offset: 0.,
            },
            &DrawOptions::new(),
        );

        Some(Box::new(dt))
    }
    fn render_plane_to_image(
        &self,
        plane: &TessellationPlane,
        figure: &TessellationFigure,
        m: &Transform,
    ) -> Option<Box<dyn OutputImage>> {
        let mut dt = DrawTarget::new(400, 400);
        let colors = vec![
            SolidSource {
                r: 0xf6,
                g: 0x88,
                b: 0xbb,
                a: 0xff,
            },
            SolidSource {
                r: 0xe8,
                g: 0xf9,
                b: 0xe9,
                a: 0xff,
            },
            SolidSource {
                r: 0xba,
                g: 0xfa,
                b: 0xa1,
                a: 0xff,
            },
            SolidSource {
                r: 0x9d,
                g: 0xe3,
                b: 0xd0,
                a: 0xff,
            },
        ];
        // white background
        dt.clear(SolidSource {
            r: 0xff,
            g: 0xff,
            b: 0xff,
            a: 0xff,
        });
        let mut row = 0;
        let g = plane.grid(&figure, 400.0, 400.0, 70.);
        let mut c = 0;

        for rotdiv in 1..=figure.rotdiv {
            let angle = Angle::degrees(360.0 * (rotdiv as f32) / (figure.rotdiv as f32));

            for gridrow in &g {
                if !figure.is_reversed {
                    c = row % 2;
                }
                for gridpoint in gridrow {
                    if figure.is_reversed {
                        c = rotdiv - 1;
                    }
                    if !figure.is_reversed && figure.gridincy < figure.gridincx {
                        c = row % 3;
                    }

                    let m = Transform::create_rotation(angle)
                        .post_scale(70.0, 70.0)
                        .post_translate(euclid::vec2(gridpoint.x, gridpoint.y));
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
                    let mut pb = PathBuilder::new();

                    pb.move_to(p1.x, p1.y);
                    for p in points.iter().skip(1) {
                        pb.line_to(p.x, p.y);
                    }
                    pb.close();
                    let path = pb.finish();

                    dt.fill(
                        &path,
                        &Source::Solid(colors[(c % 4) as usize]),
                        &DrawOptions::new(),
                    );
                    c += 1;
                }
                row += 1;
            }
        }

        // render image
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
        let p1 = points[0];
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
                g: 0x88,
                b: 0x0,
                a: 0xff,
            }),
            &StrokeStyle {
                cap: LineCap::Square,
                join: LineJoin::Bevel,
                width: 3.0,
                miter_limit: 1.,
                dash_array: vec![],
                dash_offset: 0.,
            },
            &DrawOptions::new(),
        );

        Some(Box::new(dt))
    }
}

pub trait OutputImage {
    /// Saves rendered image to the selected path.
    fn save_png(&self, path: &std::path::Path) -> bool;
    fn get_data(&self) -> &[u32];
}

impl OutputImage for raqote::DrawTarget {
    fn save_png(&self, path: &::std::path::Path) -> bool {
        self.write_png(path).is_ok()
    }
    fn get_data(&self) -> &[u32] {
        self.get_data()
    }
}
