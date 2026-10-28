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

/// Darker, semi-transparent version of a tile color, for its outline.
pub fn edge_color(color: Color32) -> Color32 {
    let dark = |c: u8| (c as f32 * 0.7) as u8;
    Color32::from_rgba_unmultiplied(dark(color.r()), dark(color.g()), dark(color.b()), 200)
}

/// Fill the whole viewport with copies of the figure, colored like the crate's renderer.
pub fn paint_tiles(
    painter: &eframe::egui::Painter,
    figure: &TessellationFigure,
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

    // `placements()` covers a width x height area centered on the figure
    // origin (0, 0), so pass twice the farthest visible distance from 0 on
    // each axis. Each placement already carries its isometry (rotation,
    // reflection and translation) and the colour chosen by the crate.
    let lo = view.to_figure(rect.min);
    let hi = view.to_figure(rect.max);
    let width = 2.0 * lo.x.abs().max(hi.x.abs());
    let height = 2.0 * lo.y.abs().max(hi.y.abs());
    let placements = figure.placements(width, height);

    let mut mesh = Mesh::default();
    // Outline polygon, shared by all tiles (only needed for edges).
    let polygon = if show_edges {
        FigureKind::polygon(figure)
    } else {
        Vec::new()
    };

    // Collect outline paths for antialiasing (only if show_edges is true)
    let mut outlines: Vec<(Vec<Pos2>, Color32)> = Vec::new();

    for placement in &placements {
        let color = colors[placement.color % colors.len()];
        let transform = placement.transform;

        let base = mesh.vertices.len() as u32;
        for v in &tri.vertices {
            let p = transform.transform_point(*v);
            mesh.colored_vertex(view.to_screen(p), color);
        }
        mesh.indices.extend(tri.indices.iter().map(|i| base + i));

        // Only create outlines if show_edges is true
        if show_edges {
            // Create outline for this tile
            let mut outline_points: Vec<Pos2> = polygon
                .iter()
                .map(|p| view.to_screen(transform.transform_point(*p)))
                .collect();

            if let Some(&first) = outline_points.first() {
                outline_points.push(first);
            }

            // Darker outline color for subtle edge definition
            let outline_color = edge_color(color);
            outlines.push((outline_points, outline_color));
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

/// The two shortest independent translation vectors of the tiling.
fn lattice_basis(figure: &TessellationFigure, width: f32, height: f32) -> Option<(Point, Point)> {
    let mut translations: Vec<Point> = figure
        .placements(width, height)
        .into_iter()
        .filter(|p| {
            let t = p.transform;
            (t.m11 - 1.0).abs() < 1e-4
                && t.m12.abs() < 1e-4
                && t.m21.abs() < 1e-4
                && (t.m22 - 1.0).abs() < 1e-4
        })
        .map(|p| Point::new(p.transform.m31, p.transform.m32))
        .filter(|p| p.x * p.x + p.y * p.y > 1e-6)
        .collect();
    let len2 = |p: &Point| p.x * p.x + p.y * p.y;
    translations.sort_by(|a, b| len2(a).partial_cmp(&len2(b)).unwrap());
    translations.dedup_by(|a, b| (*a - *b).square_length() < 1e-6);

    let t1 = *translations.first()?;
    let t2 = *translations
        .iter()
        .find(|t| (t1.x * t.y - t1.y * t.x).abs() > 1e-3)?;
    Some((t1, t2))
}

/// Draw very light grid lines along the tiling's translation lattice.
fn draw_grid_lines(
    painter: &eframe::egui::Painter,
    figure: &TessellationFigure,
    view: View,
    rect: eframe::egui::Rect,
) {
    let lo = view.to_figure(rect.min);
    let hi = view.to_figure(rect.max);
    let width = 2.0 * lo.x.abs().max(hi.x.abs());
    let height = 2.0 * lo.y.abs().max(hi.y.abs());
    let Some((t1, t2)) = lattice_basis(figure, width, height) else {
        return;
    };

    let det = t1.x * t2.y - t1.y * t2.x;
    if det.abs() < 1e-6 {
        return;
    }
    let cross = |a: Point, b: Point| a.x * b.y - a.y * b.x;

    // Very light gray for grid lines
    let grid_color = Color32::from_rgba_unmultiplied(180, 180, 180, 60);

    let corners = [lo, Point::new(hi.x, lo.y), hi, Point::new(lo.x, hi.y)];

    // Lines parallel to t1: p = j*t2 + s*t1.
    let (mut j_min, mut j_max) = (f32::INFINITY, f32::NEG_INFINITY);
    let (mut s_min, mut s_max) = (f32::INFINITY, f32::NEG_INFINITY);
    // Lines parallel to t2: p = i*t1 + s*t2.
    let (mut i_min, mut i_max) = (f32::INFINITY, f32::NEG_INFINITY);
    let (mut r_min, mut r_max) = (f32::INFINITY, f32::NEG_INFINITY);
    for &c in &corners {
        let j = cross(t1, c) / det;
        let s = cross(c, t2) / det;
        j_min = j_min.min(j);
        j_max = j_max.max(j);
        s_min = s_min.min(s);
        s_max = s_max.max(s);

        let i = cross(c, t2) / det;
        let r = cross(t1, c) / det;
        i_min = i_min.min(i);
        i_max = i_max.max(i);
        r_min = r_min.min(r);
        r_max = r_max.max(r);
    }

    for j in j_min.floor() as i32..=j_max.ceil() as i32 {
        let (ox, oy) = (t2.x * j as f32, t2.y * j as f32);
        painter.line_segment(
            [
                view.to_screen(Point::new(ox + t1.x * s_min, oy + t1.y * s_min)),
                view.to_screen(Point::new(ox + t1.x * s_max, oy + t1.y * s_max)),
            ],
            Stroke::new(0.5, grid_color),
        );
    }
    for i in i_min.floor() as i32..=i_max.ceil() as i32 {
        let (ox, oy) = (t1.x * i as f32, t1.y * i as f32);
        painter.line_segment(
            [
                view.to_screen(Point::new(ox + t2.x * r_min, oy + t2.y * r_min)),
                view.to_screen(Point::new(ox + t2.x * r_max, oy + t2.y * r_max)),
            ],
            Stroke::new(0.5, grid_color),
        );
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

    // Handles. Interior points of each line are movable; line endpoints are
    // the fixed corners. Built-in figures use identity line transforms, so
    // each edge's points already are its on-screen outline.
    for (li, line) in figure.lines.iter().enumerate() {
        let pts = line.points();
        let n = pts.len();
        for (pi, &p) in pts.iter().enumerate() {
            let pos = view.to_screen(p);
            if pi == 0 || pi == n - 1 {
                painter.circle_filled(pos, 3.0, Color32::DARK_GRAY);
                continue;
            }
            let is_sel = selected.is_some_and(|s| s.line_index == li && s.point_index == pi);
            let (fill, r) = if is_sel {
                (Color32::YELLOW, HANDLE_RADIUS_PX + 2.0)
            } else {
                (Color32::WHITE, HANDLE_RADIUS_PX)
            };
            painter.circle(pos, r, fill, Stroke::new(2.0, OUTLINE));
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
    fn placement_colors_are_in_palette_range() {
        let palette_len = Palette::Original.colors().len();
        for kind in FigureKind::ALL {
            let fig = kind.build();
            for p in fig.placements(8.0, 8.0) {
                assert!(p.color < fig.colors.max(1), "{kind:?}: colour {}", p.color);
                assert!(p.color % palette_len < palette_len, "{kind:?}");
            }
        }
    }

    /// Neighbouring tiles must get different colours.
    #[test]
    fn adjacent_tiles_get_different_colors() {
        use std::collections::HashMap;

        let key = |a: Point, b: Point| {
            let r = |v: f32| (v * 1000.0).round() as i64;
            let (a, b) = ((r(a.x), r(a.y)), (r(b.x), r(b.y)));
            if a <= b {
                (a.0, a.1, b.0, b.1)
            } else {
                (b.0, b.1, a.0, a.1)
            }
        };

        for kind in FigureKind::ALL {
            let fig = kind.build();
            let placements = fig.placements(8.0, 8.0);
            let mut edges: HashMap<(i64, i64, i64, i64), (usize, usize)> = HashMap::new();
            for (i, p) in placements.iter().enumerate() {
                for line in &fig.lines {
                    let pts = line.points();
                    let (Some(&a), Some(&b)) = (pts.first(), pts.last()) else {
                        continue;
                    };
                    let (a, b) = (
                        p.transform.transform_point(a),
                        p.transform.transform_point(b),
                    );
                    if a == b {
                        continue;
                    }
                    if let Some(&(j, color)) = edges.get(&key(a, b)) {
                        if j != i {
                            assert_ne!(color, p.color, "{kind:?}: tiles {i} and {j} share colour");
                        }
                    } else {
                        edges.insert(key(a, b), (i, p.color));
                    }
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
                    paint_tiles(&painter, &fig, view, rect, &settings);
                }
            }
            paint_editor(&painter, &fig, None, view);
        }
    }
}
