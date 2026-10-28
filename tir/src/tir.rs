use pixels::{Pixels, SurfaceTexture};
use std::fs;
use std::sync::Arc;

use winit::{
    dpi::LogicalSize,
    event::{Event, MouseButton, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    keyboard::KeyCode,
    window::Window,
};
use winit_input_helper::WinitInputHelper;

use tessellations::render::*;
use tessellations::tessellationfigure::{TessellationFigure, FIGURE_IDS};
use tessellations::tessellationline::{Point, PointIndexPath, Transform};

const WIDTH: u32 = 400;
const HEIGHT: u32 = 400;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let event_loop = EventLoop::new()?;
    let mut input = WinitInputHelper::new();
    let mut figure_index = 0usize;
    let mut f = TessellationFigure::by_index(figure_index);
    let window = Arc::new({
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        event_loop.create_window(
            Window::default_attributes()
                .with_title(TessellationFigure::title(figure_index))
                .with_inner_size(size)
                .with_min_inner_size(size),
        )?
    });
    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture =
            SurfaceTexture::new(window_size.width, window_size.height, window.clone());
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };
    let backend = Box::new(Backend);
    let svgbackend = Box::new(SVGBackend);
    let mut drag: Option<(f32, f32)> = None;
    let m: Transform = Transform::scale(100.0, 100.0).then_translate(euclid::vec2(100.0, 100.0));
    let mi = m.inverse().unwrap();
    let mut selected_point_index: Option<PointIndexPath> = None;

    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop.run(move |event, event_loop| {
        match &event {
            Event::NewEvents(_) => input.step(),
            Event::WindowEvent { event, .. } => {
                if let WindowEvent::Resized(size) = event {
                    if let Err(err) = pixels.resize_surface(size.width, size.height) {
                        eprintln!("Failed to resize surface: {err}");
                        event_loop.exit();
                        return;
                    }
                }
                if input.process_window_event(event) {
                    let image = backend.render_tiling_to_image(&f, &m).unwrap();
                    pixels.frame_mut().copy_from_slice(image.get_data());

                    if pixels.render().is_err() {
                        event_loop.exit();
                    }
                }
            }
            Event::DeviceEvent { event, .. } => input.process_device_event(event),
            Event::AboutToWait => {
                input.end_step();

                if input.key_pressed(KeyCode::Escape)
                    || input.key_pressed(KeyCode::KeyQ)
                    || input.close_requested()
                    || input.destroyed()
                {
                    event_loop.exit();
                    return;
                }
                if input.key_pressed(KeyCode::KeyS) {
                    fs::write(
                        "figure.json",
                        serde_json::to_string(&f).expect("json error").as_bytes(),
                    )
                    .expect("file error");
                    let svg = svgbackend.compose_tiling(&f, &m).unwrap();
                    svg.save_svg(std::path::Path::new("out.svg"));
                }
                if input.key_pressed(KeyCode::KeyL) {
                    f = serde_json::from_str(
                        fs::read_to_string("figure.json")
                            .expect("file error")
                            .as_str(),
                    )
                    .expect("json error");
                    window.request_redraw();
                }
                if input.key_pressed(KeyCode::KeyE) {
                    let image = backend.render_tiling_to_image(&f, &m).unwrap();
                    image.save_png(std::path::Path::new("out.png"));
                }
                if input.key_pressed(KeyCode::KeyN) {
                    figure_index = (figure_index + 1) % FIGURE_IDS.len();
                    f.load(TessellationFigure::by_index(figure_index));
                    window.set_title(&TessellationFigure::title(figure_index));
                    window.request_redraw();
                }
                if input.key_pressed(KeyCode::KeyP) {
                    figure_index = (figure_index + FIGURE_IDS.len() - 1) % FIGURE_IDS.len();
                    f.load(TessellationFigure::by_index(figure_index));
                    window.set_title(&TessellationFigure::title(figure_index));
                    window.request_redraw();
                }

                if input.mouse_held(MouseButton::Left) {
                    if let Some(mouse) = input.cursor() {
                        // `pixels` may letterbox the 400x400 framebuffer when the
                        // window is scaled fractionally (e.g. KDE at 125%). Let it
                        // do the physical-window -> framebuffer conversion so the
                        // cursor lines up with the image.
                        if let Ok((pixel_x, pixel_y)) = pixels.window_pos_to_pixel(mouse) {
                            let p = mi.transform_point(Point::new(
                                pixel_x as f32 + 0.5,
                                pixel_y as f32 + 0.5,
                            ));
                            match drag {
                                Some(d) => {
                                    if d != mouse {
                                        if let Some(h) = selected_point_index {
                                            f.update(h, p);
                                            window.request_redraw();
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
                        }
                        drag = Some(mouse);
                    }
                } else if input.mouse_released(MouseButton::Left) {
                    selected_point_index = None;
                    drag = None;
                }

                window.request_redraw();
            }
            _ => {}
        }
    })?;

    Ok(())
}
