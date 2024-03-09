use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {

    #[wasm_bindgen(extends = Object)]
    #[derive(Debug)]
    pub type Sender;

    #[wasm_bindgen(method, getter)]
    pub fn id(this: &Sender) -> Option<String>;

    #[wasm_bindgen(method, getter)]
    pub fn tab(this: &Sender) -> TabInfo;

    #[wasm_bindgen(catch, js_namespace = ["chrome", "runtime"], js_name="sendMessage")]
    pub async fn send_message(s: &JsValue) -> Result<JsValue, JsValue>;


    #[wasm_bindgen(extends = Object)]
    #[derive(Debug)]
    pub type TabInfo;

    #[wasm_bindgen(method, getter)]
    pub fn id(this: &TabInfo) -> u32;

    #[wasm_bindgen(method, getter)]
    pub fn url(this: &TabInfo) -> String;
}
