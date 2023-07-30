mod utils;

use std::{cell::Cell, rc::Rc};

use crate::utils::get_client_canvas;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

#[wasm_bindgen]
pub struct Canvas {
    underlying_layer: HtmlCanvasElement,
}

#[wasm_bindgen]
impl Canvas {
    pub fn new_canvas(height: u32, width: u32) -> Result<Canvas, JsValue> {
        let client_canvas = get_client_canvas()?;
        let canvas = Canvas {
            underlying_layer: client_canvas,
        };

        canvas.underlying_layer.set_height(height);
        canvas.underlying_layer.set_width(width);
        canvas.get_context()?.set_line_cap("round");
        canvas.setup_default_stroke()?;

        Ok(canvas)
    }

    pub fn set_stroke_width(&self, width: f64) -> Result<(), JsValue> {
        let context = self.get_context()?;
        context.set_line_width(width);

        Ok(())
    }

    pub fn set_stroke_color(&self, color: String) -> Result<(), JsValue> {
        let context = self.get_context()?;
        context.set_stroke_style(&JsValue::from_str(&color));

        Ok(())
    }
}

impl Canvas {
    fn get_context(&self) -> Result<CanvasRenderingContext2d, js_sys::Object> {
        self.underlying_layer
            .get_context("2d")?
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
    }

    fn setup_default_stroke(&self) -> Result<(), JsValue> {
        let context = Rc::new(self.get_context()?);
        let pressed = Rc::new(Cell::new(false));
        {
            let context = context.clone();
            let pressed = pressed.clone();
            let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::MouseEvent| {
                context.begin_path();
                context.move_to(event.offset_x() as f64, event.offset_y() as f64);
                pressed.set(true);
            });
            self.underlying_layer
                .add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())?;
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
            self.underlying_layer
                .add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }
        {
            let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::MouseEvent| {
                pressed.set(false);
                context.line_to(event.offset_x() as f64, event.offset_y() as f64);
                context.stroke();
            });
            self.underlying_layer
                .add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }
        Ok(())
    }
}
