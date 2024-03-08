use js_sys::{Array, Object};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {

    // #[wasm_bindgen(extends = Array, typescript_type="RegisteredContentScript[]")]
    // #[derive(Debug)]
    // pub type RegisteredContentScripts;

    #[wasm_bindgen(extends = Object)]
    #[derive(Debug)]
    /// Dynamic content script
    ///
    /// ⧉ [Chrome Documentation](https://developer.chrome.com/docs/extensions/reference/api/scripting#type-RegisteredContentScript)
    ///
    pub type RegisteredContentScript;

    #[wasm_bindgen(method, getter, js_name = "allFrames")]
    /// If specified true, it will inject into all frames,
    /// even if the frame is not the top-most frame in the tab.
    /// Each frame is checked independently for URL requirements;
    /// it will not inject into child frames if the URL requirements are not met.
    /// Defaults to false, meaning that only the top frame is matched.
    pub fn all_frames(this: &RegisteredContentScript) -> Option<bool>;

    #[wasm_bindgen(method, setter, js_name = "allFrames")]
    pub fn set_all_frames(this: &RegisteredContentScript, all_frames: bool);

    #[wasm_bindgen(method, getter, js_name = "css")]
    pub fn css(this: &RegisteredContentScript) -> Option<Array>;

    #[wasm_bindgen(method, setter, js_name = "css")]
    pub fn set_css(this: &RegisteredContentScript, css: Array);

    #[wasm_bindgen(method, getter, js_name = "excludeMatches")]
    pub fn exclude_matches(this: &RegisteredContentScript) -> Option<Array>;

    #[wasm_bindgen(method, setter, js_name = "excludeMatches")]
    pub fn set_exclude_matches(this: &RegisteredContentScript, css: Array);

    #[wasm_bindgen(method, getter, js_name = "id")]
    pub fn id(this: &RegisteredContentScript) -> String;

    #[wasm_bindgen(method, setter, js_name = "id")]
    pub fn set_id(this: &RegisteredContentScript, id: String);

    #[wasm_bindgen(method, getter, js_name = "js")]
    pub fn js_impl(this: &RegisteredContentScript) -> Option<Array>;

    #[wasm_bindgen(method, setter, js_name = "js")]
    pub fn set_js_impl(this: &RegisteredContentScript, js: Array);

    #[wasm_bindgen(method, getter, js_name = "matchOriginAsFallback")]
    pub fn match_origin_as_fallback(this: &RegisteredContentScript) -> Option<String>;

    #[wasm_bindgen(method, setter, js_name = "matchOriginAsFallback")]
    pub fn set_match_origin_as_fallback(
        this: &RegisteredContentScript,
        match_origin_as_fallback: bool,
    );

    #[wasm_bindgen(method, getter, js_name = "matches")]
    pub fn matches_impl(this: &RegisteredContentScript) -> Option<Array>;

    #[wasm_bindgen(method, setter, js_name = "matches")]
    pub fn set_matches_impl(this: &RegisteredContentScript, args: Array);

    #[wasm_bindgen(method, getter, js_name = "persistAcrossSessions")]
    pub fn persist_across_sessions(this: &RegisteredContentScript) -> Option<bool>;

    #[wasm_bindgen(method, setter, js_name = "persistAcrossSessions")]
    pub fn set_persist_across_sessions(this: &RegisteredContentScript, world: bool);

    #[wasm_bindgen(method, getter, js_name = "runAt")]
    pub fn run_at(this: &RegisteredContentScript) -> Option<String>;

    #[wasm_bindgen(method, setter, js_name = "runAt")]
    pub fn set_run_at(this: &RegisteredContentScript, run_at: String);

    #[wasm_bindgen(method, getter, js_name = "world")]
    pub fn world(this: &RegisteredContentScript) -> String;

    #[wasm_bindgen(method, setter, js_name = "world")]
    pub fn set_world(this: &RegisteredContentScript, world: String);

    #[wasm_bindgen(extends = Object)]
    #[derive(Debug)]
    /// ScriptInjection
    ///
    /// ⧉ [Chrome Documentation](https://developer.chrome.com/docs/extensions/reference/api/scripting#type-ScriptInjection)
    ///
    pub type ScriptInjection;

    #[wasm_bindgen(method, getter, js_name = "args")]
    pub fn args_impl(this: &ScriptInjection) -> Option<Array>;

    #[wasm_bindgen(method, setter, js_name = "args")]
    pub fn set_args_impl(this: &ScriptInjection, args: Array);

    #[wasm_bindgen(method, getter, js_name = "files")]
    pub fn files_impl(this: &ScriptInjection) -> Option<Array>;

    #[wasm_bindgen(method, setter, js_name = "files")]
    pub fn set_files_impl(this: &ScriptInjection, args: Array);

    #[wasm_bindgen(method, getter, js_name = "injectImmediately")]
    pub fn inject_immediately(this: &ScriptInjection) -> Option<bool>;

    #[wasm_bindgen(method, setter, js_name = "injectImmediately")]
    pub fn set_inject_immediately(this: &ScriptInjection, inject_immediately: bool);

    #[wasm_bindgen(method, getter, js_name = "target")]
    pub fn target(this: &ScriptInjection) -> InjectionTarget;

    #[wasm_bindgen(method, setter, js_name = "target")]
    pub fn set_target(this: &ScriptInjection, target: InjectionTarget);

    #[wasm_bindgen(method, getter, js_name = "world")]
    pub fn world(this: &ScriptInjection) -> Option<String>;

    #[wasm_bindgen(method, setter, js_name = "world")]
    pub fn set_world(this: &ScriptInjection, world: String);

    #[wasm_bindgen(method, getter, js_name = "func")]
    pub fn func(this: &ScriptInjection) -> Option<bool>;

    #[wasm_bindgen(method, setter, js_name = "func")]
    pub fn set_func(this: &ScriptInjection, value: &dyn Fn());

    #[wasm_bindgen(method, setter, js_name = "func")]
    pub fn set_func_with_arg_u32_closure(this: &ScriptInjection, value: &Closure<dyn FnMut(u32)>);

    #[wasm_bindgen(method, setter, js_name = "func")]
    pub fn set_func_with_arg_u32(this: &ScriptInjection, value: &js_sys::Function);

    #[wasm_bindgen(method, setter, js_name = "func")]
    pub fn set_func_with_arg_str(this: &ScriptInjection, value: &mut dyn FnMut(String));

    #[wasm_bindgen(method, setter, js_name = "func")]
    pub fn set_func_with_arg_array(this: &ScriptInjection, value: &mut dyn FnMut(Array));

    #[wasm_bindgen(extends = Object)]
    #[derive(Debug)]
    /// InjectionTarget
    ///
    /// ⧉ [Chrome Documentation](https://developer.chrome.com/docs/extensions/reference/api/scripting#type-InjectionTarget)
    ///
    pub type InjectionTarget;

    #[wasm_bindgen(method, getter, js_name = "allFrames")]
    pub fn all_frames(this: &InjectionTarget) -> Option<bool>;

    #[wasm_bindgen(method, setter, js_name = "allFrames")]
    pub fn set_all_frames(this: &InjectionTarget, value: bool);

    #[wasm_bindgen(method, getter, js_name = "documentIds")]
    pub fn document_ids_impl(this: &InjectionTarget) -> Option<Array>;

    #[wasm_bindgen(method, setter, js_name = "documentIds")]
    pub fn set_document_ids_impl(this: &InjectionTarget, value: Array);

    #[wasm_bindgen(method, getter, js_name = "frameIds")]
    pub fn frame_ids_impl(this: &InjectionTarget) -> Option<Array>;

    #[wasm_bindgen(method, setter, js_name = "frameIds")]
    pub fn set_frame_ids_impl(this: &InjectionTarget, value: Array);

    #[wasm_bindgen(method, getter, js_name = "tabId")]
    pub fn tab_id(this: &InjectionTarget) -> Option<u32>;

    #[wasm_bindgen(method, setter, js_name = "tabId")]
    pub fn set_tab_id(this: &InjectionTarget, value: u32);

    #[wasm_bindgen(extends = Object)]
    #[derive(Debug)]
    /// ContentScriptFilter
    ///
    /// ⧉ [Chrome Documentation](https://developer.chrome.com/docs/extensions/reference/api/scripting#type-ContentScriptFilter)
    ///
    pub type ContentScriptFilter;

    #[wasm_bindgen(method, setter, js_name = "ids")]
    pub fn set_ids(this: &ContentScriptFilter, value: Array);

    #[wasm_bindgen(js_namespace = ["chrome", "scripting"], js_name="registerContentScripts")]
    pub async fn register_content_scripts_impl(scripts: Array);

    #[wasm_bindgen(js_namespace = ["chrome", "scripting"], js_name="unregisterContentScripts")]
    pub async fn unregister_content_scripts_impl(filter: JsValue);

    #[wasm_bindgen(js_namespace = ["chrome", "scripting"], js_name="unregisterContentScripts")]
    pub async fn unregister_content_scripts_all();

    #[wasm_bindgen(js_namespace = ["chrome", "scripting"], js_name="executeScript")]
    pub async fn execute_script(script: ScriptInjection) -> JsValue;

}

