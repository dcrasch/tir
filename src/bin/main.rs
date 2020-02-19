use minifb::{Key, MouseButton, MouseMode, Window, WindowOptions};
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
    let mut f = TessellationFigure::square();
    let backend = Box::new(Backend);
    let mut drag: Option<(f32, f32)> = None;
    let m: Transform = Transform::create_scale(100.0, 100.0)
        .post_translate(euclid::vec2(10.0, 10.0))
        .inverse()
        .unwrap();
    let mut selectedPointIndex: Option<PointIndexPath> = None;

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
    while window.is_open() && !window.is_key_down(Key::Escape) {
        let image = backend.render_to_image(&f).unwrap();
        window
            .update_with_buffer(image.get_data(), size.0, size.1)
            .unwrap();

        window.get_mouse_pos(MouseMode::Discard).map(|mouse| {
            if window.get_mouse_down(MouseButton::Left) {
                let p = m.transform_point(Point::new(mouse.0, mouse.1));
                match drag {
                    Some(d) => {
                        if d != mouse {
                            // do drag point
                            println!("drag {:?}", mouse);
                            println!("{:?}", p);
                        }
                    }
                    _ => {
                        // do click point
                        println!("click {:?}", mouse);
                        println!("p {:?}", p);
                        match f.hitline(p, 0.05) {
                            Some(h) => {
                                println!("hit {:?}", h);
                                f.insert(h, p);
                                selectedPointIndex = Some(h);
                            }
                            _ => (),
                        }
                        println!("f= {:?}", f);
                    }
                }
                drag = Some(mouse);
            } else {
                drag = None;
            }
        });
    }
}
