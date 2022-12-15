use crate::alert;
use std::f64;
use std::rc::Rc;
use wasm_bindgen::{prelude::*, JsCast};

#[wasm_bindgen]
pub fn canvas_1() {
    let window = web_sys::window().expect("window should be there");
    let document = window.document().expect("Document should be there");
    // let body = document.body().expect("body should be there");

    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    context.begin_path();

    // Draw the outer circle.
    context
        .arc(75.0, 75.0, 50.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();

    // Draw the mouth.
    context.move_to(110.0, 75.0);
    context.arc(75.0, 75.0, 35.0, 0.0, f64::consts::PI).unwrap();

    // Draw the left eye.
    context.move_to(65.0, 65.0);
    context
        .arc(60.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();

    // Draw the right eye.
    context.move_to(95.0, 65.0);
    context
        .arc(90.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();

    context.stroke();

    context.set_fill_style(&"red".into());
    let mut x = 0.0;
    let mut y = 0.0;
    use web_sys::console;

    let closure = Closure::wrap(Box::new(move || {
        console::log_2(&JsValue::from(x), &JsValue::from(y));
        x += 1.0;
        y += 1.0;
        // let radom_x = context.fill_rect(x, y, w, h);
        context.fill_rect(x, y, 2.2, 2.2);
    }) as Box<dyn FnMut()>);
    window
        .set_interval_with_callback_and_timeout_and_arguments_0(
            closure.as_ref().unchecked_ref(),
            100,
        )
        .expect("Should set interval");

    closure.forget();
    drop(x);
    drop(y);
}
