mod utils;

use utils::{create_canvas, get_context, setup_default_stroke};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn new_canvas(height: u32, width: u32) -> Result<(), JsValue> {
    let canvas = create_canvas(height, width)?;
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

#[wasm_bindgen]
pub fn set_stroke_color(color: String) -> Result<(), JsValue> {
    let context = get_context()?;
    context.set_stroke_style(&JsValue::from_str(&color));

    Ok(())
}

