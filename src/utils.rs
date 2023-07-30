use wasm_bindgen::JsCast;
use web_sys::{Document, Element, HtmlCanvasElement};

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

fn get_document() -> Document {
    web_sys::window().unwrap().document().unwrap()
}

pub fn get_client_canvas() -> Result<HtmlCanvasElement, Element> {
    let document = get_document();

    document
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
}
