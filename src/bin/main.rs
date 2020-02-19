use tessellation::render::*;
use tessellation::tessellationfigure::TessellationFigure;

use minifb::{MouseMode, MouseButton, Window, WindowOptions, Key};
const WIDTH: usize = 400;
const HEIGHT: usize = 400;

fn main() {
    let mut window = Window::new("Tessellation", WIDTH, HEIGHT, WindowOptions {
                                    ..WindowOptions::default()
                                }).unwrap();
    let size = window.get_size();
    let mut f = TessellationFigure::square();
    let backend = Box::new(Backend);
    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let image = backend.render_to_image(&f).unwrap();
        window.update_with_buffer(image.get_data(), size.0, size.1).unwrap();

        window.get_mouse_pos(MouseMode::Discard).map(|mouse| {
            if window.get_mouse_down(MouseButton::Left) {
                println!("left {:?}", mouse);
            }
            if window.get_mouse_down(MouseButton::Right) {
                println!("right {:?}", mouse);

            }
        });
    }
}