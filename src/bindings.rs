use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {

    #[wasm_bindgen(extends = Object)]
    #[derive(Debug)]
    pub type Sender;

    #[wasm_bindgen(method, getter)]
    pub fn id(this: &Sender) -> Option<String>;

    #[wasm_bindgen(catch, js_namespace = ["chrome", "runtime"], js_name="sendMessage")]
    pub async fn send_message(s: &JsValue) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(js_namespace = ["chrome", "runtime", "onMessage"], js_name="addListener")]
    pub fn add_listener(closure: &Closure<dyn FnMut(JsValue, Sender, JsValue) -> JsValue>);

}
