//! "Pick symmetry" window: a scrollable grid of tile cards, grouped by the
//! symmetry categories of the tiled.art chart.
//!
//! The 35 thumbnails are triangulated once when the window opens (and again
//! when the palette changes), then reused every frame: only the vertex
//! positions are mapped onto each card's image rect while drawing.

use eframe::egui::{
    self, Color32, CornerRadius, Mesh, Pos2, Rect, Sense, Stroke, StrokeKind, Vec2,
};
use tessellations::tessellationfigure::{TessellationFigure, figure_categories};

use crate::app::FontScale;
use crate::figure::{FigureKind, Palette};

/// Size of one card, including its labels.
const CARD_SIZE: Vec2 = Vec2::new(160.0, 190.0);
/// Size of the square thumbnail drawn at the top of a card.
const THUMB_SIZE: f32 = 128.0;
/// Padding between a card's border and its contents.
const CARD_PADDING: f32 = 8.0;

/// One tile card: a thumbnail of the tiling plus its name and metadata.
struct Card {
    kind: FigureKind,
    name: String,
    meta: String,
    /// Filled tiles of the tiling, laid out in a unit square (0..1 on both
    /// axes), ready to be mapped onto a card's image rect.
    mesh: Mesh,
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
                        Card {
                            kind,
                            name: info.name(),
                            meta: format!("{}, IH {}, {}", info.heesch, info.ih, info.group),
                            mesh: thumbnail_mesh(&kind.build(), &colors),
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
        ui.painter().rect(
            rect,
            visuals.corner_radius,
            visuals.weak_bg_fill,
            visuals.bg_stroke,
            StrokeKind::Inside,
        );

        // Thumbnail: map the unit-square mesh onto the image rect.
        let image = Rect::from_min_size(
            Pos2::new(rect.left() + CARD_PADDING, rect.top() + CARD_PADDING),
            Vec2::splat(THUMB_SIZE),
        );
        let mut mesh = card.mesh.clone();
        for vertex in &mut mesh.vertices {
            vertex.pos =
                image.min + Vec2::new(vertex.pos.x * image.width(), vertex.pos.y * image.height());
        }
        ui.painter().add(egui::Shape::mesh(mesh));
        ui.painter().rect_stroke(
            image,
            CornerRadius::same(2),
            Stroke::new(1.0, visuals.bg_stroke.color),
            StrokeKind::Inside,
        );

        // Name and metadata, wrapped to the card width and centred.
        let text_color = visuals.text_color();
        let wrap = rect.width() - 2.0 * CARD_PADDING;
        let mut y = image.bottom() + 3.0;
        for (line, size) in [(&card.name, 12.0), (&card.meta, 10.0)] {
            let font = egui::FontId::proportional(size);
            let galley = ui.painter().layout(line.clone(), font, text_color, wrap);
            let x = rect.center().x - galley.size().x / 2.0;
            ui.painter()
                .galley(Pos2::new(x, y), galley.clone(), text_color);
            y += galley.size().y;
        }
    }

    response.clicked()
}

/// Build a unit-square mesh of the tiling's filled tiles: place a few tiles,
/// then centre and scale them so the bounding box fills (0, 0)..(1, 1).
fn thumbnail_mesh(figure: &TessellationFigure, colors: &[Color32; 4]) -> Mesh {
    let mut mesh = Mesh::default();
    let Some(tri) = FigureKind::triangulate(figure) else {
        return mesh;
    };
    let placements = figure.placements(3.0, 3.0);

    let mut min = Pos2::new(f32::INFINITY, f32::INFINITY);
    let mut max = Pos2::new(f32::NEG_INFINITY, f32::NEG_INFINITY);
    for placement in &placements {
        for v in &tri.vertices {
            let q = placement.transform.transform_point(*v);
            min = min.min(Pos2::new(q.x, q.y));
            max = max.max(Pos2::new(q.x, q.y));
        }
    }
    if !(min.x.is_finite() && min.y.is_finite() && max.x.is_finite() && max.y.is_finite()) {
        return mesh;
    }

    let span = max - min;
    let scale = 1.0 / span.x.max(span.y).max(1e-6);
    let center = min + span * 0.5;

    for placement in &placements {
        let color = colors[placement.color % colors.len()];
        let base = mesh.vertices.len() as u32;
        for v in &tri.vertices {
            let q = placement.transform.transform_point(*v);
            mesh.colored_vertex(
                Pos2::new(
                    (q.x - center.x) * scale + 0.5,
                    (q.y - center.y) * scale + 0.5,
                ),
                color,
            );
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
    fn every_card_has_a_unit_square_thumbnail() {
        let picker = Picker::build(Palette::Original);
        for (_, cards) in &picker.groups {
            for card in cards {
                assert!(!card.mesh.is_empty(), "{}", card.name);
                for v in &card.mesh.vertices {
                    assert!(
                        (-1e-4..=1.0 + 1e-4).contains(&v.pos.x)
                            && (-1e-4..=1.0 + 1e-4).contains(&v.pos.y),
                        "{}: {:?}",
                        card.name,
                        v.pos
                    );
                }
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
