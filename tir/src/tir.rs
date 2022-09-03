use pixels::{Error, Pixels, SurfaceTexture};
use std::fs;

use winit::{
    dpi::LogicalSize,
    event::{Event, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

use raqote::*;

use tessellations::render::*;
use tessellations::tessellationfigure::{TessellationFigure, TessellationPlane};
use tessellations::tessellationline::PointIndexPath;

const WIDTH: u32 = 400;
const HEIGHT: u32 = 400;

fn main() -> Result<(), Error> {
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Hello Tessellations")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };
    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };
    let mut f = TessellationFigure::square();
    let p = TessellationPlane {};
    let backend = Box::new(Backend);
    let mut drag: Option<(f32, f32)> = None;
    let m: Transform = Transform::scale(100.0, 100.0).then_translate(euclid::vec2(100.0, 100.0));
    let mi = m.inverse().unwrap();
    let mut selected_point_index: Option<PointIndexPath> = None;

    event_loop.run(move |event, _, control_flow| {
        if let Event::RedrawRequested(_) = event {
            let image = backend.render_plane_to_image(&p, &f, &m).unwrap();
            for (dst, &src) in pixels
                .get_frame()
                .chunks_exact_mut(4)
                .zip(image.get_data().iter())
            {
                dst[0] = (src >> 16) as u8;
                dst[1] = (src >> 8) as u8;
                dst[2] = src as u8;
                dst[3] = (src >> 24) as u8;
            }

            if pixels.render().is_err() {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }
        if input.update(&event) {
            if input.key_pressed(VirtualKeyCode::Escape)
                || input.key_pressed(VirtualKeyCode::Q)
                || input.quit()
            {
                *control_flow = ControlFlow::Exit;
            }
            if input.key_pressed(VirtualKeyCode::S) {
                fs::write(
                    "figure.json",
                    serde_json::to_string(&f).expect("json error").as_bytes(),
                )
                .expect("file error");
            }
            if input.key_pressed(VirtualKeyCode::L) {
                f = serde_json::from_str(
                    fs::read_to_string("figure.json")
                        .expect("file error")
                        .as_str(),
                )
                .expect("json error"); //TODO set matrix
                window.request_redraw();
            }
            if input.key_pressed(VirtualKeyCode::E) {
                let image = backend.render_plane_to_image(&p, &f, &m).unwrap();
                image.save_png(std::path::Path::new("out.png"));
            }
            if input.key_pressed(VirtualKeyCode::Key1) {
                f = TessellationFigure::square();
                window.request_redraw();
            }
            if input.key_pressed(VirtualKeyCode::Key2) {
                f = TessellationFigure::triangle();
                window.request_redraw();
            }
            if input.key_pressed(VirtualKeyCode::Key3) {
                f = TessellationFigure::square90();
                window.request_redraw();
            }
            if input.key_pressed(VirtualKeyCode::Key4) {
                f = TessellationFigure::diamond();
                window.request_redraw();
            }
            if input.key_pressed(VirtualKeyCode::Key5) {
                f = TessellationFigure::brick();
                window.request_redraw();
            }
            if input.key_pressed(VirtualKeyCode::Key6) {
                f = TessellationFigure::hexagon();
                window.request_redraw();
            }

            if input.mouse_held(0) {
                if let Some(mouse) = input.mouse() {
                    let p = mi.transform_point(Point::new(
                        mouse.0 / window.scale_factor() as f32,
                        mouse.1 / window.scale_factor() as f32,
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
                    drag = Some(mouse);
                }
            } else if input.mouse_released(0) {
                selected_point_index = None;
                drag = None;
            }
        }
    });
}
