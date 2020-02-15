use tessellation::render::*;
use tessellation::tessellationfigure::TessellationFigure;

fn main() {
    let f = TessellationFigure::square();
    println!("{:?}", f.points());
    let backend = Box::new(Backend);
    let image = backend.render_to_image(f).unwrap();
    image.save_png(std::path::Path::new("out.png"));
}