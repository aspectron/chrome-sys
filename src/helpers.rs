use wasm_bindgen::prelude::*;

pub fn chrome_obj() -> std::result::Result<JsValue, JsValue> {
    js_sys::Reflect::get(&js_sys::global(), &"chrome".into())
}

pub fn runtime_obj() -> std::result::Result<JsValue, JsValue> {
    js_sys::Reflect::get(&chrome_obj()?, &"runtime".into())
}

pub fn runtime_id() -> std::result::Result<String, JsValue> {
    Ok(js_sys::Reflect::get(&runtime_obj()?, &"id".into())?
        .as_string()
        .unwrap())
}
