mod utils;

use std::{cell::Cell, f64::consts::PI, rc::Rc};

use crate::utils::{get_client_canvas, get_document};
use utils::two_point_distance;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

#[wasm_bindgen]
pub struct Canvas {
    underlying_layer: HtmlCanvasElement,
    top_layer: HtmlCanvasElement,
    height: u32,
    width: u32,
}

#[wasm_bindgen]
impl Canvas {
    pub fn new_canvas(height: u32, width: u32) -> Result<Canvas, JsValue> {
        let document = get_document();
        let top_canvas = document
            .create_element("canvas")?
            .dyn_into::<web_sys::HtmlCanvasElement>()?;

        let client_canvas = get_client_canvas()?;
        let canvas = Canvas {
            underlying_layer: client_canvas,
            top_layer: top_canvas,
            height,
            width,
        };

        canvas.underlying_layer.set_height(height);
        canvas.underlying_layer.set_width(width);
        canvas.top_layer.set_height(height);
        canvas.top_layer.set_width(width);

        let paint_div = document
            .get_element_by_id("paint-assemble")
            .unwrap()
            .dyn_into::<web_sys::HtmlDivElement>()?;

        let _ = paint_div.append_child(&canvas.top_layer);

        canvas.get_context()?.set_line_cap("round");
        canvas.get_top_context()?.set_line_cap("round");
        canvas.setup_circle()?;

        Ok(canvas)
    }

    pub fn set_stroke_width(&self, width: f64) -> Result<(), JsValue> {
        let context = self.get_context()?;
        let top_context = self.get_top_context()?;

        context.set_line_width(width);
        top_context.set_line_width(width);

        Ok(())
    }

    pub fn set_stroke_color(&self, color: String) -> Result<(), JsValue> {
        let context = self.get_context()?;
        let top_context = self.get_top_context()?;

        context.set_stroke_style(&JsValue::from_str(&color));
        top_context.set_stroke_style(&JsValue::from_str(&color));

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

    fn get_top_context(&self) -> Result<CanvasRenderingContext2d, js_sys::Object> {
        self.top_layer
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
            self.top_layer
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
            self.top_layer
                .add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }
        {
            let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::MouseEvent| {
                pressed.set(false);
                context.line_to(event.offset_x() as f64, event.offset_y() as f64);
                context.stroke();
            });
            self.top_layer
                .add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }
        Ok(())
    }

    fn setup_straight_line(&self) -> Result<(), JsValue> {
        let top_context = Rc::new(self.get_top_context()?);
        let pressed = Rc::new(Cell::new(false));
        let line_start_x = Rc::new(Cell::new(0.0));
        let line_start_y = Rc::new(Cell::new(0.0));
        {
            let top_context = top_context.clone();
            let pressed = pressed.clone();
            let line_start_x = line_start_x.clone();
            let line_start_y = line_start_y.clone();

            let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::MouseEvent| {
                top_context.begin_path();
                line_start_x.set(event.offset_x() as f64);
                line_start_y.set(event.offset_y() as f64);
                top_context.move_to(event.offset_x() as f64, event.offset_y() as f64);
                pressed.set(true);
            });
            self.top_layer
                .add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }
        {
            let top_context = top_context.clone();
            let pressed = pressed.clone();
            let line_start_x = line_start_x.clone();
            let line_start_y = line_start_y.clone();

            let height = self.height;
            let width = self.width;

            let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::MouseEvent| {
                if pressed.get() {
                    top_context.clear_rect(0.0, 0.0, width as f64, height as f64);
                    top_context.line_to(event.offset_x() as f64, event.offset_y() as f64);
                    top_context.stroke();
                    top_context.begin_path();
                    top_context.move_to(line_start_x.get(), line_start_y.get());
                }
            });
            self.top_layer
                .add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }
        {
            let context = self.get_context()?;
            let line_start_x = line_start_x.clone();
            let line_start_y = line_start_y.clone();
            let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::MouseEvent| {
                pressed.set(false);
                context.begin_path();
                context.move_to(line_start_x.get(), line_start_y.get());
                context.line_to(event.offset_x() as f64, event.offset_y() as f64);
                context.stroke();
            });
            self.top_layer
                .add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }

        Ok(())
    }
    fn setup_circle(&self) -> Result<(), JsValue> {
        let top_context = Rc::new(self.get_top_context()?);
        let pressed = Rc::new(Cell::new(false));
        let line_start_x = Rc::new(Cell::new(0.0));
        let line_start_y = Rc::new(Cell::new(0.0));
        {
            let top_context = top_context.clone();
            let pressed = pressed.clone();
            let line_start_x = line_start_x.clone();
            let line_start_y = line_start_y.clone();

            let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::MouseEvent| {
                top_context.begin_path();
                line_start_x.set(event.offset_x() as f64);
                line_start_y.set(event.offset_y() as f64);
                pressed.set(true);
            });
            self.top_layer
                .add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }
        {
            let top_context = top_context.clone();
            let pressed = pressed.clone();

            let height = self.height;
            let width = self.width;
            let line_start_x = line_start_x.clone();
            let line_start_y = line_start_y.clone();

            let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::MouseEvent| {
                if pressed.get() {
                    top_context.clear_rect(0.0, 0.0, width as f64, height as f64);
                    let radius = two_point_distance(
                        line_start_x.get() as f64,
                        line_start_y.get() as f64,
                        event.offset_x() as f64,
                        event.offset_y() as f64,
                    );
                    let _ = top_context.arc(
                        event.offset_x() as f64,
                        event.offset_y() as f64,
                        radius,
                        0.0,
                        2.0 * PI,
                    );
                    top_context.stroke();
                    top_context.begin_path();
                }
            });
            self.top_layer
                .add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }
        {
            let context = self.get_context()?;
            let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::MouseEvent| {
                pressed.set(false);
                context.begin_path();
                let radius = two_point_distance(
                    line_start_x.get() as f64,
                    line_start_y.get() as f64,
                    event.offset_x() as f64,
                    event.offset_y() as f64,
                );
                let _ = context.arc(
                    event.offset_x() as f64,
                    event.offset_y() as f64,
                    radius,
                    0.0,
                    2.0 * PI,
                );
                context.stroke();
            });
            self.top_layer
                .add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }

        Ok(())
    }
}
