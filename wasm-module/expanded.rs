#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_keyboard::{
    macros::{start_keyboard_handler, new_simplified_key_handler},
    uievents_code::{KeyboardEventCode, KEY_W},
};
use web_sys::KeyboardEvent;
#[allow(dead_code)]
fn main() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = Rc::new(window.document().expect("should have a document on window"));
    let body = Rc::new(document.body().expect("document should have a body"));
    let w_handler = ::wasm_keyboard::implementors::KeyHandler::<
        {
            ::core::convert::identity::<
                ::wasm_keyboard::uievents_code::KeyboardEventCode,
            >(KeyboardEventCode::KeyW) as u8
        },
        _,
        _,
        _,
    >::new(
        (::std::cell::Cell::new(false), ()),
        {
            let body = body.clone();
            let document = document.clone();
            move |(ref is_pressed, ref state)| {
                let handler = move |_state| {
                    let val = document.create_element("p").unwrap();
                    val.set_inner_html("W pressed down!");
                    body.append_child(&val).unwrap();
                };
                if !is_pressed.get() {
                    is_pressed.set(true);
                    handler(state);
                }
            }
        },
        {
            let body = body.clone();
            let document = document.clone();
            move |(ref is_pressed, ref state)| {
                let handler = move |_state| {
                    let val = document.create_element("p").unwrap();
                    val.set_inner_html("W released!");
                    body.append_child(&val).unwrap();
                };
                is_pressed.set(false);
                handler(state);
            }
        },
    );
    struct Kh<T0>
    where
        T0: ::wasm_keyboard::KeyHandler,
    {
        key_handler0: T0,
    }
    impl<T0> Kh<T0>
    where
        T0: ::wasm_keyboard::KeyHandler,
    {
        fn inner_handle_keydown(&self, event: &::web_sys::KeyboardEvent) {
            match event.code().as_str() {
                KEY_W => self.key_handler0.handle_keydown(),
                _ => {}
            }
        }
        fn inner_handle_keyup(&self, event: &::web_sys::KeyboardEvent) {
            match event.code().as_str() {
                KEY_W => self.key_handler0.handle_keyup(),
                _ => {}
            }
        }
    }
    let kh = ::std::rc::Rc::new(Kh { key_handler0: w_handler });
    {
        let kh = kh.clone();
        let __handler = ::wasm_bindgen::closure::Closure::<
            dyn ::core::ops::FnMut(_),
        >::new::<_>(move |event: KeyboardEvent| kh.inner_handle_keydown(&event));
        document
            .add_event_listener_with_callback(
                "keydown",
                ::wasm_bindgen::JsCast::unchecked_ref(__handler.as_ref()),
            )
            .unwrap();
        ::wasm_bindgen::closure::Closure::forget(__handler);
    }
    {
        let kh = kh.clone();
        let __handler = ::wasm_bindgen::closure::Closure::<
            dyn ::core::ops::FnMut(_),
        >::new::<_>(move |event: KeyboardEvent| kh.inner_handle_keyup(&event));
        document
            .add_event_listener_with_callback(
                "keyup",
                ::wasm_bindgen::JsCast::unchecked_ref(__handler.as_ref()),
            )
            .unwrap();
        ::wasm_bindgen::closure::Closure::forget(__handler);
    };
    let val = document.create_element("p")?;
    val.set_inner_html("Hello from Rust!");
    body.append_child(&val)?;
    Ok(())
}
#[automatically_derived]
const _: () = {
    pub unsafe extern "C" fn __wasm_bindgen_generated_main() -> <() as wasm_bindgen::convert::ReturnWasmAbi>::Abi {
        const _ASSERT: fn() = || -> <() as wasm_bindgen::convert::ReturnWasmAbi>::Abi {
            loop {}
        };
        let _ret = {
            let _ret = main();
            <Result<(), JsValue> as wasm_bindgen::__rt::Start>::start(_ret)
        };
        <() as wasm_bindgen::convert::ReturnWasmAbi>::return_abi(_ret)
    }
};
#[allow(dead_code)]
pub fn add(a: u32, b: u32) -> u32 {
    a + b
}
#[automatically_derived]
const _: () = {
    pub unsafe extern "C" fn __wasm_bindgen_generated_add(
        arg0: <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi,
        arg1: <u32 as wasm_bindgen::convert::FromWasmAbi>::Abi,
    ) -> <u32 as wasm_bindgen::convert::ReturnWasmAbi>::Abi {
        let _ret = {
            let arg0 = unsafe {
                <u32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(arg0)
            };
            let arg1 = unsafe {
                <u32 as wasm_bindgen::convert::FromWasmAbi>::from_abi(arg1)
            };
            let _ret = add(arg0, arg1);
            _ret
        };
        <u32 as wasm_bindgen::convert::ReturnWasmAbi>::return_abi(_ret)
    }
};
