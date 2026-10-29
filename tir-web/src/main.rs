#![recursion_limit = "1024"]

use console_error_panic_hook::set_once as set_panic_hook;

use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use tessellations::render::*;
use tessellations::tessellationfigure::{
    figure_categories, TessellationFigure, FIGURES, FIGURE_IDS,
};
use tessellations::tessellationline::{Point, PointIndexPath, Transform};

/// Side of the SVG viewBox, in user units.
const VIEW_SIZE: f32 = 400.0;
/// ViewBox units per figure unit.
const VIEW_SCALE: f32 = 100.0;

/// Transform that maps figure coordinates into the SVG viewBox, centred on the
/// figure's bounding box so the editable tile sits in the middle of the drawing.
fn view_transform(figure: &TessellationFigure) -> Transform {
    let points = figure.points();
    let (mut min, mut max) = (Point::new(0.0, 0.0), Point::new(0.0, 0.0));
    if let Some(&first) = points.first() {
        (min, max) = (first, first);
        for p in &points {
            min = min.min(*p);
            max = max.max(*p);
        }
    }
    let center = min.lerp(max, 0.5);
    let half = VIEW_SIZE / 2.0;
    Transform::scale(VIEW_SCALE, VIEW_SCALE).then_translate(euclid::vec2(
        half - VIEW_SCALE * center.x,
        half - VIEW_SCALE * center.y,
    ))
}

/// Editor state shared by the event handlers.
struct Scene {
    context: web_sys::HtmlElement,
    palette_select: web_sys::HtmlSelectElement,
    /// The gallery landing page and the editor page, toggled by the route.
    gallery: web_sys::HtmlElement,
    editor_page: web_sys::HtmlElement,
    figure: RefCell<TessellationFigure>,
    figure_index: Cell<usize>,
    palette_index: Cell<usize>,
    selected_point_index: Cell<Option<PointIndexPath>>,
    pressed: Cell<bool>,
    /// Maps figure coordinates into the SVG viewBox, centred on the loaded
    /// figure's bounding box. Frozen when the figure loads, so editing a tile
    /// doesn't shift the view under the pointer.
    view: Cell<Transform>,
    /// Inverse of `view`, for turning mouse positions into figure coordinates.
    view_inverse: Cell<Transform>,
}

