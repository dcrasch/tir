mod utils;

use std::cell::Cell;
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use wasm_bindgen::JsCast;

use web_sys::console;
use web_sys::{CanvasRenderingContext2d, ImageData};

use raqote::*;
use tessellations::render::*;
use tessellations::tessellationfigure::{TessellationFigure, TessellationPlane};
use tessellations::tessellationline::PointIndexPath;

pub fn draw(
    ctx: &CanvasRenderingContext2d,
    width: u32,
    height: u32,
    f: &TessellationFigure,
) -> Result<(), JsValue> {
    let backend = Box::new(Backend);
    let m: Transform =
        Transform::create_scale(100.0, 100.0).post_translate(euclid::vec2(100.0, 100.0));

    let mut image = backend.render_to_image(&f, &m).unwrap();
    let mut data = image.get_data_u8();

    let data = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut data), width, height)?;
    ctx.put_image_data(&data, 0.0, 0.0)
}

fn app(name: &str) -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let editor = document.get_element_by_id(name);
    let canvas = document
        .create_element("canvas")?
        .dyn_into::<web_sys::HtmlCanvasElement>()?;
    editor.unwrap().append_child(&canvas)?;
    canvas.set_width(400);
    canvas.set_height(400);
    canvas.style().set_property("border", "solid")?;
    let context = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()?;
    console::log_1(&"Draw".into());
    let m1: Transform =
        Transform::create_scale(100.0, 100.0).post_translate(euclid::vec2(100.0, 100.0));
    let mi = m1.inverse().unwrap();
    let mut f2 = TessellationFigure::square();
    let frc = Rc::new(f2);
    let context = Rc::new(context);
    let pressed = Rc::new(Cell::new(false));
    let mut sel_point: Option<PointIndexPath> = None;
    let selected = Rc::new(sel_point);
    draw(&context, 400, 400, &frc);
    {
        let context = context.clone();
        let pressed = pressed.clone();
        let f = frc.clone();
        let selected_point_index = selected.clone();
        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            let p =
                mi.transform_point(Point::new(event.offset_x() as f32, event.offset_y() as f32));

            let s = match f.hitpoints(p, 0.05) {
                Some(h) => Some(h),
                _ => match f.hitline(p, 0.05) {
                    Some(h) => {
                        f.insert(h, p);
                        draw(&context, 400, 400, &f);
                        Some(PointIndexPath {
                            line_index: h.line_index,
                            point_index: h.point_index + 1,
                            corrp: h.corrp,
                        })
                    }
                    _ => None,
                },
            };
	    selected_point_index = Rc::new(s);
            pressed.set(true);
        }) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }
    {
        let context = context.clone();
        let pressed = pressed.clone();
        let mut f = frc.clone();
        let selected_point_index = selected.clone();
        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            if pressed.get() {
                console::log_1(&"Drag".into());
                let p = mi
                    .transform_point(Point::new(event.offset_x() as f32, event.offset_y() as f32));

                if let Some(h) = selected_point_index.as_ref() {
                    f.update(*h, p);
                    draw(&context, 400, 400, &f);
                }
            }
        }) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }
    {
        let context = context.clone();
        let pressed = pressed.clone();
        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            pressed.set(false);
        }) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    Ok(())
}

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    app("editor")
}
