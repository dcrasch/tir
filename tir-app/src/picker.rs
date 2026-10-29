//! "Pick symmetry" window: a scrollable grid of tile cards, grouped by the
//! symmetry categories of the tiled.art chart.
//!
//! The 35 thumbnails are triangulated once when the window opens (and again
//! when the palette changes), then reused every frame: only the vertex
//! positions are mapped onto each card's image rect while drawing.

use eframe::egui::{
    self, Color32, CornerRadius, Mesh, Pos2, Rect, Sense, Shape, Stroke, StrokeKind, Vec2,
};
use tessellations::tessellationfigure::{TessellationFigure, figure_categories};
use tessellations::tessellationline::Point;

use crate::app::FontScale;
use crate::figure::{FigureKind, Palette};

/// Size of the square thumbnail drawn at the top of a card. The card is as
/// wide as its thumbnail, so the image sits flush with the card's edges.
const THUMB_SIZE: f32 = 128.0;
/// Size of one card: the thumbnail plus the padded labels below it.
const CARD_SIZE: Vec2 = Vec2::new(THUMB_SIZE, 210.0);
/// Corner radius of a card, matching tir-web's `border-radius: 8px`.
const CARD_RADIUS: u8 = 8;
/// Padding around a card's caption, matching tir-web's `.card .caption`.
const CAPTION_PADDING: f32 = 8.0;

/// Figure units shown across a thumbnail, matching tir-web's view
/// (`VIEW_SIZE / VIEW_SCALE`). The tiling is drawn at this fixed zoom, centred
/// on the figure's bounding box, instead of being scaled to fit the card.
const VIEW_SPAN: f32 = 4.0;
/// Width of the white halo drawn under the editable tile's outline, so it
/// stays visible on dark tiles.
const OUTLINE_HALO: f32 = 6.0;
/// Width of the dark line drawn over the halo, so it stays visible on light
/// tiles.
const OUTLINE_LINE: f32 = 2.0;

/// One tile card: a thumbnail of the tiling plus its name and metadata.
struct Card {
    kind: FigureKind,
    name: String,
    meta: String,
    /// Filled tiles of the tiling, in the thumbnail's unit square: the figure's
    /// `VIEW_SPAN`-wide window around its bounding box maps to 0..1 on both
    /// axes, so tiles may extend past it and are clipped when drawn.
    mesh: Mesh,
    /// Outline of the editable tile, in the same unit square as `mesh`.
    outline: Vec<Pos2>,
}

/// The picker's contents, built once per palette.
pub struct Picker {
    palette: Palette,
    groups: Vec<(&'static str, Vec<Card>)>,
}

impl Picker {
    /// Triangulate every figure and group the cards like the tiled.art chart.
    pub fn build(palette: Palette) -> Self {
        let colors = palette.colors();
        let groups = figure_categories()
            .into_iter()
            .map(|(name, members)| {
                let cards = members
                    .into_iter()
                    .map(|index| {
                        let kind = FigureKind::from_index(index);
                        let info = kind.info();
                        let figure = kind.build();
                        let center = bbox_center(&figure);
                        Card {
                            kind,
                            name: info.name(),
                            meta: format!("{}, IH {}, {}", info.heesch, info.ih, info.group),
                            mesh: thumbnail_mesh(&figure, &colors, center),
                            outline: FigureKind::polygon(&figure)
                                .into_iter()
                                .map(|p| project(p, center))
                                .collect(),
                        }
                    })
                    .collect();
                (name, cards)
            })
            .collect();
        Self { palette, groups }
    }

    /// True if the thumbnails were built for `palette`.
    pub fn matches(&self, palette: Palette) -> bool {
        self.palette == palette
    }

    /// Total number of cards (all 35 figures).
    pub fn card_count(&self) -> usize {
        self.groups.iter().map(|(_, cards)| cards.len()).sum()
    }

    /// Show the picker window, returning the figure the user clicked.
    pub fn show(
        &self,
        ctx: &egui::Context,
        open: &mut bool,
        current: FigureKind,
        font_scale: FontScale,
    ) -> Option<FigureKind> {
        let mut chosen = None;
        egui::Window::new("Pick symmetry")
            .open(open)
            .default_size([760.0, 580.0])
            .resizable(true)
            .show(ctx, |ui| {
                font_scale.apply(ui.style_mut());
                egui::ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        for (name, cards) in &self.groups {
                            ui.add_space(4.0);
                            ui.label(egui::RichText::new(name.to_uppercase()).weak());
                            ui.add_space(2.0);
                            ui.horizontal_wrapped(|ui| {
                                for card in cards {
                                    if card_ui(ui, card, current) {
                                        chosen = Some(card.kind);
                                    }
                                }
                            });
                        }
                    });
            });
        if chosen.is_some() {
            *open = false;
        }
        chosen
    }
}

