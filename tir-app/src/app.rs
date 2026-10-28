//! Main application state and eframe::App implementation.

use eframe::egui::{self, Color32, Sense, Vec2};
use tessellations::tessellationline::Point;

use crate::figure::{FigureKind, Settings, View};
use crate::input::handle_input;
use crate::render::{paint_editor, paint_tiles};
use crate::ui::MenuAction;

/// Android system font size setting, bucketed like the Settings app does.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FontScale {
    Small,
    Default,
    Large,
    Largest,
    /// Accessibility / Extra Large, keeps the raw `fontScale`.
    ExtraLarge(f32),
}

impl FontScale {
    /// Bucket `Configuration.fontScale`.
    pub fn from_raw(font_scale: f32) -> Self {
        match font_scale {
            s if s <= 0.85 => Self::Small,
            s if s <= 1.0 => Self::Default,
            s if s <= 1.15 => Self::Large,
            s if s <= 1.30 => Self::Largest,
            s => Self::ExtraLarge(s),
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Small => "Small",
            Self::Default => "Default",
            Self::Large => "Large",
            Self::Largest => "Largest",
            Self::ExtraLarge(_) => "Accessibility / Extra Large",
        }
    }

    /// Multiplier applied to text and widget sizes.
    pub fn factor(self) -> f32 {
        match self {
            Self::Small => 0.85,
            Self::Default => 1.0,
            Self::Large => 1.15,
            Self::Largest => 1.30,
            Self::ExtraLarge(s) => s.clamp(1.30, 2.0),
        }
    }

    /// Scale text, spacing and widget sizes by `factor()`.
    pub fn apply(self, style: &mut egui::Style) {
        let f = self.factor();
        for font in style.text_styles.values_mut() {
            font.size *= f;
        }
        let s = &mut style.spacing;
        s.interact_size *= f;
        s.button_padding *= f;
        s.item_spacing *= f;
        s.menu_margin *= f;
        s.slider_width *= f;
        s.icon_width *= f;
        s.icon_width_inner *= f;
        s.icon_spacing *= f;
    }
}

pub struct TirApp {
    pub kind: FigureKind,
    pub figure: tessellations::tessellationfigure::TessellationFigure,
    pub selected: Option<tessellations::tessellationline::PointIndexPath>,
    /// Center of the untouched figure, so the view doesn't jump while editing.
    pub home_center: Point,
    /// View options from the menu (zoom, palette, visibility toggles).
    pub settings: Settings,
    /// Last known Android system-bar/cutout insets (points).
    #[cfg(target_os = "android")]
    pub safe_area: egui::SafeAreaInsets,
    /// Window size the insets were queried for; re-query when it changes.
    #[cfg(target_os = "android")]
    pub insets_for_size: Option<Vec2>,
    /// System font size bucket; scales the hamburger and its menu.
    pub font_scale: FontScale,
}

impl Default for TirApp {
    fn default() -> Self {
        let mut app = Self {
            kind: FigureKind::Square,
            figure: FigureKind::Square.build(),
            selected: None,
            home_center: Point::new(0.5, 0.5),
            settings: Settings::default(),
            #[cfg(target_os = "android")]
            safe_area: Default::default(),
            #[cfg(target_os = "android")]
            insets_for_size: None,
            font_scale: FontScale::Default,
        };
        app.reset();
        app
    }
}

impl TirApp {
    /// Load the pristine version of the current figure kind and recenter.
    pub fn reset(&mut self) {
        self.figure = self.kind.build();
        self.selected = None;
        let pts = self.figure.points();
        let (mut min, mut max) = (pts[0], pts[0]);
        for p in &pts {
            min = min.min(*p);
            max = max.max(*p);
        }
        self.home_center = min.lerp(max, 0.5);
    }

    /// Act on a menu choice that changes the figure.
    pub fn apply(&mut self, action: MenuAction) {
        match action {
            MenuAction::SelectFigure(kind) => self.kind = kind,
            MenuAction::Reset => {}
        }
        self.reset();
    }

    /// Unique polygon points of the figure (crate emits shared endpoints twice).
    pub fn polygon(&self) -> Vec<Point> {
        FigureKind::polygon(&self.figure)
    }

    /// Bend both edges like a user would: press on the edge, drag.
    #[cfg(feature = "screenshot")]
    pub fn demo_edit(&mut self) {
        let mids: Vec<(Point, Point)> = self
            .figure
            .lines
            .iter()
            .map(|l| {
                let p = l.points();
                let m = p[0].lerp(p[1], 0.5);
                let c = self.home_center;
                (m, m.lerp(c, 0.35))
            })
            .collect();
        for (on_edge, to) in mids {
            if let Some(h) = self.figure.hitline(on_edge, 0.05) {
                self.figure.insert(h, on_edge);
                let sel = tessellations::tessellationline::PointIndexPath {
                    point_index: h.point_index + 1,
                    ..h
                };
                self.figure.update(sel, to);
            }
        }
    }
}

