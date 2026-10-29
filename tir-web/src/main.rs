#![recursion_limit = "1024"]

use console_error_panic_hook::set_once as set_panic_hook;

use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use tessellations::render::*;
use tessellations::tessellationfigure::{figure_groups, TessellationFigure, FIGURES};
use tessellations::tessellationline::{Point, PointIndexPath, Transform};

/// Editor state shared by the event handlers.
struct Scene {
    context: web_sys::HtmlElement,
    figure_select: web_sys::HtmlSelectElement,
    palette_select: web_sys::HtmlSelectElement,
    figure: RefCell<TessellationFigure>,
    figure_index: Cell<usize>,
    palette_index: Cell<usize>,
    selected_point_index: Cell<Option<PointIndexPath>>,
    pressed: Cell<bool>,
}

impl Scene {
    fn palette(&self) -> &'static Palette {
        &PALETTES[self.palette_index.get() % PALETTES.len()]
    }

    fn draw(&self) {
        let m: Transform =
            Transform::scale(100.0, 100.0).then_translate(euclid::vec2(100.0, 100.0));
        let figure = self.figure.borrow();
        let svg_document = SVGBackend
            .compose_tiling_with_palette(&figure, &m, self.palette())
            .unwrap();
        self.context.set_inner_html(&svg_document.get_data());
    }

    /// Load the figure at `index`, redraw it and keep the title and menus in sync.
    fn load_figure(&self, index: usize) {
        let index = index % FIGURES.len();
        self.figure_index.set(index);
        self.figure
            .borrow_mut()
            .load(TessellationFigure::by_index(index));
        if let Some(document) = web_sys::window().and_then(|w| w.document()) {
            document.set_title(&TessellationFigure::title(index));
        }
        self.figure_select.set_value(&index.to_string());
        self.draw();
    }

    /// Select the palette at `index` and redraw the current figure with it.
    fn set_palette(&self, index: usize) {
        let index = index % PALETTES.len();
        self.palette_index.set(index);
        self.palette_select.set_value(&index.to_string());
        self.draw();
    }
}

/// Fill a `<select>` with one `<option>` per label, valued by position.
fn fill_select(
    document: &web_sys::Document,
    select: &web_sys::HtmlSelectElement,
    labels: impl Iterator<Item = String>,
) -> Result<(), JsValue> {
    for (index, label) in labels.enumerate() {
        let option = document
            .create_element("option")?
            .dyn_into::<web_sys::HtmlOptionElement>()?;
        option.set_value(&index.to_string());
        option.set_text(&label);
        select.append_child(&option)?;
    }
    Ok(())
}

/// Fill the tile menu with one `<optgroup>` per shape, ordered by IH number.
///
/// The option values are figure indices, so the shape headings are only a
/// label: each variant still loads its own figure. Each option spells out the
/// full name and IH number, because a closed `<select>` shows only the option
/// text and never the optgroup heading.
fn fill_tile_select(
    document: &web_sys::Document,
    select: &web_sys::HtmlSelectElement,
) -> Result<(), JsValue> {
    for (shape, members) in figure_groups() {
        let group = document
            .create_element("optgroup")?
            .dyn_into::<web_sys::HtmlOptGroupElement>()?;
        group.set_label(shape);
        for index in members {
            let info = &FIGURES[index];
            let option = document
                .create_element("option")?
                .dyn_into::<web_sys::HtmlOptionElement>()?;
            option.set_value(&index.to_string());
            option.set_text(&format!(
                "{} — {}, IH {}, {}",
                info.name(),
                info.heesch,
                info.ih,
                info.group
            ));
            group.append_child(&option)?;
        }
        select.append_child(&group)?;
    }
    Ok(())
}

fn app(name: &str) -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let editor = document
        .get_element_by_id(name)
        .unwrap()
        .dyn_into::<web_sys::HtmlElement>()?;
    let figure_select = document
        .get_element_by_id("figure-select")
        .unwrap()
        .dyn_into::<web_sys::HtmlSelectElement>()?;
    let palette_select = document
        .get_element_by_id("color-select")
        .unwrap()
        .dyn_into::<web_sys::HtmlSelectElement>()?;

    fill_tile_select(&document, &figure_select)?;
    fill_select(
        &document,
        &palette_select,
        PALETTES.iter().map(|palette| palette.name.to_string()),
    )?;

    let context = document
        .create_element("div")
        .unwrap()
        .dyn_into::<web_sys::HtmlElement>()?;
    let _ = editor.append_child(&context);

    let scene = Rc::new(Scene {
        context,
        figure_select,
        palette_select,
        figure: RefCell::new(TessellationFigure::by_index(0)),
        figure_index: Cell::new(0),
        palette_index: Cell::new(0),
        selected_point_index: Cell::new(None),
        pressed: Cell::new(false),
    });

    scene.set_palette(0);
    scene.load_figure(0);

    let m1: Transform = Transform::scale(100.0, 100.0).then_translate(euclid::vec2(100.0, 100.0));
    let mi = m1.inverse().unwrap();

    {
        let target = scene.figure_select.clone();
        let scene = scene.clone();

        let closure = Closure::wrap(Box::new(move |_event: web_sys::Event| {
            if let Ok(i) = scene.figure_select.value().parse::<usize>() {
                scene.load_figure(i);
            }
        }) as Box<dyn FnMut(_)>);
        target.add_event_listener_with_callback("change", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    {
        let target = scene.palette_select.clone();
        let scene = scene.clone();

        let closure = Closure::wrap(Box::new(move |_event: web_sys::Event| {
            if let Ok(i) = scene.palette_select.value().parse::<usize>() {
                scene.set_palette(i);
            }
        }) as Box<dyn FnMut(_)>);
        target.add_event_listener_with_callback("change", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    {
        let scene = scene.clone();

        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            let p =
                mi.transform_point(Point::new(event.offset_x() as f32, event.offset_y() as f32));
            let (selected, inserted) = {
                let mut f = scene.figure.borrow_mut();
                match f.hitpoints(p, 0.05) {
                    Some(h) => (Some(h), false),
                    _ => match f.hitline(p, 0.05) {
                        Some(h) => {
                            f.insert(h, p);
                            (
                                Some(PointIndexPath {
                                    line_index: h.line_index,
                                    point_index: h.point_index + 1,
                                    corrp: h.corrp,
                                }),
                                true,
                            )
                        }
                        _ => (None, false),
                    },
                }
            };
            scene.selected_point_index.set(selected);
            scene.pressed.set(true);
            if inserted {
                scene.draw();
            }
        }) as Box<dyn FnMut(_)>);
        editor.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    {
        let scene = scene.clone();

        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            let Some(h) = scene.selected_point_index.get() else {
                return;
            };
            if !scene.pressed.get() {
                return;
            }

            let p =
                mi.transform_point(Point::new(event.offset_x() as f32, event.offset_y() as f32));
            scene.figure.borrow_mut().update(h, p);
            scene.draw();
        }) as Box<dyn FnMut(_)>);
        editor.add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    {
        let scene = scene.clone();

        let closure = Closure::wrap(Box::new(move |_event: web_sys::MouseEvent| {
            scene.pressed.set(false);
        }) as Box<dyn FnMut(_)>);
        editor.add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    {
        let scene = scene.clone();

        let closure = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
            let i = scene.figure_index.get();
            let i = match event.key().as_str() {
                "n" => (i + 1) % FIGURES.len(),
                "p" => (i + FIGURES.len() - 1) % FIGURES.len(),
                "c" => {
                    scene.set_palette(scene.palette_index.get() + 1);
                    return;
                }
                _ => return,
            };
            scene.load_figure(i);
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