impl Scene {
    fn palette(&self) -> &'static Palette {
        &PALETTES[self.palette_index.get() % PALETTES.len()]
    }

    fn draw(&self) {
        let m = self.view.get();
        let figure = self.figure.borrow();
        let svg_document = SVGBackend
            .compose_tiling_named(&figure, &m, self.palette(), "editor-figure")
            .unwrap();
        self.context.set_inner_html(&svg_document.get_data());
    }

    /// Load the figure at `index`, redraw it and keep the title in sync.
    fn load_figure(&self, index: usize) {
        let index = index % FIGURES.len();
        self.figure_index.set(index);
        let figure = TessellationFigure::by_index(index);
        // Centre the view on this figure and freeze it, so later edits to the
        // tile don't move the view.
        let m = view_transform(&figure);
        self.view.set(m);
        self.view_inverse
            .set(m.inverse().expect("view transform is invertible"));
        self.figure.borrow_mut().load(figure);
        if let Some(document) = web_sys::window().and_then(|w| w.document()) {
            document.set_title(&TessellationFigure::title(index));
        }
        self.draw();
    }

    /// Select the palette at `index` and redraw the current figure with it.
    fn set_palette(&self, index: usize) {
        let index = index % PALETTES.len();
        self.palette_index.set(index);
        self.palette_select.set_value(&index.to_string());
        self.draw();
    }

    /// Show the gallery or the editor, depending on the URL hash.
    ///
    /// `#/edit/<id>` opens the editor on that figure; any other hash (including
    /// none) shows the gallery. Called at start-up and on every `hashchange`,
    /// so the browser back and forward buttons move between the two pages.
    fn route(&self) {
        let hash = web_sys::window()
            .and_then(|w| w.location().hash().ok())
            .unwrap_or_default();
        match hash
            .strip_prefix("#/edit/")
            .and_then(TessellationFigure::id_index)
        {
            Some(index) => {
                self.gallery.set_hidden(true);
                self.editor_page.set_hidden(false);
                self.load_figure(index);
            }
            None => {
                self.editor_page.set_hidden(true);
                self.gallery.set_hidden(false);
                if let Some(document) = web_sys::window().and_then(|w| w.document()) {
                    document.set_title("Tessellations");
                }
            }
        }
    }

    /// Navigate to the editor route for the figure at `index`.
    fn open_figure(&self, index: usize) {
        let id = FIGURE_IDS[index % FIGURE_IDS.len()];
        self.set_hash(&format!("#/edit/{id}"));
    }

    /// True while the editor (rather than the gallery) is on screen.
    fn editing(&self) -> bool {
        !self.editor_page.hidden()
    }

    fn set_hash(&self, hash: &str) {
        if let Some(window) = web_sys::window() {
            let _ = window.location().set_hash(hash);
        }
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

/// Build the gallery: one section per symmetry category, each holding a card
/// with a thumbnail of the tiling that links to the editor for that figure.
fn build_gallery(
    document: &web_sys::Document,
    gallery: &web_sys::HtmlElement,
) -> Result<(), JsValue> {
    for (category, members) in figure_categories() {
        let section = document.create_element("section")?;
        section.set_class_name("category");

        let heading = document.create_element("h2")?;
        heading.set_text_content(Some(category));
        section.append_child(&heading)?;

        let grid = document.create_element("div")?;
        grid.set_class_name("grid");
        for index in members {
            let info = &FIGURES[index];
            let figure = TessellationFigure::by_index(index);
            let m = view_transform(&figure);
            let id = format!("figure-{}", info.id);
            let Some(svg) = SVGBackend.compose_tiling_named(&figure, &m, &DEFAULT_PALETTE, &id)
            else {
                continue;
            };
            let card = document.create_element("a")?;
            card.set_class_name("card");
            card.set_attribute("href", &format!("#/edit/{}", info.id))?;
            card.set_attribute("title", &info.name())?;
            card.set_inner_html(&format!(
                "{}\n<div class=\"caption\">{}<div class=\"meta\">{} · IH {} · {}</div></div>",
                svg.get_data(),
                info.name(),
                info.heesch,
                info.ih,
                info.group
            ));
            grid.append_child(&card)?;
        }
        section.append_child(&grid)?;
        gallery.append_child(&section)?;
    }
    Ok(())
}

fn app(name: &str) -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let editor = document
        .get_element_by_id(name)
        .unwrap()
        .dyn_into::<web_sys::HtmlElement>()?;
    let palette_select = document
        .get_element_by_id("color-select")
        .unwrap()
        .dyn_into::<web_sys::HtmlSelectElement>()?;
    let gallery = document
        .get_element_by_id("gallery")
        .unwrap()
        .dyn_into::<web_sys::HtmlElement>()?;
    let editor_page = document
        .get_element_by_id("editor-page")
        .unwrap()
        .dyn_into::<web_sys::HtmlElement>()?;

    fill_select(
        &document,
        &palette_select,
        PALETTES.iter().map(|palette| palette.name.to_string()),
    )?;
    build_gallery(&document, &gallery)?;

    let context = document
        .create_element("div")
        .unwrap()
        .dyn_into::<web_sys::HtmlElement>()?;
    let _ = editor.append_child(&context);

    let scene = Rc::new(Scene {
        context,
        palette_select,
        gallery,
        editor_page,
        figure: RefCell::new(TessellationFigure::by_index(0)),
        figure_index: Cell::new(0),
        palette_index: Cell::new(0),
        selected_point_index: Cell::new(None),
        pressed: Cell::new(false),
        view: Cell::new(Transform::identity()),
        view_inverse: Cell::new(Transform::identity()),
    });

    scene.set_palette(0);
    scene.load_figure(0);

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
            let p = scene
                .view_inverse
                .get()
                .transform_point(Point::new(event.offset_x() as f32, event.offset_y() as f32));
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

            let p = scene
                .view_inverse
                .get()
                .transform_point(Point::new(event.offset_x() as f32, event.offset_y() as f32));
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
            if !scene.editing() {
                return;
            }
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
            scene.open_figure(i);
        }) as Box<dyn FnMut(_)>);
        document.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    {
        let scene = scene.clone();

        let closure = Closure::wrap(Box::new(move |_event: web_sys::Event| {
            scene.route();
        }) as Box<dyn FnMut(_)>);
        web_sys::window()
            .unwrap()
            .add_event_listener_with_callback("hashchange", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    scene.route();

    Ok(())
}

fn main() {
    set_panic_hook();
    app("editor").expect("Failed to start editor");
}