pub async fn register_content_scripts(scripts: Vec<RegisteredContentScript>) {
    let scripts = Array::from_iter(scripts.iter().map(JsValue::from).collect::<Vec<JsValue>>());
    register_content_scripts_impl(scripts).await
}

pub async fn unregister_content_scripts(filter: Option<Vec<&str>>) {
    if let Some(list) = filter {
        let filter = ContentScriptFilter::new();
        filter.set_ids(vector_to_array(list.into()));
        unregister_content_scripts_impl(filter.into()).await;
    } else {
        unregister_content_scripts_all().await;
    }
}

pub fn vector_to_array(values: Vec<&str>) -> Array {
    Array::from_iter(
        values
            .iter()
            .map(|a| JsValue::from(*a))
            .collect::<Vec<JsValue>>(),
    )
}
pub fn js_value_vector_to_array(values: Vec<JsValue>) -> Array {
    Array::from_iter(values.iter())
}

pub fn array_to_vector(values: Array) -> Vec<String> {
    values
        .into_iter()
        .map(|a| a.as_string().unwrap_or_default())
        .collect()
}

pub fn array_to_js_value_vector(values: Array) -> Vec<JsValue> {
    values.into_iter().collect()
}

pub trait ObjectHelper {
    fn new() -> Self
    where
        Self: wasm_bindgen::JsCast,
    {
        wasm_bindgen::JsCast::unchecked_into(Object::new())
    }
}

