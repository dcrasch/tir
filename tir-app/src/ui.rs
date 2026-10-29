//! UI components: menu, toolbar, and widgets.

use eframe::egui;

use crate::app::FontScale;
use crate::figure::{FigureKind, Palette, Settings};

/// Extra text size for the menu popup, on top of the system font scale.
const MENU_TEXT_SCALE: f32 = 1.4;

/// Something the app has to act on after the menu was shown.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MenuAction {
    /// Open the "pick symmetry" window.
    PickSymmetry,
    /// Switch to the chosen figure (the picker's selection).
    SelectFigure(FigureKind),
    /// Discard edits and reload the current figure.
    Reset,
}

/// Hamburger menu holding all toolbar controls. View options are edited in
/// place; figure changes are returned so the app can rebuild the figure.
pub fn menu(
    ui: &mut egui::Ui,
    settings: &mut Settings,
    font_scale: FontScale,
) -> Option<MenuAction> {
    use egui::containers::menu::{MenuButton, MenuConfig, menu_style};

    // Stay open while dragging the zoom slider; close explicitly on actions.
    // The popup doesn't inherit the parent Ui style, so scale it here too.
    let config = MenuConfig::new()
        .close_behavior(egui::PopupCloseBehavior::CloseOnClickOutside)
        .style(move |style: &mut egui::Style| {
            menu_style(style);
            font_scale.apply(style);
            for font in style.text_styles.values_mut() {
                font.size *= MENU_TEXT_SCALE;
            }
            style.spacing.interact_size.y *= MENU_TEXT_SCALE;
            style.spacing.icon_width *= MENU_TEXT_SCALE;
            style.spacing.icon_width_inner *= MENU_TEXT_SCALE;
            style.spacing.icon_spacing *= MENU_TEXT_SCALE;
        });

    let mut action = None;
    MenuButton::new(egui::RichText::new("☰").text_style(egui::TextStyle::Heading))
        .config(config)
        .ui(ui, |ui| {
            ui.set_min_width(180.0 * MENU_TEXT_SCALE * font_scale.factor());
            if ui.button("Pick symmetry…").clicked() {
                action = Some(MenuAction::PickSymmetry);
                ui.close();
            }
            ui.separator();
            ui.label(egui::RichText::new("Palette").weak());
            for p in Palette::ALL {
                if ui
                    .selectable_label(settings.palette == p, p.label())
                    .clicked()
                {
                    settings.palette = p;
                    ui.close();
                }
            }
            ui.separator();
            ui.checkbox(&mut settings.show_edges, "Show edges");
            ui.checkbox(&mut settings.show_grid, "Show grid");
            ui.separator();
            if ui.button("Reset").clicked() {
                action = Some(MenuAction::Reset);
                ui.close();
            }
            // Hide the tessellated tiles behind the editable tile.
            let mut hide_plane = !settings.show_tiles;
            if ui.checkbox(&mut hide_plane, "Hide plane").changed() {
                settings.show_tiles = !hide_plane;
            }
            // Hide the single editable tile (outline + handles).
            let mut hide_tile = !settings.show_editor;
            if ui.checkbox(&mut hide_tile, "Hide tile").changed() {
                settings.show_editor = !hide_tile;
            }
            ui.separator();
            ui.label(egui::RichText::new("Zoom").weak());
            ui.add(egui::Slider::new(&mut settings.zoom, Settings::ZOOM_RANGE).show_value(false));
        });
    action
}
