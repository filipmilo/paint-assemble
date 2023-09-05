use regex::Regex;
use std::rc::Rc;
use wasm_bindgen::{prelude::wasm_bindgen, Clamped, JsCast, JsValue};
use web_sys::{CanvasRenderingContext2d, Document, Element, HtmlCanvasElement, ImageData};

use crate::Color;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

pub fn _set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

pub fn get_document() -> Document {
    web_sys::window().unwrap().document().unwrap()
}

pub fn get_client_canvas() -> Result<HtmlCanvasElement, Element> {
    let document = get_document();

    document
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
}

pub fn two_point_distance(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()
}

pub fn get_content_inside_rect(
    ctx: Rc<CanvasRenderingContext2d>,
    x: f64,
    y: f64,
    w: f64,
    h: f64,
) -> Result<ImageData, JsValue> {
    ctx.get_image_data(x, y, w, h)
}

pub fn fill(
    ctx: Rc<CanvasRenderingContext2d>,
    x: usize,
    y: usize,
    width: u32,
    height: u32,
    color: &Color,
) -> Result<(), JsValue> {
    let image = ctx.get_image_data(0.0, 0.0, width as f64, height as f64)?;
    let mut data = image.data();

    let replacement_color = color.value();

    let index = y * (width as usize * 4) + x * 4;
    let replace_color = (
        data[index],
        data[index + 1],
        data[index + 2],
        data[index + 3],
    );

    if replacement_color == replace_color {
        return Ok(());
    }

    let mut stack: Vec<usize> = vec![index];

    while !stack.is_empty() {
        let node = stack.pop().unwrap();

        if node > (width * height * 4 - 4) as usize
            || (data[node], data[node + 1], data[node + 2], data[node + 3]) != replace_color
        {
            continue;
        }

        data[node] = replacement_color.0;
        data[node + 1] = replacement_color.1;
        data[node + 2] = replacement_color.2;
        data[node + 3] = replacement_color.3;

        stack.push(node + 4);
        stack.push(node - 4);
        stack.push(node - width as usize * 4);
        stack.push(node + width as usize * 4);
    }

    ctx.put_image_data(
        &ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut data.0[..]), width, height)?,
        0.0,
        0.0,
    )
}

pub fn define_postition(line_start: f64, offset: f64) -> f64 {
    if line_start < offset {
        line_start
    } else {
        offset
    }
}

pub fn define_distance(line_start: f64, offset: f64) -> f64 {
    (line_start - offset).abs()
}

pub fn match_input(input: &str) -> bool {
    Regex::new(r"^[a-zA-z0-9 ]$").unwrap().is_match(input)
}