/// Draw one card and return whether it was clicked.
fn card_ui(ui: &mut egui::Ui, card: &Card, current: FigureKind) -> bool {
    let (rect, response) = ui.allocate_exact_size(CARD_SIZE, Sense::click());
    let selected = card.kind == current;

    if ui.is_rect_visible(rect) {
        let visuals = ui.style().interact_selectable(&response, selected);
        let radius = CornerRadius::same(CARD_RADIUS);
        // Card background first; the border is stroked on top of the thumbnail
        // so the rounded corners stay crisp.
        ui.painter().rect_filled(rect, radius, visuals.weak_bg_fill);

        // Thumbnail: map the unit-square mesh onto the image rect, clipped to
        // it so tiles outside the fixed view are cut off. The image fills the
        // card's full width, with no padding.
        let image = Rect::from_min_size(rect.min, Vec2::splat(THUMB_SIZE));
        let painter = ui.painter().with_clip_rect(image);
        let mut mesh = card.mesh.clone();
        for vertex in &mut mesh.vertices {
            vertex.pos = to_image(vertex.pos, image);
        }
        painter.add(Shape::mesh(mesh));

        // The editable tile is outlined twice, like tir-web: a light halo below
        // so it stays visible on dark tiles, and a dark line above so it stays
        // visible on light ones.
        if card.outline.len() >= 3 {
            let mut outline: Vec<Pos2> = card.outline.iter().map(|p| to_image(*p, image)).collect();
            outline.push(outline[0]);
            painter.add(Shape::line(
                outline.clone(),
                Stroke::new(OUTLINE_HALO, Color32::WHITE),
            ));
            painter.add(Shape::line(
                outline,
                Stroke::new(OUTLINE_LINE, Color32::BLACK),
            ));
        }

        // Round the thumbnail's top corners to match the card, like tir-web's
        // `overflow: hidden`.
        round_top_corners(
            &painter,
            image,
            CARD_RADIUS as f32,
            ui.visuals().window_fill,
        );

        ui.painter()
            .rect_stroke(rect, radius, visuals.bg_stroke, StrokeKind::Inside);

        // Name and metadata, wrapped to the card width and aligned left, with
        // the same caption padding as tir-web.
        let text_color = visuals.text_color();
        let wrap = rect.width() - 2.0 * CAPTION_PADDING;
        let mut y = image.bottom() + CAPTION_PADDING;
        for (line, size) in [(&card.name, 12.0), (&card.meta, 10.0)] {
            let font = egui::FontId::proportional(size);
            let galley = ui.painter().layout(line.clone(), font, text_color, wrap);
            ui.painter().galley(
                Pos2::new(rect.left() + CAPTION_PADDING, y),
                galley.clone(),
                text_color,
            );
            y += galley.size().y;
        }
    }

    response.clicked()
}

/// Cover the top corners of `image` with `color` so they follow the card's
/// rounded border, the way tir-web clips a card's thumbnail with
/// `overflow: hidden`. The image's bottom corners are interior, so they stay
/// square.
fn round_top_corners(painter: &egui::Painter, image: Rect, radius: f32, color: Color32) {
    use std::f32::consts::{FRAC_PI_2, PI};
    const SEGMENTS: usize = 8;
    // Each corner: the card corner point, the arc's centre and the angle of its
    // first point. The arc sweeps a quarter turn from there.
    let corners = [
        (
            image.left_top(),
            Pos2::new(image.left() + radius, image.top() + radius),
            PI,
        ),
        (
            image.right_top(),
            Pos2::new(image.right() - radius, image.top() + radius),
            PI + FRAC_PI_2,
        ),
    ];
    let mut mesh = Mesh::default();
    for (corner, centre, start) in corners {
        let base = mesh.vertices.len() as u32;
        mesh.colored_vertex(corner, color);
        for i in 0..=SEGMENTS {
            let angle = start + FRAC_PI_2 * (i as f32 / SEGMENTS as f32);
            mesh.colored_vertex(centre + Vec2::angled(angle) * radius, color);
        }
        for i in 0..SEGMENTS as u32 {
            mesh.indices.extend([base, base + 1 + i, base + 2 + i]);
        }
    }
    painter.add(Shape::mesh(mesh));
}

