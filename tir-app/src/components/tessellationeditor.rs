use dioxus::prelude::*;
use euclid::Point2D;
use fermi::*;
use freya::events::MouseEvent;
use freya::prelude::{Props, elements as dioxus_elements, use_canvas, Canvas};
use skia_safe::{Color, Paint, PaintStyle, Path, Point};

use crate::components::global_state::DATA;

use tessellations::tessellationline::PointIndexPath;

#[derive(Debug, Props, PartialEq, Clone)]
pub struct TessellationEditorProps {
    //editing: bool,
    offset_x: f32,
    offset_y: f32,
    scale: f32,

    #[props(default = "100%".to_string(), into)]
    width: String,
    /// Height of the Graph. Default 100%.
    #[props(default = "100%".to_string(), into)]
    height: String,
}

#[allow(non_snake_case)]
pub fn TessellationEditor(cx: Scope<TessellationEditorProps>) -> Element {
    use_init_atom_root(cx);

    let changed = use_state::<bool>(cx, || true);
    let clicking_drag = use_state::<Option<(PointIndexPath, (f32, f32))>>(cx, || None);

    let canvas = use_canvas(cx, changed, |changed| {
        let data = use_atom_ref(cx, &DATA);
        let offset_x = cx.props.offset_x;
        let offset_y = cx.props.offset_y;
        let scale = cx.props.scale;

        let points = data
            .read()
            .points()
            .windows(2)
            .filter_map(|l| {
                if l[0] != l[1] {
                    Some(Point {
                        x: l[0].x,
                        y: l[0].y,
                    })
                } else {
                    None
                }
            })
            .collect::<Vec<Point>>();
        Box::new(move |canvas, _font_collection, region| {
            canvas.translate((region.min_x() + offset_x, region.min_y() + offset_y));
            canvas.scale((scale, scale));
            let mut paint = Paint::default();
            paint.set_color(Color::WHITE);
            paint.set_anti_alias(true);
            paint.set_stroke_width(1.0 / scale);
            paint.set_style(PaintStyle::Stroke);

            let mut path = Path::new();
            path.move_to(points[0]);
            points.iter().skip(1).for_each(|p| {
                path.line_to(*p);
            });
            path.close();
            canvas.draw_path(&path, &paint);

            paint.set_color(Color::GRAY);
            paint.set_style(PaintStyle::Stroke);
            canvas.draw_path(&path, &paint);

            canvas.restore();
        })
    });

    let onmousedown = |e: MouseEvent| {
        let data = use_atom_ref(cx, &DATA);

        let (x, y) = e.get_element_coordinates().to_tuple();
        let x = x as f32;
        let y = y as f32;
        let fx = (x - cx.props.offset_x) / cx.props.scale;
        let fy = (y - cx.props.offset_y) / cx.props.scale;
        let p = Point2D::new(fx, fy);
        let mut f = data.write();

        let s = match f.hitpoints(p, 0.05) {
            Some(h) => Some((h, (fx, fy))),
            _ => match f.hitline(p, 0.05) {
                Some(h) => {
                    f.insert(h, p);
                    let pi = PointIndexPath {
                        line_index: h.line_index,
                        point_index: h.point_index + 1,
                        corrp: h.corrp,
                    };
                    Some((pi, (fx, fy)))
                }
                _ => None,
            },
        };
        clicking_drag.set(s);
    };

    let onclick = |_: MouseEvent| {
        clicking_drag.set(None);
    };

    let onmouseover = |e: MouseEvent| {
        if let Some((selected_point_index, (oldx, oldy))) = clicking_drag.get() {
            let data = use_atom_ref(cx, &DATA);

            let (x, y) = e.get_element_coordinates().to_tuple();
            let x = x as f32;
            let y = y as f32;
            let fx = (x - cx.props.offset_x) / cx.props.scale;
            let fy = (y - cx.props.offset_y) / cx.props.scale;
            let p = Point2D::new(fx, fy);

            if (oldx-x).abs()>0.000001 && (oldy-y).abs()>0.000001 {
                let mut f = data.write();
                f.update(*selected_point_index, p);
                changed.set(true);
                clicking_drag.set(Some((*selected_point_index,(x,y))));

            }
        }
    };

    let width = &cx.props.width;
    let height = &cx.props.height;

    render!(
        rect {
            width: "{width}",
            height: "{height}",
            onmousedown: onmousedown,
            onclick: onclick,
            onmouseover: onmouseover,
            Canvas {
                canvas: canvas,
                width: "100%",
                height: "100%",

            }
        }
    )
}
