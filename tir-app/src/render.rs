//! Rendering and triangulation for tessellation figures.

use eframe::egui::{Color32, Mesh, Pos2, Shape, Stroke};
use lyon::tessellation::{BuffersBuilder, FillOptions, FillTessellator, FillVertex, VertexBuffers};
use tessellations::render::{Builder, LyonBackend};
use tessellations::tessellationfigure::TessellationFigure;
use tessellations::tessellationline::Point;

use crate::figure::{FigureKind, HANDLE_RADIUS_PX, OUTLINE, Settings, View};

impl FigureKind {
    /// Unique polygon points of the figure (crate emits shared endpoints twice).
    pub fn polygon(figure: &TessellationFigure) -> Vec<Point> {
        let pts = figure.points();
        pts.windows(2)
            .filter_map(|w| (w[0] != w[1]).then_some(w[0]))
            .collect()
    }

    /// Triangulate the figure once, in figure space. Works for concave and
    /// self-intersecting shapes, which egui's convex polygon fill does not.
    pub fn triangulate(figure: &TessellationFigure) -> Option<VertexBuffers<Point, u32>> {
        let path = LyonBackend.build(figure)?;
        let mut buffers: VertexBuffers<Point, u32> = VertexBuffers::new();
        FillTessellator::new()
            .tessellate_path(
                &path,
                &FillOptions::tolerance(0.001),
                &mut BuffersBuilder::new(&mut buffers, |v: FillVertex| {
                    let p = v.position();
                    Point::new(p.x, p.y)
                }),
            )
            .ok()?;
        Some(buffers)
    }
}

/// Palette slot (0..4) for the copy of the figure at lattice `origin`,
/// rotated by `rot` (1..=rotdiv) steps of 360°/rotdiv.
pub fn tile_color_index(figure: &TessellationFigure, origin: Point, rot: u32) -> usize {
    let idx = if figure.rotdiv > 1 {
        // Rotational figures: one colour per rotation (like the crate).
        rot.saturating_sub(1)
    } else {
        // Lattice indices of this origin: origin = r*(shiftx, gridincy) + k*(gridincx, 0).
        let r = (origin.y / figure.gridincy).round() as i32;
        let k = ((origin.x - r as f32 * figure.shiftx) / figure.gridincx).round() as i32;
        // Translational figures: neighbours in a row and in
        // adjacent rows always get different colours.
        (k + 2 * r.rem_euclid(2)).rem_euclid(4) as u32
    };
    (idx % 4) as usize
}

/// Darker, semi-transparent version of a tile color, for its outline.
pub fn edge_color(color: Color32) -> Color32 {
    let dark = |c: u8| (c as f32 * 0.7) as u8;
    Color32::from_rgba_unmultiplied(dark(color.r()), dark(color.g()), dark(color.b()), 200)
}

/// Fill the whole viewport with copies of the figure, colored like the crate's renderer.
pub fn paint_tiles(
    painter: &eframe::egui::Painter,
    figure: &TessellationFigure,
    plane: &tessellations::tessellationfigure::TessellationPlane,
    view: View,
    rect: eframe::egui::Rect,
    settings: &Settings,
) {
    let Settings {
        palette,
        show_edges,
        show_grid,
        ..
    } = *settings;
    let Some(tri) = FigureKind::triangulate(figure) else {
        return;
    };

    let colors = palette.colors();

    // Tile origins from the crate (tessellations >= 0.3.1). `grid()` covers a
    // width x height area centered on the figure origin (0, 0), so pass
    // twice the farthest visible distance from 0 on each axis. The figure is
    // drawn `rotdiv` times at each origin, rotated by 360°/rotdiv.
    let lo = view.to_figure(rect.min);
    let hi = view.to_figure(rect.max);
    let width = 2.0 * lo.x.abs().max(hi.x.abs());
    let height = 2.0 * lo.y.abs().max(hi.y.abs());
    let grid = plane.grid(figure, width, height);

    let mut mesh = Mesh::default();
    // Outline polygon, shared by all tiles (only needed for edges).
    let polygon = if show_edges {
        FigureKind::polygon(figure)
    } else {
        Vec::new()
    };

    // Collect outline paths for antialiasing (only if show_edges is true)
    let mut outlines: Vec<(Vec<Pos2>, Color32)> = Vec::new();

    for origin in grid.iter().flatten() {
        for rot in 1..=figure.rotdiv {
            let color = colors[tile_color_index(figure, *origin, rot)];
            let angle = std::f32::consts::TAU * rot as f32 / figure.rotdiv as f32;
            let (sin, cos) = angle.sin_cos();

            let base = mesh.vertices.len() as u32;
            for v in &tri.vertices {
                let p = Point::new(
                    v.x * cos - v.y * sin + origin.x,
                    v.x * sin + v.y * cos + origin.y,
                );
                mesh.colored_vertex(view.to_screen(p), color);
            }
            mesh.indices.extend(tri.indices.iter().map(|i| base + i));

            // Only create outlines if show_edges is true
            if show_edges {
                // Create outline for this tile
                let mut outline_points: Vec<Pos2> = polygon
                    .iter()
                    .map(|p| {
                        let rotated = Point::new(
                            p.x * cos - p.y * sin + origin.x,
                            p.x * sin + p.y * cos + origin.y,
                        );
                        view.to_screen(rotated)
                    })
                    .collect();

                if let Some(&first) = outline_points.first() {
                    outline_points.push(first);
                }

                // Darker outline color for subtle edge definition
                let outline_color = edge_color(color);
                outlines.push((outline_points, outline_color));
            }
        }
    }

    // Draw filled tiles
    painter.add(Shape::mesh(mesh));

    // Draw grid lines if enabled (very light)
    if show_grid {
        draw_grid_lines(painter, figure, view, rect);
    }

    // Draw subtle outlines for antialiasing effect (only if show_edges is true)
    for (points, color) in outlines {
        painter.add(Shape::line(points, Stroke::new(1.0, color)));
    }
}

