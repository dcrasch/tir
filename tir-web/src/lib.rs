mod utils;

use std::cell::Cell;
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use wasm_bindgen::JsCast;

use web_sys::{CanvasRenderingContext2d, ImageData};

use raqote::*;
use tessellations::render::*;
use tessellations::tessellationfigure::{TessellationFigure, TessellationPlane};
use tessellations::tessellationline::PointIndexPath;

#[wasm_bindgen]
pub fn draw(ctx: &CanvasRenderingContext2d, width: u32, height: u32) -> Result<(), JsValue> {
    let mut f = TessellationFigure::hexagon();
    let p = TessellationPlane {};
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
    let context = Rc::new(context);
    let pressed = Rc::new(Cell::new(false));
    {
        let context = context.clone();
        let pressed = pressed.clone();
        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            context.begin_path();
            context.move_to(event.offset_x() as f64, event.offset_y() as f64);
            pressed.set(true);
        }) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }
    {
        let context = context.clone();
        let pressed = pressed.clone();
        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            if pressed.get() {
                context.line_to(event.offset_x() as f64, event.offset_y() as f64);
                context.stroke();
                context.begin_path();
                context.move_to(event.offset_x() as f64, event.offset_y() as f64);
		draw(&context,400,400);
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
            context.line_to(event.offset_x() as f64, event.offset_y() as f64);
            context.stroke();
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
