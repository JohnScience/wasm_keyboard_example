use std::rc::Rc;

use wasm_bindgen::prelude::*;
use wasm_keyboard::{
    macros::{new_simplified_key_handler, start_keywise_keyboard_handler},
    uievents_code::{KeyboardEventCode, KEY_W},
};
use web_sys::KeyboardEvent;

// Called when the wasm module is instantiated
#[wasm_bindgen(start)]
fn main() -> Result<(), JsValue> {
    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window = web_sys::window().expect("no global `window` exists");
    let document = Rc::new(window.document().expect("should have a document on window"));
    let body = Rc::new(document.body().expect("document should have a body"));

    let w_handler = new_simplified_key_handler!(
        KeyboardEventCode::KeyW,
        state = (),
        keydown = {
            let body = body.clone();
            let document = document.clone();
            move |_state| {
                let val = document.create_element("p").unwrap();
                val.set_inner_html("W pressed down!");
                body.append_child(&val).unwrap();
            }
        },
        keyup = {
            let body = body.clone();
            let document = document.clone();
            move |_state| {
                let val = document.create_element("p").unwrap();
                val.set_inner_html("W released!");
                body.append_child(&val).unwrap();
            }
        }
    );

    start_keywise_keyboard_handler!(kh: Kh, document, [KEY_W => w_handler]);

    // Manufacture the element we're gonna append
    let val = document.create_element("p")?;
    val.set_inner_html("Hello from Rust!");

    body.append_child(&val)?;

    Ok(())
}

#[wasm_bindgen]
pub fn add(a: u32, b: u32) -> u32 {
    a + b
}