/// Centre of the figure's bounding box, so the fixed-zoom view sits on the
/// figure like tir-web's `view_transform`.
fn bbox_center(figure: &TessellationFigure) -> Point {
    let points = figure.points();
    let Some(&first) = points.first() else {
        return Point::new(0.0, 0.0);
    };
    let (mut min, mut max) = (first, first);
    for p in &points {
        min = min.min(*p);
        max = max.max(*p);
    }
    min.lerp(max, 0.5)
}

/// Map a figure-space point into the thumbnail's unit square: the
/// `VIEW_SPAN`-wide window centred on `center` becomes 0..1 on both axes.
fn project(p: Point, center: Point) -> Pos2 {
    Pos2::new(
        (p.x - center.x) / VIEW_SPAN + 0.5,
        (p.y - center.y) / VIEW_SPAN + 0.5,
    )
}

/// Map a point in the thumbnail's unit square onto a card's image rect.
fn to_image(p: Pos2, image: Rect) -> Pos2 {
    image.min + Vec2::new(p.x * image.width(), p.y * image.height())
}

/// Build a mesh of the tiling's filled tiles, projected into the thumbnail's
/// unit square at the fixed `VIEW_SPAN` zoom.
fn thumbnail_mesh(figure: &TessellationFigure, colors: &[Color32; 4], center: Point) -> Mesh {
    let mut mesh = Mesh::default();
    let Some(tri) = FigureKind::triangulate(figure) else {
        return mesh;
    };
    // A generous area, like tir-web, so the fixed window is covered even when
    // the figure's bounding box is off-centre.
    for placement in &figure.placements(12.0, 12.0) {
        let color = colors[placement.color % colors.len()];
        let base = mesh.vertices.len() as u32;
        for v in &tri.vertices {
            let q = placement.transform.transform_point(*v);
            mesh.colored_vertex(project(q, center), color);
        }
        mesh.indices.extend(tri.indices.iter().map(|i| base + i));
    }
    mesh
}

#[cfg(test)]
mod tests {
    use super::*;
    use tessellations::tessellationfigure::FIGURE_IDS;

    #[test]
    fn groups_match_the_tiled_art_chart() {
        let picker = Picker::build(Palette::Original);
        let names: Vec<&str> = picker.groups.iter().map(|(name, _)| *name).collect();
        assert_eq!(
            names,
            vec![
                "1-way",
                "2-way",
                "2-way with flip",
                "3-way",
                "4-way",
                "4-way with flip",
                "6-way",
            ]
        );
        let sizes: Vec<usize> = picker.groups.iter().map(|(_, cards)| cards.len()).collect();
        assert_eq!(sizes, vec![4, 8, 4, 3, 4, 8, 4]);
        assert_eq!(picker.card_count(), 35);
    }

    #[test]
    fn every_card_has_a_thumbnail_and_a_centred_outline() {
        let picker = Picker::build(Palette::Original);
        for (_, cards) in &picker.groups {
            for card in cards {
                assert!(!card.mesh.is_empty(), "{}", card.name);
                assert!(card.outline.len() >= 3, "{}", card.name);
                let (mut min, mut max) = (
                    Pos2::new(f32::INFINITY, f32::INFINITY),
                    Pos2::new(f32::NEG_INFINITY, f32::NEG_INFINITY),
                );
                for p in &card.outline {
                    assert!(
                        (0.0..=1.0).contains(&p.x) && (0.0..=1.0).contains(&p.y),
                        "{}: {p:?}",
                        card.name
                    );
                    min = min.min(*p);
                    max = max.max(*p);
                }
                // The view is centred on the editable tile's bounding box.
                let center = min + (max - min) * 0.5;
                assert!(
                    (center.x - 0.5).abs() < 1e-3 && (center.y - 0.5).abs() < 1e-3,
                    "{}: {center:?}",
                    card.name
                );
                assert!(!card.name.is_empty());
                assert!(!card.meta.is_empty());
            }
        }
    }

    #[test]
    fn palette_change_requires_a_rebuild() {
        let picker = Picker::build(Palette::Original);
        assert!(picker.matches(Palette::Original));
        assert!(!picker.matches(Palette::Grayscale));
    }

    #[test]
    fn every_figure_appears_once() {
        let picker = Picker::build(Palette::Original);
        let mut kinds: Vec<usize> = picker
            .groups
            .iter()
            .flat_map(|(_, cards)| cards.iter().map(|c| c.kind.index()))
            .collect();
        kinds.sort_unstable();
        assert_eq!(kinds, (0..FIGURE_IDS.len()).collect::<Vec<_>>());
    }
}
