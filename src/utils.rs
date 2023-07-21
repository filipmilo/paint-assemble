use std::{cell::Cell, rc::Rc};

use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::{CanvasRenderingContext2d, Document, Element, HtmlCanvasElement};

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

fn get_canvas() -> Result<HtmlCanvasElement, Element> {
    let document = get_document();

    document
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
}

pub fn get_context() -> Result<CanvasRenderingContext2d, js_sys::Object> {
    let canvas = get_canvas()?;

    canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
}

pub fn create_canvas() -> Result<HtmlCanvasElement, Element> {
    let document = get_document();
    let canvas = document
        .create_element("canvas")?
        .dyn_into::<web_sys::HtmlCanvasElement>()?;
    canvas.set_attribute("id", "canvas")?;
    document.body().unwrap().append_child(&canvas)?;
    canvas.set_width(1500);
    canvas.set_height(1000);
    canvas.style().set_property("border", "solid")?;

    Ok(canvas)
}

pub fn setup_default_stroke(
    context: CanvasRenderingContext2d,
    canvas: HtmlCanvasElement,
) -> Result<(), JsValue> {
    let context = Rc::new(context);
    let pressed = Rc::new(Cell::new(false));
    {
        let context = context.clone();
        let pressed = pressed.clone();
        let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::MouseEvent| {
            context.begin_path();
            context.move_to(event.offset_x() as f64, event.offset_y() as f64);
            pressed.set(true);
        });
        canvas.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }
    {
        let context = context.clone();
        let pressed = pressed.clone();
        let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::MouseEvent| {
            if pressed.get() {
                context.line_to(event.offset_x() as f64, event.offset_y() as f64);
                context.stroke();
                context.begin_path();
                context.move_to(event.offset_x() as f64, event.offset_y() as f64);
            }
        });
        canvas.add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }
    {
        let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::MouseEvent| {
            pressed.set(false);
            context.line_to(event.offset_x() as f64, event.offset_y() as f64);
            context.stroke();
        });
        canvas.add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }
    Ok(())
}
