use tessellations::render::*;
use tessellations::tessellationfigure::{TessellationFigure, FIGURES};
use tessellations::tessellationline::Transform;

fn main() {
    let m: Transform = Transform::scale(100.0, 100.0).then_translate(euclid::vec2(100.0, 100.0));
    let backend = Backend;

    for (info, f) in FIGURES.iter().zip(TessellationFigure::all()) {
        let image = backend.render_tiling_to_image(&f, &m).unwrap();
        image.save_png(std::path::Path::new(&format!("out-{}.png", info.id)));
    }
}