/// Draw very light grid lines showing the original tile layout.
fn draw_grid_lines(
    painter: &eframe::egui::Painter,
    figure: &TessellationFigure,
    view: View,
    rect: eframe::egui::Rect,
) {
    let lo = view.to_figure(rect.min);
    let hi = view.to_figure(rect.max);

    // Very light gray for grid lines
    let grid_color = Color32::from_rgba_unmultiplied(180, 180, 180, 60); // Very transparent light gray

    // Draw vertical grid lines
    let x_start = (lo.x / figure.gridincx).floor() as i32;
    let x_end = (hi.x / figure.gridincx).ceil() as i32;

    for i in x_start..=x_end {
        let x = i as f32 * figure.gridincx;
        let p1 = view.to_screen(Point::new(x, lo.y));
        let p2 = view.to_screen(Point::new(x, hi.y));
        painter.line_segment([p1, p2], Stroke::new(0.5, grid_color));
    }

    // Draw horizontal grid lines
    let y_start = (lo.y / figure.gridincy).floor() as i32;
    let y_end = (hi.y / figure.gridincy).ceil() as i32;

    // For shifted rows (like brick pattern)
    for j in y_start..=y_end {
        let y = j as f32 * figure.gridincy;
        let shift = j as f32 * figure.shiftx;

        let x1 = lo.x;
        let x2 = hi.x;

        let p1 = view.to_screen(Point::new(x1 + shift, y));
        let p2 = view.to_screen(Point::new(x2 + shift, y));
        painter.line_segment([p1, p2], Stroke::new(0.5, grid_color));
    }
}

