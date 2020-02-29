mod utils;

use wasm_bindgen::prelude::*;
use tessellations::tessellationfigure::{TessellationFigure};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    let f = TessellationFigure::square();
    alert(&serde_json::to_string(&f).expect("json error"));
}
