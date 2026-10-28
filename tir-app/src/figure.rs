//! Domain types for tessellation figures.

use eframe::egui::{Color32, Pos2, Vec2};
use tessellations::tessellationfigure::TessellationFigure;
use tessellations::tessellationline::Point;

/// Color palette for rendering tessellations.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Palette {
    Original,
    Grayscale,
}

impl Palette {
    pub const ALL: [Self; 2] = [Self::Original, Self::Grayscale];

    pub fn label(self) -> &'static str {
        match self {
            Self::Original => "Original",
            Self::Grayscale => "Grayscale",
        }
    }

    /// Get the 4 colors for this palette.
    pub fn colors(self) -> [Color32; 4] {
        match self {
            Self::Original => [
                Color32::from_rgb(0xf6, 0x88, 0xbb),
                Color32::from_rgb(0xe8, 0xf9, 0xe9),
                Color32::from_rgb(0xba, 0xfa, 0xa1),
                Color32::from_rgb(0x9d, 0xe3, 0xd0),
            ],
            Self::Grayscale => [
                Color32::from_rgb(0x20, 0x20, 0x20),
                Color32::from_rgb(0x60, 0x60, 0x60),
                Color32::from_rgb(0xa0, 0xa0, 0xa0),
                Color32::from_rgb(0xe0, 0xe0, 0xe0),
            ],
        }
    }
}

/// User-adjustable view options (everything in the menu except the figure).
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Settings {
    /// Tile size in screen points.
    pub zoom: f32,
    /// Draw the tessellated plane behind the editable tile.
    pub show_tiles: bool,
    /// Draw (and allow editing of) the single editable tile.
    pub show_editor: bool,
    /// Show tile edge outlines.
    pub show_edges: bool,
    /// Show grid lines for the original tile layout.
    pub show_grid: bool,
    /// Color palette for rendering tiles.
    pub palette: Palette,
}

impl Settings {
    pub const ZOOM_RANGE: std::ops::RangeInclusive<f32> = 40.0..=400.0;
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            zoom: if cfg!(target_os = "android") {
                140.0
            } else {
                120.0
            },
            show_tiles: true,
            show_editor: true,
            show_edges: true,
            show_grid: false,
            palette: Palette::Original,
        }
    }
}

pub const OUTLINE: Color32 = Color32::from_rgb(0x00, 0x88, 0x00);
pub const HANDLE_RADIUS_PX: f32 = 6.0;

/// Figure (model) space <-> screen space mapping.
#[derive(Clone, Copy)]
pub struct View {
    pub scale: f32,
    /// screen position of figure-space (0, 0)
    pub origin: Pos2,
}

impl View {
    pub fn to_screen(self, p: Point) -> Pos2 {
        self.origin + Vec2::new(p.x, p.y) * self.scale
    }

    pub fn to_figure(self, p: Pos2) -> Point {
        let v = (p - self.origin) / self.scale;
        Point::new(v.x, v.y)
    }
}

/// All base figures provided by the `tessellations` crate.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FigureKind {
    Square,
    Triangle,
    Square90,
    Diamond,
    Brick,
    Hexagon,
}

impl FigureKind {
    pub const ALL: [Self; 6] = [
        Self::Square,
        Self::Triangle,
        Self::Square90,
        Self::Diamond,
        Self::Brick,
        Self::Hexagon,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Self::Square => "Square",
            Self::Triangle => "Triangle",
            Self::Square90 => "Square 90°",
            Self::Diamond => "Diamond",
            Self::Brick => "Brick",
            Self::Hexagon => "Hexagon",
        }
    }

    pub fn build(self) -> TessellationFigure {
        match self {
            Self::Square => TessellationFigure::square(),
            Self::Triangle => TessellationFigure::triangle(),
            Self::Square90 => TessellationFigure::square90(),
            Self::Diamond => TessellationFigure::diamond(),
            Self::Brick => TessellationFigure::brick(),
            Self::Hexagon => TessellationFigure::hexagon(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn view() -> View {
        View {
            scale: 100.0,
            origin: Pos2::new(50.0, 50.0),
        }
    }

    #[test]
    fn view_roundtrip() {
        let v = view();
        let p = Point::new(1.5, 2.3);
        let screen = v.to_screen(p);
        let back = v.to_figure(screen);
        assert!((p.x - back.x).abs() < 1e-5);
        assert!((p.y - back.y).abs() < 1e-5);
    }

    #[test]
    fn palette_colors_count() {
        for palette in Palette::ALL {
            let colors = palette.colors();
            assert_eq!(colors.len(), 4, "{palette:?} should have 4 colors");
        }
    }

    #[test]
    fn palette_labels_exist() {
        for palette in Palette::ALL {
            let label = palette.label();
            assert!(!label.is_empty(), "{palette:?} should have a label");
        }
    }

    #[test]
    fn view_maps_origin_and_scales() {
        let v = view();
        assert_eq!(v.to_screen(Point::new(0.0, 0.0)), Pos2::new(50.0, 50.0));
        assert_eq!(v.to_screen(Point::new(1.0, -0.5)), Pos2::new(150.0, 0.0));
        let p = v.to_figure(Pos2::new(250.0, 150.0));
        assert_eq!((p.x, p.y), (2.0, 1.0));
    }

    #[test]
    fn palettes_are_distinct() {
        for p in Palette::ALL {
            let c = p.colors();
            for i in 0..c.len() {
                for j in i + 1..c.len() {
                    assert_ne!(c[i], c[j], "{p:?}: slots {i} and {j}");
                }
            }
        }
        assert_ne!(Palette::Original.colors(), Palette::Grayscale.colors());
    }

    #[test]
    fn grayscale_is_gray() {
        for c in Palette::Grayscale.colors() {
            assert!(c.r() == c.g() && c.g() == c.b(), "{c:?}");
        }
    }

    #[test]
    fn figure_kinds_unique_labels() {
        let labels: std::collections::HashSet<_> =
            FigureKind::ALL.iter().map(|k| k.label()).collect();
        assert_eq!(labels.len(), FigureKind::ALL.len());
    }

    #[test]
    fn default_settings() {
        let s = Settings::default();
        assert!(s.show_tiles && s.show_editor && s.show_edges && !s.show_grid);
        assert_eq!(s.palette, Palette::Original);
        assert!(Settings::ZOOM_RANGE.contains(&s.zoom));
    }
}
