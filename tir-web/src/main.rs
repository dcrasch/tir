#![recursion_limit = "1024"]

use console_error_panic_hook::set_once as set_panic_hook;

use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use wasm_bindgen::JsCast;

use raqote::*;
use tessellations::render::*;
use tessellations::tessellationfigure::{TessellationFigure, TessellationPlane};
use tessellations::tessellationline::PointIndexPath;

pub fn draw(
    ctx: &web_sys::HtmlElement,
    _: u32,
    _: u32,
    f: &TessellationFigure,
) -> Result<(), JsValue> {
    let backend = Box::new(SVGBackend);
    let m: Transform = Transform::scale(100.0, 100.0).then_translate(euclid::vec2(100.0, 100.0));
    let p = TessellationPlane {};
    let svg_document = backend.compose_plane(&p, f, &m).unwrap();
    Ok(ctx.set_inner_html(&svg_document.get_data()))
}

fn app(name: &str) -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let body = document.body().expect("Could not access document.body");
    let editor = document
        .get_element_by_id(name)
        .unwrap()
        .dyn_into::<web_sys::HtmlElement>()?;
    let context = document
        .create_element("div")
        .unwrap()
        .dyn_into::<web_sys::HtmlElement>()?;
    editor.append_child(&context);

    let m1: Transform = Transform::scale(100.0, 100.0).then_translate(euclid::vec2(100.0, 100.0));
    let mi = m1.inverse().unwrap();
    let figure: Rc<RefCell<TessellationFigure>> =
        Rc::new(RefCell::new(TessellationFigure::triangle()));
    let selected_point_index: Rc<Cell<Option<PointIndexPath>>> = Rc::new(Cell::new(None));

    let context = Rc::new(context);
    let pressed = Rc::new(Cell::new(false));

    draw(&context, 400, 400, &figure.borrow_mut())?;

    {
        let context = context.clone();
        let pressed = pressed.clone();
        let figure_cloned = figure.clone();
        let selected_point_index_cloned = selected_point_index.clone();

        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            let p =
                mi.transform_point(Point::new(event.offset_x() as f32, event.offset_y() as f32));
            let mut f = figure_cloned.borrow_mut();
            let s = match f.hitpoints(p, 0.05) {
                Some(h) => Some(h),
                _ => match f.hitline(p, 0.05) {
                    Some(h) => {
                        f.insert(h, p);
                        draw(&context, 400, 400, &f).unwrap();
                        Some(PointIndexPath {
                            line_index: h.line_index,
                            point_index: h.point_index + 1,
                            corrp: h.corrp,
                        })
                    }
                    _ => None,
                },
            };
            selected_point_index_cloned.set(s);
            pressed.set(true);
        }) as Box<dyn FnMut(_)>);
        editor.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    {
        let context = context.clone();
        let pressed = pressed.clone();
        let figure_cloned = figure.clone();
        let selected_point_index_cloned = selected_point_index;

        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            let mut f = figure_cloned.borrow_mut();

            if pressed.get() {
                let p = mi
                    .transform_point(Point::new(event.offset_x() as f32, event.offset_y() as f32));

                if let Some(h) = selected_point_index_cloned.get() {
                    f.update(h, p);
                    draw(&context, 400, 400, &f).unwrap();
                }
            }
        }) as Box<dyn FnMut(_)>);
        editor.add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    {
        let pressed = pressed;
        let closure = Closure::wrap(Box::new(move |_event: web_sys::MouseEvent| {
            pressed.set(false);
        }) as Box<dyn FnMut(_)>);
        editor.add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    {
        let context = context.clone();
        let figure_cloned = figure.clone();
        let closure = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
            let mut f = figure_cloned.borrow_mut();
            if event.key() == "1" {
                f.load(TessellationFigure::brick());
            } else if event.key() == "2" {
                f.load(TessellationFigure::triangle());
            }
            draw(&context, 400, 400, &f).unwrap();
        }) as Box<dyn FnMut(_)>);
        editor.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    Ok(())
}

fn main() {
    set_panic_hook();
    app("editor").expect("Failed to start editor");
}
