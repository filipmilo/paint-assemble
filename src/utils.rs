use wasm_bindgen::{JsCast, prelude::Closure};
use web_sys::{Document, Element, HtmlCanvasElement, MouseEvent};

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

pub fn empty_closure() -> Closure<dyn FnMut(MouseEvent)> {
    Closure::<dyn FnMut(_)>::new(|_| {})
}