impl eframe::App for TirApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // The root `Ui` covers the whole viewport (not `content_rect()`), so the
        // panels must keep clear of the status bar / cutout / nav bar
        // themselves. Insets come from `raw_input_hook` (zero on desktop).
        let safe = ui.ctx().input(|i| i.safe_area_insets()).0;
        let px = |v: f32| v.ceil().clamp(0.0, 127.0) as i8;

        // Transparent top bar: the canvas fills the whole screen and the bar is
        // overlaid on top of it (drawn afterwards, on a higher layer).
        let screen = ui.max_rect();
        let mut top_margin = egui::Frame::side_top_panel(ui.style()).inner_margin;
        top_margin.top += px(safe.top);
        top_margin.left += px(safe.left);
        top_margin.right += px(safe.right);

        egui::CentralPanel::default()
            .frame(
                egui::Frame::NONE
                    .fill(Color32::WHITE)
                    .inner_margin(egui::Margin {
                        left: px(safe.left),
                        right: px(safe.right),
                        top: 0,
                        // Bottom inset ignored: let the canvas run under the nav bar.
                        // bottom: px(safe.bottom),
                        bottom: 0,
                    }),
            )
            .show(ui, |ui| {
                let (response, painter) =
                    ui.allocate_painter(ui.available_size(), Sense::click_and_drag());
                let rect = response.rect;

                let scale = self.settings.zoom;
                let origin =
                    rect.center() - Vec2::new(self.home_center.x, self.home_center.y) * scale;
                let view = View { scale, origin };

                // Touch needs a bigger hit area than a mouse.
                let tol_px = if cfg!(target_os = "android") {
                    24.0
                } else {
                    10.0
                };
                // No editing while the tile (and its handles) is hidden.
                if self.settings.show_editor {
                    self.selected =
                        handle_input(&mut self.figure, ui, &response, view, tol_px, self.selected);
                } else {
                    self.selected = None;
                }

                if self.settings.show_tiles {
                    paint_tiles(&painter, &self.figure, view, rect, &self.settings);
                }
                if self.settings.show_editor {
                    paint_editor(&painter, &self.figure, self.selected, view);
                }
            });

        // `Order::Middle`: above the canvas (background), below menu popups.
        let toolbar_layer = egui::LayerId::new(egui::Order::Middle, egui::Id::new("toolbar"));
        ui.scope_builder(
            egui::UiBuilder::new()
                .layer_id(toolbar_layer)
                .max_rect(screen),
            |ui| {
                egui::Frame::NONE.inner_margin(top_margin).show(ui, |ui| {
                    // Figure name on the left, hamburger on the far right.
                    let mut action = None;
                    ui.horizontal(|ui| {
                        self.font_scale.apply(ui.style_mut());
                        ui.heading(self.kind.label());
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            action =
                                crate::ui::menu(ui, self.kind, &mut self.settings, self.font_scale);
                        });
                    });
                    if let Some(action) = action {
                        self.apply(action);
                    }
                });
            },
        );

        #[cfg(feature = "screenshot")]
        crate::utilities::screenshot::handle(ui.ctx());
    }

    /// egui-winit doesn't report safe-area insets on Android, so query them via
    /// `WindowInsets.getInsets()` and hand them to egui, which then keeps the
    /// UI inside `ctx.content_rect()`. Re-queried when the window size changes
    /// (rotation) and until the first successful query.
    #[cfg(target_os = "android")]
    fn raw_input_hook(&mut self, ctx: &egui::Context, raw_input: &mut egui::RawInput) {
        let size = raw_input.screen_rect.map(|r| r.size());
        if (self.insets_for_size.is_none() || self.insets_for_size != size)
            && let Some(app) = crate::platform::android::ANDROID_APP.get()
            && let Some(px) = crate::platform::android::android_safe_area(app)
        {
            let ppp = ctx.pixels_per_point();
            self.safe_area = egui::SafeAreaInsets(egui::epaint::MarginF32 {
                left: px[0] as f32 / ppp,
                top: px[1] as f32 / ppp,
                right: px[2] as f32 / ppp,
                bottom: px[3] as f32 / ppp,
            });
            self.insets_for_size = size;
            log::info!("safe area insets (points): {:?}", self.safe_area);
        }
        raw_input.safe_area_insets = Some(self.safe_area);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::figure::View;
    use eframe::egui::Pos2;
    use tessellations::tessellationline::{Point, PointIndexPath};

    #[test]
    fn press_on_edge_inserts_point_and_drag_moves_both_sides() {
        let mut app = TirApp::default();
        let v = View {
            scale: 100.0,
            origin: Pos2::new(50.0, 50.0),
        };
        // Left edge of the square, halfway: figure (0, 0.5).
        let press = v.to_screen(Point::new(0.0, 0.5));
        let tol = 10.0 / v.scale;
        let p = v.to_figure(press);
        assert!(app.figure.hitpoints(p, tol).is_none());
        let h = app.figure.hitline(p, tol).expect("edge hit");
        app.figure.insert(h, p);
        let sel = PointIndexPath {
            point_index: h.point_index + 1,
            ..h
        };

        // Drag inward.
        app.figure.update(sel, Point::new(0.2, 0.5));
        let poly = app.polygon();
        assert!(poly.contains(&Point::new(0.2, 0.5)), "{poly:?}");
        // The opposite (right) edge follows via the line's translation.
        assert!(poly.contains(&Point::new(1.2, 0.5)), "{poly:?}");

        // The moved point is now grabbable as a handle.
        assert_eq!(app.figure.hitpoints(Point::new(0.2, 0.5), tol), Some(sel));
        // ...and from the linked point on the opposite edge too.
        let mirrored = app.figure.hitpoints(Point::new(1.2, 0.5), tol).unwrap();
        assert_eq!(mirrored.line_index, 1, "{mirrored:?}");
        assert!(!mirrored.corrp, "{mirrored:?}");

        // Concave shape still triangulates.
        assert!(
            crate::figure::FigureKind::triangulate(&app.figure)
                .unwrap()
                .indices
                .len()
                >= 3 * 4
        );
    }

    #[test]
    fn font_scale_buckets() {
        let label = |s| FontScale::from_raw(s).label();
        assert_eq!(label(0.85), "Small");
        assert_eq!(label(0.9), "Default");
        assert_eq!(label(1.0), "Default");
        assert_eq!(label(1.15), "Large");
        assert_eq!(label(1.3), "Largest");
        assert_eq!(label(1.5), "Accessibility / Extra Large");
        assert_eq!(FontScale::from_raw(1.0).factor(), 1.0);
        assert_eq!(FontScale::from_raw(1.8).factor(), 1.8);
        assert_eq!(FontScale::from_raw(5.0).factor(), 2.0);
    }

    /// No overlaps or gaps: every sampled point of the plane lies in exactly
    /// one placement.
    #[test]
    fn tiles_exactly_cover_the_plane() {
        /// Even-odd rule.
        fn inside(poly: &[Point], p: Point) -> bool {
            let mut c = false;
            let n = poly.len();
            for i in 0..n {
                let (a, b) = (poly[i], poly[(i + n - 1) % n]);
                if (a.y > p.y) != (b.y > p.y) && p.x < (b.x - a.x) * (p.y - a.y) / (b.y - a.y) + a.x
                {
                    c = !c;
                }
            }
            c
        }

        for kind in crate::figure::FigureKind::ALL {
            let mut app = TirApp {
                kind,
                ..TirApp::default()
            };
            app.reset();
            let poly = app.polygon();
            let tiles: Vec<Vec<Point>> = app
                .figure
                .placements(8.0, 6.0)
                .iter()
                .map(|pl| {
                    poly.iter()
                        .map(|&p| pl.transform.transform_point(p))
                        .collect()
                })
                .collect();

            // Irregular steps so samples don't land exactly on tile edges.
            for iy in 0..21 {
                for ix in 0..21 {
                    let p = Point::new(
                        -4.0 + 8.0 * (ix as f32 + 0.37) / 21.0,
                        -3.0 + 6.0 * (iy as f32 + 0.61) / 21.0,
                    );
                    let hits = tiles.iter().filter(|t| inside(t, p)).count();
                    assert_eq!(hits, 1, "{kind:?}: {p:?} covered {hits} times");
                }
            }
        }
    }

    #[test]
    fn every_figure_builds_triangulates_and_is_editable() {
        for kind in crate::figure::FigureKind::ALL {
            let mut app = TirApp {
                kind,
                ..TirApp::default()
            };
            app.reset();
            let poly = app.polygon();
            assert!(poly.len() >= 3, "{kind:?}: {poly:?}");
            let tri = crate::figure::FigureKind::triangulate(&app.figure)
                .unwrap_or_else(|| panic!("{kind:?} triangulate"));
            assert!(!tri.indices.is_empty(), "{kind:?}");

            // Press on the middle of the first line -> inserts a point we can drag.
            let pts = app.figure.lines[0].points();
            let mid = pts[0].lerp(pts[1], 0.5);
            let h = app
                .figure
                .hitline(mid, 0.05)
                .unwrap_or_else(|| panic!("{kind:?} hitline"));
            app.figure.insert(h, mid);
            let sel = PointIndexPath {
                point_index: h.point_index + 1,
                ..h
            };
            let to = mid.lerp(app.home_center, 0.3);
            app.figure.update(sel, to);
            assert!(
                app.polygon().iter().any(|p| p.distance_to(to) < 1e-4),
                "{kind:?}"
            );
            assert!(
                crate::figure::FigureKind::triangulate(&app.figure).is_some(),
                "{kind:?} after edit"
            );
        }
    }

    #[test]
    fn reset_action_discards_edits() {
        let mut app = TirApp::default();
        let pristine = app.polygon();
        let h = app.figure.hitline(Point::new(0.0, 0.5), 0.05).unwrap();
        app.figure.insert(h, Point::new(0.0, 0.5));
        app.selected = Some(h);
        assert_ne!(app.polygon(), pristine);

        app.apply(MenuAction::Reset);
        assert_eq!(app.polygon(), pristine);
        assert_eq!(app.selected, None);
        assert_eq!(app.kind, FigureKind::Square);
    }

    #[test]
    fn select_figure_action_switches_and_recenters() {
        let mut app = TirApp::default();
        app.settings.zoom = 222.0;
        for kind in FigureKind::ALL {
            app.apply(MenuAction::SelectFigure(kind));
            assert_eq!(app.kind, kind);
            assert_eq!(app.polygon(), FigureKind::polygon(&kind.build()));
            // home_center is the bounding-box center of the figure.
            let pts = app.figure.points();
            let inside = |v: f32, f: fn(&Point) -> f32| {
                let lo = pts.iter().map(f).fold(f32::INFINITY, f32::min);
                let hi = pts.iter().map(f).fold(f32::NEG_INFINITY, f32::max);
                (v - (lo + hi) / 2.0).abs() < 1e-5
            };
            assert!(inside(app.home_center.x, |p| p.x), "{kind:?}");
            assert!(inside(app.home_center.y, |p| p.y), "{kind:?}");
        }
        // View settings survive figure changes.
        assert_eq!(app.settings.zoom, 222.0);
    }

    #[test]
    fn font_scale_apply_scales_text() {
        let base = egui::Style::default();
        for (raw, f) in [(0.85, 0.85), (1.0, 1.0), (1.3, 1.3), (1.6, 1.6)] {
            let mut style = base.clone();
            FontScale::from_raw(raw).apply(&mut style);
            for (ts, font) in &style.text_styles {
                let want = base.text_styles[ts].size * f;
                assert!((font.size - want).abs() < 1e-4, "{raw}: {ts:?}");
            }
            assert!(
                (style.spacing.interact_size.y - base.spacing.interact_size.y * f).abs() < 1e-4
            );
        }
    }

    #[test]
    fn font_scale_bucket_boundaries() {
        assert_eq!(FontScale::from_raw(0.5), FontScale::Small);
        assert_eq!(FontScale::from_raw(0.86), FontScale::Default);
        assert_eq!(FontScale::from_raw(1.01), FontScale::Large);
        assert_eq!(FontScale::from_raw(1.16), FontScale::Largest);
        assert_eq!(FontScale::from_raw(1.31), FontScale::ExtraLarge(1.31));
        // Extra large never shrinks below "Largest".
        assert!(FontScale::ExtraLarge(1.0).factor() >= FontScale::Largest.factor());
    }

    /// Runs whole frames headlessly: the app must not panic for any figure or
    /// option combination.
    #[test]
    fn app_runs_headless_frames() {
        use eframe::App as _;
        let ctx = egui::Context::default();
        let mut app = TirApp::default();
        let input = || egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(800.0, 600.0),
            )),
            ..Default::default()
        };
        for kind in FigureKind::ALL {
            app.apply(MenuAction::SelectFigure(kind));
            for (tiles, editor) in [(true, true), (false, true), (true, false)] {
                app.settings.show_tiles = tiles;
                app.settings.show_editor = editor;
                let mut out = ctx.run_ui(input(), |ui| {
                    let mut frame = eframe::Frame::_new_kittest();
                    app.ui(ui, &mut frame);
                });
                // No renderer here: drop the font atlas upload explicitly.
                out.textures_delta.clear();
            }
        }
    }
}
