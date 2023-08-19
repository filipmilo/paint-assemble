use std::rc::Rc;
use wasm_bindgen::{prelude::wasm_bindgen, Clamped, JsCast, JsValue};
use web_sys::{CanvasRenderingContext2d, Document, Element, HtmlCanvasElement, ImageData};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
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

pub fn fill(
    ctx: Rc<CanvasRenderingContext2d>,
    x: usize,
    y: usize,
    width: u32,
    height: u32,
) -> Result<(), JsValue> {
    let image = ctx.get_image_data(0.0, 0.0, width as f64, height as f64)?;
    let mut data = image.data();

    let replace_color: (u8, u8, u8, u8) = (0, 0, 0, 0);
    let replacement_color: (u8, u8, u8, u8) = (255, 0, 0, 255);

    let index = y * (width as usize * 4) + x * 4;
    let node = (
        data[index],
        data[index + 1],
        data[index + 2],
        data[index + 3],
    );

    if node != replace_color {
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
