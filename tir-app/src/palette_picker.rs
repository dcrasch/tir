//! "Palette" window: a grid of cards, each showing a palette's four colours
//! and its name. Clicking a card selects that palette.

use eframe::egui::{self, Color32, CornerRadius, Pos2, Rect, Sense, Stroke, StrokeKind, Vec2};

use crate::app::FontScale;
use crate::figure::Palette;

/// Size of one card: the swatch grid plus the name below it.
const CARD_SIZE: Vec2 = Vec2::new(190.0, 150.0);
/// Corner radius of a card, matching the symmetry picker's cards.
const CARD_RADIUS: u8 = 8;
/// Padding between a card's border and its contents.
const CARD_PADDING: f32 = 10.0;
/// Gap between the four colour swatches.
const SWATCH_GAP: f32 = 6.0;
/// Corner radius of a swatch.
const SWATCH_RADIUS: u8 = 4;
/// Height reserved below the swatches for the palette name.
const NAME_HEIGHT: f32 = 26.0;
/// Font size of the palette name.
const NAME_SIZE: f32 = 13.0;

/// Show the palette window, returning the palette the user clicked.
pub fn show(
    ctx: &egui::Context,
    open: &mut bool,
    current: Palette,
    font_scale: FontScale,
) -> Option<Palette> {
    let mut chosen = None;
    egui::Window::new("Palette")
        .open(open)
        .default_size([430.0, 360.0])
        .resizable(true)
        .show(ctx, |ui| {
            font_scale.apply(ui.style_mut());
            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    ui.horizontal_wrapped(|ui| {
                        for palette in Palette::ALL {
                            if card_ui(ui, palette, current) {
                                chosen = Some(palette);
                            }
                        }
                    });
                });
        });
    if chosen.is_some() {
        *open = false;
    }
    chosen
}

/// Draw one palette card and return whether it was clicked.
fn card_ui(ui: &mut egui::Ui, palette: Palette, current: Palette) -> bool {
    let (rect, response) = ui.allocate_exact_size(CARD_SIZE, Sense::click());
    let selected = palette == current;

    if ui.is_rect_visible(rect) {
        let visuals = ui.style().interact_selectable(&response, selected);
        let radius = CornerRadius::same(CARD_RADIUS);
        ui.painter().rect_filled(rect, radius, visuals.weak_bg_fill);

        // A 2x2 grid with one swatch per palette colour.
        let inner = rect.shrink(CARD_PADDING);
        let swatch_area = Rect::from_min_size(
            inner.min,
            Vec2::new(inner.width(), inner.height() - NAME_HEIGHT),
        );
        let swatch = Vec2::new(
            (swatch_area.width() - SWATCH_GAP) / 2.0,
            (swatch_area.height() - SWATCH_GAP) / 2.0,
        );
        for (i, color) in palette.colors().into_iter().enumerate() {
            let offset = Vec2::new(
                (i % 2) as f32 * (swatch.x + SWATCH_GAP),
                (i / 2) as f32 * (swatch.y + SWATCH_GAP),
            );
            let swatch_rect = Rect::from_min_size(swatch_area.min + offset, swatch);
            let swatch_radius = CornerRadius::same(SWATCH_RADIUS);
            ui.painter().rect_filled(swatch_rect, swatch_radius, color);
            // A faint outline keeps very light swatches visible on light cards.
            ui.painter().rect_stroke(
                swatch_rect,
                swatch_radius,
                Stroke::new(1.0, Color32::from_black_alpha(30)),
                StrokeKind::Inside,
            );
        }

        // The palette name below the swatches.
        let text_color = visuals.text_color();
        let galley = ui.painter().layout(
            palette.label().to_owned(),
            egui::FontId::proportional(NAME_SIZE),
            text_color,
            inner.width(),
        );
        ui.painter().galley(
            Pos2::new(inner.left(), swatch_area.bottom() + SWATCH_GAP),
            galley,
            text_color,
        );

        ui.painter()
            .rect_stroke(rect, radius, visuals.bg_stroke, StrokeKind::Inside);
    }

    response.clicked()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_card_per_palette() {
        assert_eq!(Palette::ALL.len(), 5);
        for palette in Palette::ALL {
            assert_eq!(palette.colors().len(), 4, "{}", palette.label());
            assert!(!palette.label().is_empty());
        }
    }

    #[test]
    fn palette_window_runs_headless() {
        let ctx = egui::Context::default();
        for size in [egui::vec2(900.0, 700.0), egui::vec2(360.0, 640.0)] {
            for font_scale in [
                FontScale::Small,
                FontScale::Large,
                FontScale::ExtraLarge(1.8),
            ] {
                let mut open = true;
                let input = || egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, size)),
                    ..Default::default()
                };
                let mut out = ctx.run_ui(input(), |ui| {
                    let _ = show(ui.ctx(), &mut open, Palette::Alhambra, font_scale);
                });
                out.textures_delta.clear();
            }
        }
    }
}