impl ObjectHelper for RegisteredContentScript {}
impl ObjectHelper for ScriptInjection {}
impl ObjectHelper for InjectionTarget {}
impl ObjectHelper for ContentScriptFilter {}

impl RegisteredContentScript {
    pub fn matches(&self) -> Option<Vec<String>> {
        self.matches_impl().map(array_to_vector)
    }

    pub fn set_matches(&self, values: Vec<&str>) {
        self.set_matches_impl(vector_to_array(values));
    }

    pub fn js(&self) -> Option<Vec<String>> {
        self.js_impl().map(array_to_vector)
    }

    pub fn set_js(&self, values: Vec<&str>) {
        self.set_js_impl(vector_to_array(values));
    }
}

impl ScriptInjection {
    pub fn args(&self) -> Option<Vec<JsValue>> {
        self.args_impl().map(array_to_js_value_vector)
    }

    pub fn set_args(&self, args: Vec<JsValue>) {
        self.set_args_impl(js_value_vector_to_array(args))
    }

    pub fn files(&self) -> Option<Vec<String>> {
        self.files_impl().map(array_to_vector)
    }

    pub fn set_files(&self, values: Vec<&str>) {
        self.set_files_impl(vector_to_array(values));
    }
}

impl InjectionTarget {
    pub fn document_ids(&self) -> Option<Vec<String>> {
        self.document_ids_impl().map(array_to_vector)
    }

    pub fn set_document_ids(&self, values: Vec<&str>) {
        self.set_document_ids_impl(vector_to_array(values));
    }

    pub fn frame_ids(&self) -> Option<Vec<String>> {
        self.frame_ids_impl().map(array_to_vector)
    }

    pub fn set_frame_ids(&self, values: Vec<&str>) {
        self.set_frame_ids_impl(vector_to_array(values));
    }
}

// impl From<Vec<RegisteredContentScript>> for RegisteredContentScripts{
//     fn from(values: Vec<RegisteredContentScript>) -> Self {
//         Array::from_iter(values.iter().map(JsValue::from).collect::<Vec<JsValue>>()).dyn_into().unwrap()
//     }
// }
