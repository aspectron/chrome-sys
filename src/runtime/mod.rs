pub mod on_connect;
pub mod on_message;
pub mod port;
pub mod scripting;

use js_sys::Object;
use port::Port;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Object)]
    #[derive(Debug)]
    /// chrome.runtime.connect()
    ///
    /// ⧉ [Chrome Documentation](https://developer.chrome.com/docs/extensions/reference/api/runtime#method-connect)
    ///
    pub type ConnectInfo;

    #[wasm_bindgen(method, setter, js_name = "name")]
    pub fn set_name(this: &ConnectInfo, value: String);

    /// chrome.runtime.connect()
    ///
    /// ⧉ [Chrome Documentation](https://developer.chrome.com/docs/extensions/reference/api/runtime#method-connect)
    ///
    #[wasm_bindgen(js_namespace = ["chrome", "runtime"], js_name="connect")]
    pub fn connect(info: ConnectInfo) -> Port;

}

impl ConnectInfo {
    pub fn new(name: &str) -> Self {
        let this: Self = wasm_bindgen::JsCast::unchecked_into(Object::new());

        this.set_name(name.into());
        this
    }
}
