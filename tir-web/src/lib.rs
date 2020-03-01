mod utils;

use wasm_bindgen::prelude::*;

use tessellations::render::*;
use tessellations::tessellationfigure::{TessellationFigure, TessellationPlane};
use tessellations::tessellationline::PointIndexPath;
use raqote::*;


use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d, ImageData};

#[wasm_bindgen]
pub fn draw(
    ctx: &CanvasRenderingContext2d,
    width: u32,
    height: u32,
) -> Result<(), JsValue> {

    let mut f = TessellationFigure::square();
    let p = TessellationPlane {};
    let backend = Box::new(Backend);
    let m: Transform =
        Transform::create_scale(100.0, 100.0).post_translate(euclid::vec2(100.0, 100.0));
 
    let mut image = backend.render_to_image(&f, &m).unwrap();
    let mut data =  image.get_data_u8();

    let data = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut data), width, height)?;
    ctx.put_image_data(&data, 0.0, 0.0)
}