use dioxus::prelude::*;
use fermi::*;
use freya::prelude::{elements as dioxus_elements, use_canvas};
use skia_safe::{Color, Paint, PaintStyle, Path, Point};

use crate::components::global_state::DATA;

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

    let canvas = use_canvas(cx, cx.props, |state| {
        let data = use_atom_ref(cx, &DATA);

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
            canvas.translate((
                region.min_x() + state.offset_x,
                region.min_y() + state.offset_x,
            ));
            canvas.scale((state.scale, state.scale));
            let mut paint = Paint::default();
            paint.set_color(Color::WHITE);
            paint.set_anti_alias(true);
            paint.set_stroke_width(1.0 / state.scale);
            paint.set_style(PaintStyle::Stroke);

            let mut path = Path::new();
            path.move_to(points[0]);
            for p in points.iter().skip(1) {
                path.line_to(*p);
            }
            points.iter().skip(1).for_each(|p| {
                path.line_to(*p);
            });
            path.close();
            canvas.draw_path(&path, &paint);

            paint.set_color(Color::GRAY);
            paint.set_style(PaintStyle::Fill);
            canvas.draw_path(&path, &paint);

            canvas.restore();
        })
    });

    let width = &cx.props.width;
    let height = &cx.props.height;

    render!(
        rect {
            width: "{width}",
            height: "{height}",
            //padding: "15 5",
            //background: "white",
            rect {
                canvas_reference: canvas.attribute(cx),
                width: "100%",
                height: "100%",
            }
        }
    )
}
