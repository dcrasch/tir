//! User input handling and hit testing.

use eframe::egui;
use tessellations::tessellationfigure::TessellationFigure;
use tessellations::tessellationline::PointIndexPath;

use crate::figure::View;

/// Press: hit-test handles first, then edges (inserting a new point).
/// Move: drag the selected point. Release: deselect.
pub fn handle_input(
    figure: &mut TessellationFigure,
    ui: &egui::Ui,
    response: &egui::Response,
    view: View,
    tol_px: f32,
    current_selected: Option<PointIndexPath>,
) -> Option<PointIndexPath> {
    let tol = tol_px / view.scale;
    let (pressed, released, origin) = ui.input(|i| {
        (
            i.pointer.primary_pressed(),
            i.pointer.primary_released(),
            i.pointer.press_origin(),
        )
    });

    let mut selected = current_selected;

    if pressed
        && response.contains_pointer()
        && let Some(pos) = origin
    {
        let p = view.to_figure(pos);
        selected = match figure.hitpoints(p, tol) {
            Some(h) => Some(h),
            None => figure.hitline(p, tol).map(|h| {
                figure.insert(h, p);
                PointIndexPath {
                    point_index: h.point_index + 1,
                    ..h
                }
            }),
        };
    }

    if let Some(sel) = selected
        && response.is_pointer_button_down_on()
        && let Some(pos) = response.interact_pointer_pos()
    {
        figure.update(sel, view.to_figure(pos));
    }

    // Return the selected point if still dragging, or None if released
    if released { None } else { selected }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::figure::FigureKind;
    use eframe::egui::{Event, PointerButton, Pos2, RawInput, Rect, Sense, vec2};
    use tessellations::tessellationline::Point;

    const VIEW: View = View {
        scale: 100.0,
        origin: Pos2::new(50.0, 50.0),
    };

    /// Drives `handle_input` with a sequence of frames, each a list of events.
    struct Harness {
        ctx: egui::Context,
        figure: TessellationFigure,
        selected: Option<PointIndexPath>,
    }

    impl Harness {
        fn new() -> Self {
            Self {
                ctx: egui::Context::default(),
                figure: FigureKind::Square.build(),
                selected: None,
            }
        }

        fn frame(&mut self, events: Vec<Event>) {
            let input = RawInput {
                screen_rect: Some(Rect::from_min_size(Pos2::ZERO, vec2(400.0, 400.0))),
                events,
                ..Default::default()
            };
            let mut out = self.ctx.run_ui(input, |ui| {
                let (response, _) =
                    ui.allocate_painter(ui.available_size(), Sense::click_and_drag());
                self.selected =
                    handle_input(&mut self.figure, ui, &response, VIEW, 10.0, self.selected);
            });
            out.textures_delta.clear();
        }

        fn press(&mut self, p: Point) {
            let pos = VIEW.to_screen(p);
            self.frame(vec![
                Event::PointerMoved(pos),
                Event::PointerButton {
                    pos,
                    button: PointerButton::Primary,
                    pressed: true,
                    modifiers: Default::default(),
                },
            ]);
        }

        fn drag_to(&mut self, p: Point) {
            self.frame(vec![Event::PointerMoved(VIEW.to_screen(p))]);
        }

        fn release(&mut self, p: Point) {
            let pos = VIEW.to_screen(p);
            self.frame(vec![Event::PointerButton {
                pos,
                button: PointerButton::Primary,
                pressed: false,
                modifiers: Default::default(),
            }]);
        }

        fn polygon(&self) -> Vec<Point> {
            FigureKind::polygon(&self.figure)
        }

        fn has(&self, p: Point) -> bool {
            self.polygon().iter().any(|q| (*q - p).length() < 1e-3)
        }
    }

    #[test]
    fn press_drag_release_on_edge() {
        let mut h = Harness::new();
        h.frame(vec![]);
        let n = h.polygon().len();

        h.press(Point::new(0.0, 0.5));
        assert!(h.selected.is_some(), "edge press selects a new point");
        assert_eq!(h.polygon().len(), n + 2, "point inserted on both sides");

        h.drag_to(Point::new(0.2, 0.5));
        h.drag_to(Point::new(0.25, 0.4));
        assert!(h.has(Point::new(0.25, 0.4)), "{:?}", h.polygon());
        assert!(h.has(Point::new(1.25, 0.4)), "opposite edge follows");

        h.release(Point::new(0.25, 0.4));
        assert_eq!(h.selected, None);

        // Moving the pointer after release must not edit anything.
        let before = h.polygon();
        h.drag_to(Point::new(0.5, 0.5));
        assert_eq!(h.polygon(), before);
    }

    #[test]
    fn press_on_existing_handle_regrabs_it() {
        let mut h = Harness::new();
        h.frame(vec![]);
        h.press(Point::new(0.0, 0.5));
        h.drag_to(Point::new(0.2, 0.5));
        h.release(Point::new(0.2, 0.5));
        let n = h.polygon().len();

        // Grab the mirrored copy and move it: no new point, both sides move.
        h.press(Point::new(1.2, 0.5));
        assert!(h.selected.is_some_and(|s| s.corrp));
        h.drag_to(Point::new(1.1, 0.6));
        h.release(Point::new(1.1, 0.6));
        assert_eq!(h.polygon().len(), n);
        assert!(h.has(Point::new(1.1, 0.6)), "{:?}", h.polygon());
        assert!(h.has(Point::new(0.1, 0.6)), "{:?}", h.polygon());
    }

    #[test]
    fn press_in_empty_space_does_nothing() {
        let mut h = Harness::new();
        h.frame(vec![]);
        let before = h.polygon();
        h.press(Point::new(0.5, 0.5)); // center of the square, far from edges
        assert_eq!(h.selected, None);
        h.drag_to(Point::new(0.7, 0.7));
        h.release(Point::new(0.7, 0.7));
        assert_eq!(h.polygon(), before);
    }

    #[test]
    fn corners_are_not_draggable() {
        let mut h = Harness::new();
        h.frame(vec![]);
        let before = h.polygon();
        h.press(Point::new(0.0, 0.0));
        h.drag_to(Point::new(0.3, 0.3));
        h.release(Point::new(0.3, 0.3));
        assert!(
            h.has(Point::new(0.0, 0.0)),
            "corner stays put: {:?}",
            h.polygon()
        );
        // A press near a corner may insert a point on an edge, but the
        // original corners all remain.
        for p in before {
            assert!(h.has(p), "{p:?} missing from {:?}", h.polygon());
        }
    }
}
