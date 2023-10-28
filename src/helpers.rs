use wasm_bindgen::prelude::*;

pub fn chrome() -> std::result::Result<JsValue, JsValue> {
    js_sys::Reflect::get(&js_sys::global(), &"chrome".into())
}

pub fn runtime() -> std::result::Result<JsValue, JsValue> {
    js_sys::Reflect::get(&chrome()?, &"runtime".into())
}

pub fn runtime_id() -> std::result::Result<String, JsValue> {
    Ok(js_sys::Reflect::get(&runtime()?, &"id".into())?
        .as_string()
        .unwrap())
}
