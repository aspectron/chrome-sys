use js_sys::Array;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {

    #[wasm_bindgen(catch, js_namespace = ["chrome", "storage", "local"], js_name="set")]
    pub async fn set(data: JsValue) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, js_namespace = ["chrome", "storage", "local"], js_name="get")]
    pub async fn get(key: String) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, js_namespace = ["chrome", "storage", "local"], js_name="get")]
    pub async fn get_items(keys: Array) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, js_namespace = ["chrome", "storage", "local"], js_name="get")]
    pub async fn get_all() -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch, js_namespace = ["chrome", "storage", "local"], js_name="remove")]
    pub async fn remove(key: String) -> Result<(), JsValue>;

    #[wasm_bindgen(catch, js_namespace = ["chrome", "storage", "local"], js_name="remove")]
    pub async fn remove_items(keys: Array) -> Result<(), JsValue>;

    #[wasm_bindgen(catch, js_namespace = ["chrome", "storage", "local"], js_name="clear")]
    pub async fn clear() -> Result<(), JsValue>;

}
