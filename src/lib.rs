mod utils;

use utils::{create_canvas, get_context, setup_default_stroke};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
fn start() -> Result<(), JsValue> {
    let canvas = create_canvas()?;
    let context = get_context()?;

    context.set_line_cap("round");

    setup_default_stroke(context, canvas)
}

#[wasm_bindgen]
pub fn set_stroke_width(width: f64) -> Result<(), JsValue> {
    let context = get_context()?;
    context.set_line_width(width);

    Ok(())
}
