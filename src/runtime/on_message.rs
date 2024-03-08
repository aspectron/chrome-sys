use crate::bindings::Sender;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {

    #[wasm_bindgen(js_namespace = ["chrome", "runtime", "onMessage"], js_name="addListener")]
    pub fn add_listener(closure: &Closure<dyn FnMut(JsValue, Sender, JsValue) -> JsValue>);

}