/// Draw the editable figure outline and handles.
pub fn paint_editor(
    painter: &eframe::egui::Painter,
    figure: &TessellationFigure,
    selected: Option<tessellations::tessellationline::PointIndexPath>,
    view: View,
) {
    // Green outline of the editable figure.
    let mut outline: Vec<Pos2> = FigureKind::polygon(figure)
        .into_iter()
        .map(|p| view.to_screen(p))
        .collect();
    if let Some(first) = outline.first().copied() {
        outline.push(first);
    }
    painter.add(Shape::line(outline, Stroke::new(3.0, OUTLINE)));

    // Handles. Interior points of each line are movable, on both the line
    // itself (corrp = false) and its transformed copy (corrp = true).
    // Line endpoints are the fixed corners.
    for (li, line) in figure.lines.iter().enumerate() {
        for (corrp, pts) in [(false, line.dpoints()), (true, line.cpoints())] {
            let n = pts.len();
            for (pi, p) in pts.into_iter().enumerate() {
                let pos = view.to_screen(p);
                if pi == 0 || pi == n - 1 {
                    painter.circle_filled(pos, 3.0, Color32::DARK_GRAY);
                    continue;
                }
                let is_sel = selected.is_some_and(|s| s.line_index == li && s.point_index == pi);
                let (fill, r) = match (is_sel, selected.map(|s| s.corrp == corrp)) {
                    (true, Some(true)) => (Color32::YELLOW, HANDLE_RADIUS_PX + 2.0),
                    (true, _) => (Color32::from_rgb(0xff, 0xe0, 0x80), HANDLE_RADIUS_PX),
                    _ => (Color32::WHITE, HANDLE_RADIUS_PX),
                };
                painter.circle(pos, r, fill, Stroke::new(2.0, OUTLINE));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::figure::Palette;
    use eframe::egui::{Pos2, Rect};

    /// Signed area of a closed polygon (shoelace).
    fn area(poly: &[Point]) -> f32 {
        (0..poly.len())
            .map(|i| {
                let (a, b) = (poly[i], poly[(i + 1) % poly.len()]);
                a.x * b.y - b.x * a.y
            })
            .sum::<f32>()
            .abs()
            / 2.0
    }

    #[test]
    fn polygon_has_no_repeated_points() {
        for kind in FigureKind::ALL {
            let poly = FigureKind::polygon(&kind.build());
            for w in poly.windows(2) {
                assert_ne!(w[0], w[1], "{kind:?}: {poly:?}");
            }
        }
    }

    #[test]
    fn triangulation_area_matches_polygon() {
        for kind in FigureKind::ALL {
            let fig = kind.build();
            let tri = FigureKind::triangulate(&fig).unwrap();
            assert_eq!(tri.indices.len() % 3, 0, "{kind:?}");
            let tri_area: f32 = tri
                .indices
                .chunks(3)
                .map(|t| area(&[0, 1, 2].map(|j| tri.vertices[t[j] as usize])))
                .sum();
            let poly_area = area(&FigureKind::polygon(&fig));
            assert!(
                (tri_area - poly_area).abs() < 1e-3,
                "{kind:?}: {tri_area} vs {poly_area}"
            );
        }
    }

    #[test]
    fn color_index_in_range() {
        for kind in FigureKind::ALL {
            let fig = kind.build();
            for x in -3..=3 {
                for y in -3..=3 {
                    let o = Point::new(x as f32 * fig.gridincx, y as f32 * fig.gridincy);
                    for rot in 1..=fig.rotdiv {
                        assert!(tile_color_index(&fig, o, rot) < 4, "{kind:?}");
                    }
                }
            }
        }
    }

    #[test]
    fn rotated_copies_get_distinct_colors() {
        for kind in FigureKind::ALL {
            let fig = kind.build();
            if fig.rotdiv <= 1 {
                continue;
            }
            let o = Point::new(0.0, 0.0);
            let idx: Vec<usize> = (1..=fig.rotdiv)
                .map(|rot| tile_color_index(&fig, o, rot))
                .collect();
            for (i, a) in idx.iter().enumerate() {
                for b in &idx[i + 1..] {
                    // Only possible to keep apart when rotdiv <= palette size.
                    if fig.rotdiv <= 4 {
                        assert_ne!(a, b, "{kind:?}: {idx:?}");
                    }
                }
            }
        }
    }

    /// Translational figures: horizontal and vertical neighbours differ.
    #[test]
    fn neighbouring_tiles_get_different_colors() {
        for kind in FigureKind::ALL {
            let fig = kind.build();
            if fig.rotdiv > 1 {
                continue;
            }
            let at = |k: i32, r: i32| {
                let o = Point::new(
                    k as f32 * fig.gridincx + r as f32 * fig.shiftx,
                    r as f32 * fig.gridincy,
                );
                tile_color_index(&fig, o, 1)
            };
            for r in -4..=4 {
                for k in -4..=4 {
                    assert_ne!(at(k, r), at(k + 1, r), "{kind:?} row ({k},{r})");
                    assert_ne!(at(k, r), at(k, r + 1), "{kind:?} col ({k},{r})");
                }
            }
        }
    }

    #[test]
    fn edge_color_is_darker_and_translucent() {
        for palette in Palette::ALL {
            for c in palette.colors() {
                let e = edge_color(c);
                assert_eq!(e.a(), 200);
                // Unmultiply to compare the actual colour channels.
                let [r, g, b, _] = e.to_srgba_unmultiplied();
                assert!(r <= c.r() && g <= c.g() && b <= c.b(), "{c:?} -> {e:?}");
            }
        }
        assert_eq!(
            edge_color(Color32::BLACK).to_srgba_unmultiplied(),
            [0, 0, 0, 200]
        );
    }

    /// Painting into a headless context must not panic, for every option combo.
    #[test]
    fn paint_tiles_all_settings() {
        let ctx = eframe::egui::Context::default();
        let plane = tessellations::tessellationfigure::TessellationPlane {};
        let rect = Rect::from_min_size(Pos2::ZERO, eframe::egui::vec2(400.0, 300.0));
        let view = View {
            scale: 50.0,
            origin: rect.center(),
        };
        let painter =
            eframe::egui::Painter::new(ctx.clone(), eframe::egui::LayerId::background(), rect);
        for kind in FigureKind::ALL {
            let fig = kind.build();
            for palette in Palette::ALL {
                for (show_edges, show_grid) in
                    [(false, false), (true, false), (false, true), (true, true)]
                {
                    let settings = Settings {
                        palette,
                        show_edges,
                        show_grid,
                        ..Settings::default()
                    };
                    paint_tiles(&painter, &fig, &plane, view, rect, &settings);
                }
            }
            paint_editor(&painter, &fig, None, view);
        }
    }
}
