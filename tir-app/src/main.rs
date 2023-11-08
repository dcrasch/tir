#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]


use freya::{common::EventMessage, prelude::*};
use skia_safe::{Color, Paint, Point, Path, PaintStyle};

use tessellations::render::*;
use tessellations::tessellationfigure::{TessellationFigure, TessellationPlane};
//use tessellations::tessellationline::PointIndexPath;
use fermi::*;

static DATA: AtomRef<TessellationFigure> = AtomRef(|_|TessellationFigure::triangle());

fn main() {
    launch(app);
}

fn app(cx: Scope) -> Element {
    use_init_atom_root(cx);    
    let backend = Box::new(Backend);
    let platform = use_platform(cx);
    let mut state = use_state(cx, || 0);
   
    
    use_effect(cx, (state,), move |_| async move {
        platform.send(EventMessage::RequestRerender).unwrap();
    });

    let canvas = use_canvas(cx, state, |state| {
	// mmmmm
	 let data = use_atom_ref(cx, &DATA);
	let points = data
	    .read()
        .points()
        .windows(2)
        .filter_map(|l| {
            if l[0] != l[1] {
                Some(Point {x:l[0].x, y:l[0].y})
            } else {
                None
            }
        })
            .collect::<Vec<Point>>();
	
	let state = *state.current();
        Box::new(move |canvas, _, region| {
            canvas.translate((region.min_x()+100.0, region.min_y()+100.0));
	    canvas.scale((100.0,100.0));
	    let mut paint = Paint::default();
            paint.set_color(Color::WHITE);
            paint.set_anti_alias(true);
            paint.set_stroke_width(1.0);
	    paint.set_style(PaintStyle::Stroke);
	    
	    let mut path = Path::new();
	    path.move_to(points[0]);
	    for p in points.iter().skip(1) {
		path.line_to(*p);
	    }
	    path.close();	   	  
	    canvas.draw_path(&path, &paint);

	    paint.set_color(Color::GRAY);
	    paint.set_style(PaintStyle::Fill);
	    canvas.draw_path(&path, &paint);
	    
            canvas.restore();
        })
    });

    render!(
        rect {
            onclick: move |_| {
                state += 1;
            },
            Canvas {
                canvas: canvas,
                background: "black",
                width: "100%",
                height: "100%"
            }
        }
    )
}
