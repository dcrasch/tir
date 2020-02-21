use minifb::{Key, KeyRepeat, MouseButton, MouseMode, Window, WindowOptions};
use raqote::*;

use tessellation::render::*;
use tessellation::tessellationfigure::TessellationFigure;
use tessellation::tessellationline::PointIndexPath;

const WIDTH: usize = 400;
const HEIGHT: usize = 400;

fn main() {
    let mut window = Window::new(
        "Tessellation",
        WIDTH,
        HEIGHT,
        WindowOptions {
            ..WindowOptions::default()
        },
    )
    .unwrap();
    let size = window.get_size();
    let mut f = TessellationFigure::diamond();
    let backend = Box::new(Backend);
    let mut drag: Option<(f32, f32)> = None;
    let m: Transform =
        Transform::create_scale(100.0, 100.0).post_translate(euclid::vec2(100.0, 100.0));
    let mi = m.inverse().unwrap();
    let mut selected_point_index: Option<PointIndexPath> = None;

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
    while window.is_open() && !window.is_key_down(Key::Escape) {
        let image = backend.render_to_image(&f, &m).unwrap();
        window
            .update_with_buffer(image.get_data(), size.0, size.1)
            .unwrap();
        if window.is_key_pressed(Key::S, KeyRepeat::No) {
            println!("save");
            println!("{:?}", f)
        }
        if window.is_key_pressed(Key::L, KeyRepeat::No) {
            println!("load")
        }

        if window.is_key_pressed(Key::Key1, KeyRepeat::No) {
            selected_point_index = None;
            f = TessellationFigure::square();
        }

        if window.is_key_pressed(Key::Key2, KeyRepeat::No) {
            selected_point_index = None;
            f = TessellationFigure::triangle();
        }

        if window.is_key_pressed(Key::Key3, KeyRepeat::No) {
            selected_point_index = None;
            f = TessellationFigure::square90();
        }

        if window.is_key_pressed(Key::Key3, KeyRepeat::No) {
            selected_point_index = None;
            f = TessellationFigure::diamond();
        }

        if let Some(mouse) = window.get_mouse_pos(MouseMode::Discard) {
            if window.get_mouse_down(MouseButton::Left) {
                let p = mi.transform_point(Point::new(mouse.0, mouse.1));
                match drag {
                    Some(d) => {
                        if d != mouse {
                            if let Some(h) = selected_point_index {
                                f.update(h, p)
                            }
                        }
                    }
                    _ => match f.hitpoints(p, 0.05) {
                        Some(h) => selected_point_index = Some(h),
                        _ => match f.hitline(p, 0.05) {
                            Some(h) => {
                                f.insert(h, p);
                                selected_point_index = Some(PointIndexPath {
                                    line_index: h.line_index,
                                    point_index: h.point_index + 1,
                                    corrp: h.corrp,
                                });
                            }
                            _ => selected_point_index = None,
                        },
                    },
                }
                drag = Some(mouse);
            } else {
                selected_point_index = None;
                drag = None;
            }
        }
    }
}
