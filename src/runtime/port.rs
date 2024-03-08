use crate::bindings::Sender;
use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {

    #[wasm_bindgen(extends = Object)]
    #[derive(Debug)]
    pub type Port;

    #[wasm_bindgen(method, getter)]
    pub fn sender(this: &Port) -> Sender;

    #[wasm_bindgen(method, getter, js_name = "onMessage")]
    pub fn on_message(this: &Port) -> PortOnMessage;

    #[wasm_bindgen(method, getter, js_name = "onDisconnect")]
    pub fn on_disconnect(this: &Port) -> PortOnDisconnect;

    #[wasm_bindgen(method, js_name = "postMessage")]
    pub fn post_message(this: &Port, value: JsValue);

    #[wasm_bindgen(extends = Object)]
    #[derive(Debug)]
    pub type PortOnMessage;

    #[wasm_bindgen(method, js_name = "addListener")]
    pub fn add_listener(this: &PortOnMessage, closure: &Closure<dyn FnMut(JsValue) -> JsValue>);

    #[wasm_bindgen(extends = Object)]
    #[derive(Debug)]
    pub type PortOnDisconnect;

    #[wasm_bindgen(method, js_name = "addListener")]
    pub fn add_listener(this: &PortOnDisconnect, closure: &Closure<dyn FnMut(JsValue) -> JsValue>);

}
