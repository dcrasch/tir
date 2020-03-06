use raqote::*;

use tessellations::render::*;
use tessellations::tessellationfigure::{TessellationFigure, TessellationPlane};

fn main() {
    let f = TessellationFigure::square();
    let p = TessellationPlane {};
    let m: Transform =
        Transform::create_scale(100.0, 100.0).post_translate(euclid::vec2(100.0, 100.0));
    let backend = Box::new(Backend);
    let image = backend.render_plane_to_image(&p, &f, &m).unwrap();
    image.save_png(std::path::Path::new("out.png"));
}
