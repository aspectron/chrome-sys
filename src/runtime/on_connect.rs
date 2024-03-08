use crate::runtime::port::Port;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {

    #[wasm_bindgen(js_namespace = ["chrome", "runtime", "onConnect"], js_name="addListener")]
    pub fn add_on_connect_listener(closure: &Closure<dyn FnMut(Port) -> JsValue>);

}
