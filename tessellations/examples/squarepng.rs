use tessellations::tessellationline::Transform;

use tessellations::render::*;
use tessellations::tessellationfigure::TessellationFigure;

fn main() {
    let f = TessellationFigure::square();
    let m: Transform = Transform::scale(100.0, 100.0).then_translate(euclid::vec2(100.0, 100.0));

    let backend = Box::new(Backend);
    let image = backend.render_tiling_to_image(&f, &m).unwrap();
    image.save_png(std::path::Path::new("out.png"));

    let svgbackend = Box::new(SVGBackend);
    let svg = svgbackend.compose_tiling(&f, &m).unwrap();
    svg.save_svg(std::path::Path::new("plane.svg"));
}
